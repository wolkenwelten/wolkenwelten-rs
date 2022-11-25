// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
extern crate wolkenwelten_client;
extern crate wolkenwelten_game;

mod input;
pub use input::InputState;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Fullscreen, Window, WindowBuilder};

use wolkenwelten_client::{prepare_frame, render_frame, ClientState, RENDER_DISTANCE};
use wolkenwelten_common::{ChunkRequestQueue, GameEvent, Message, SyncEvent};
use wolkenwelten_game::GameState;

pub type MessageSink = Box<dyn Fn(&Vec<Message>)>;

/// Stores everything necessary to run a WolkenWelten instance
pub struct AppState {
    pub game_state: GameState,
    pub render_state: ClientState,
    pub input: InputState,
    pub event_loop: EventLoop<()>,
    pub message_sinks: Vec<MessageSink>,
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
pub fn init() -> (EventLoop<()>, glium::Display) {
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
pub fn run_event_loop(state: AppState) {
    let mut render = state.render_state;
    let mut game = state.game_state;
    let mut input = state.input;
    let event_loop = state.event_loop;

    let mut msgs: Vec<Message> = vec![];
    game.set_render_distance(RENDER_DISTANCE * RENDER_DISTANCE);

    let mut request = ChunkRequestQueue::new();

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
            }

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
                msgs.push(
                    SyncEvent::DrawFrame(game.player().pos, render.ticks(), RENDER_DISTANCE).into(),
                );
                let mut frame = render.display.draw();
                prepare_frame(&mut render, &game, &mut request)
                    .expect("Error during frame preparation");
                render_frame(&mut frame, &render, &game, &mut request)
                    .expect("Error during rendering");
                frame.finish().expect("Error during frame finish");
            }

            Event::MainEventsCleared => {
                msgs.extend(input.tick(&game));
                msgs.extend(game.tick(&msgs, &mut request));
                msgs.iter().for_each(|e| match e {
                    Message::SyncEvent(SyncEvent::GameQuit) => *control_flow = ControlFlow::Exit,
                    Message::GameEvent(m) => match m {
                        GameEvent::CharacterDeath(_) => {
                            game.player_rebirth();
                        }
                        GameEvent::EntityCollision(pos) => {
                            game.world.add_explosion(pos, 7.0);
                        }
                        _ => (),
                    },
                    _ => (),
                });
                state.message_sinks.iter().for_each(|λ| λ(&msgs));

                msgs.clear();
                game.world.handle_requests(&mut request);
                render.request_redraw();
            }
            _ => {}
        };
        input.handle_winit_event(event);
    });
}

pub fn start_app(game_state: GameState, sinks: Vec<MessageSink>) {
    let (event_loop, display) = init();
    let render_state = ClientState::new(display, &game_state).expect("Can't create ClientState");
    let mut message_sinks: Vec<MessageSink> = Vec::new();
    message_sinks.extend(sinks);
    {
        let particles = render_state.particles().clone();
        let block_types = game_state.world.blocks.clone();
        let λ = move |msgs: &Vec<Message>| {
            let mut particles = particles.borrow_mut();
            let block_types = block_types.borrow();
            particles.msg_sink(msgs, &block_types);
        };
        message_sinks.push(Box::new(λ));
    }

    run_event_loop(AppState {
        game_state,
        render_state,
        event_loop,
        input: InputState::new(),
        message_sinks,
    })
}
