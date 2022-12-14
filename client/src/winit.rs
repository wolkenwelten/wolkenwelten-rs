// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
pub use super::input::InputState;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Fullscreen, Window, WindowBuilder};

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
fn init() -> (EventLoop<()>, glium::Display) {
    let title = format!("WolkenWelten - {}", env!("CARGO_PKG_VERSION"));
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(title)
        .with_decorations(false)
        .with_maximized(true);

    let cb = glium::glutin::ContextBuilder::new();
    // Disable vsync on ARM devices like the RPI4 where it seems to have a detrimental effect on the FPS
    let cb = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
        cb
    } else {
        cb.with_vsync(true)
    };

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    {
        let ctx = display.gl_window();
        let window = ctx.window();
        window.focus_window();
        let fs = Fullscreen::Borderless(window.current_monitor());
        window.set_fullscreen(Some(fs));
        grab_cursor(window);
    }

    (event_loop, display)
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
                    .display
                    .gl_window()
                    .window()
                    .set_cursor_position(center);
            }

            Event::WindowEvent {
                event: WindowEvent::Focused(b),
                ..
            } => {
                if b {
                    grab_cursor(render.display.gl_window().window());
                } else {
                    ungrab_cursor(render.display.gl_window().window());
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
                render.display.gl_window().resize(physical_size);
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
                render_reactor.run(&mut frame, &render, &game, &mut request, RENDER_DISTANCE);
                render_frame(&mut frame, &render, &game).expect("Error during rendering");
                frame.finish().expect("Error during frame finish");

                reactor.dispatch(Message::FinishedFrame {
                    player_pos,
                    ticks: render.ticks(),
                    render_distance: RENDER_DISTANCE,
                });
            }

            Event::MainEventsCleared => {
                input.tick(&game, &reactor);
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
    let (event_loop, display) = init();
    let render_state = ClientState::new(display).expect("Can't create ClientState");

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
