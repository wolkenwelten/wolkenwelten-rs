// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
pub use super::input::InputState;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Window};

use std::num::NonZeroU32;
use glium;
use glutin::prelude::*;
use glutin::display::GetGlDisplay;
use glutin::surface::WindowSurface;
use raw_window_handle::HasRawWindowHandle;

use crate::{prepare_frame, render_frame, ClientState, RenderInit, RenderReactor, RENDER_DISTANCE};
use wolkenwelten_core::{ChunkRequestQueue, GameState, Message, Reactor};

/// Stores everything necessary to run a WolkenWelten instance
pub struct AppState {
    pub game_state: GameState,
    pub render_state: ClientState,
    pub input: InputState,
    pub event_loop: EventLoop<()>,
}

/// Try and grab the cursor, first by locking, then by confiningg it
/// This also makes the cursor invisible
fn grab_cursor(window: &Window) {
    window.set_cursor_visible(false);
    let e = window.set_cursor_grab(CursorGrabMode::Locked);
    if e.is_ok() {
        return;
    }
    let e = window.set_cursor_grab(CursorGrabMode::Confined);
    if let Err(e) = e {
        println!("Error when grabbing Cursor: {:?}", e);
    }
}

/// Let go of the cursor and restore cursor visibility
fn ungrab_cursor(window: &Window) {
    window.set_cursor_visible(true);
    let _ = window.set_cursor_grab(CursorGrabMode::None);
}

/// Create a new winit EventLoop and associated glium Display
fn init() -> (EventLoop<()>, glium::Display<WindowSurface>, winit::window::Window) {
    let title = format!("WolkenWelten - {}", env!("CARGO_PKG_VERSION"));
    let event_loop = EventLoop::new();

    let window_builder = winit::window::WindowBuilder::new().with_title(title);
    let config_template_builder = glutin::config::ConfigTemplateBuilder::new();
    let display_builder = glutin_winit::DisplayBuilder::new().with_window_builder(Some(window_builder));

    // First we create a window
    let (window, gl_config) = display_builder
        .build(&event_loop, config_template_builder, |mut configs| {
            // Just use the first configuration since we don't have any special preferences here
            configs.next().unwrap()
        })
        .unwrap();
    let window = window.unwrap();

    // Then the configuration which decides which OpenGL version we'll end up using, here we just use the default which is currently 3.3 core
    // When this fails we'll try and create an ES context, this is mainly used on mobile devices or various ARM SBC's
    // If you depend on features available in modern OpenGL Versions you need to request a specific, modern, version. Otherwise things will very likely fail.
    let raw_window_handle = window.raw_window_handle();
    let context_attributes = glutin::context::ContextAttributesBuilder::new().build(Some(raw_window_handle));
    let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
        .with_context_api(glutin::context::ContextApi::Gles(None))
        .build(Some(raw_window_handle));

    let not_current_gl_context = Some(unsafe {
        gl_config.display().create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
            gl_config.display()
                .create_context(&gl_config, &fallback_context_attributes)
                .expect("failed to create context")
        })
    });

    // Determine our framebuffer size based on the window size, or default to 800x600 if it's invisible
    let (width, height): (u32, u32) = window.inner_size().into();
    let attrs = glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    );
    // Now we can create our surface, use it to make our context current and finally create our display
    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };
    let current_context = not_current_gl_context.unwrap().make_current(&surface).unwrap();
    let display = glium::Display::from_context_surface(current_context, surface).unwrap();

    (event_loop, display, window)
}

/// Run the actual game, this function only returns when the game quits
fn run_event_loop(
    state: AppState,
    mut reactor: Reactor<Message>,
    render_init_fun: Vec<RenderInit>,
) {
    let mut render = state.render_state;
    let mut game = state.game_state;
    let mut input = state.input;
    let event_loop = state.event_loop;
    let mut request = ChunkRequestQueue::new();
    let mut render_reactor = RenderReactor::new();

    render_reactor.init(&mut reactor, &render, &game, render_init_fun);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => {}

            #[cfg(not(target_os = "macos"))]
            Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion { .. },
                ..
            } => {
                use winit::dpi::PhysicalPosition;

                let (x, y) = render.window_size();
                let center = PhysicalPosition::new(x / 2, y / 2);
                let _ = render
                    .window
                    .set_cursor_position(center);
            }

            Event::WindowEvent {
                event: WindowEvent::Focused(b),
                ..
            } => {
                if b {
                    grab_cursor(&render.window);
                } else {
                    ungrab_cursor(&render.window);
                }
            }

            Event::DeviceEvent {
                event: winit::event::DeviceEvent::Key(input),
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input {
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed,
                    virtual_keycode: Some(winit::event::VirtualKeyCode::F11),
                    ..
                } => render.set_show_debug_info(false),
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed,
                    virtual_keycode: Some(winit::event::VirtualKeyCode::F12),
                    ..
                } => render.set_show_debug_info(true),
                _ => (),
            },

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                render.display.context_surface_pair().resize(physical_size.into());
                render.set_window_size((physical_size.width, physical_size.height));
            }

            Event::RedrawRequested(_) => {
                let player_pos = game.player().pos;
                reactor.dispatch(Message::DrawFrame {
                    player_pos,
                    ticks: render.ticks(),
                    render_distance: RENDER_DISTANCE,
                });

                let mut frame = render.display.draw();
                prepare_frame(&mut render, &game, &mut request)
                    .expect("Error during frame preparation");
                render_reactor.run(
                    &mut frame,
                    &render,
                    &game,
                    &reactor,
                    &mut request,
                    RENDER_DISTANCE,
                );
                render_frame(&mut frame, &render, &game).expect("Error during rendering");
                frame.finish().expect("Error during frame finish");

                reactor.dispatch(Message::FinishedFrame {
                    player_pos,
                    ticks: render.ticks(),
                    render_distance: RENDER_DISTANCE,
                });
            }

            Event::MainEventsCleared => {
                if input.tick(&game, &reactor) {
                    render.clear();
                }
                game.tick(&reactor, &mut request);
                game.world_mut().handle_requests(&mut request, &reactor);
                render.request_redraw();
                if !game.running() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        };
        input.handle_winit_event(&reactor, event);
    });
}

pub fn start_client(
    game_state: GameState,
    reactor: Reactor<Message>,
    render_init_fun: Vec<RenderInit>,
) {
    let (event_loop, display, window) = init();
    let render_state = ClientState::new(display, window).expect("Can't create ClientState");

    run_event_loop(
        AppState {
            game_state,
            render_state,
            event_loop,
            input: InputState::new(),
        },
        reactor,
        render_init_fun,
    )
}
