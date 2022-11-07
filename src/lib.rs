// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
extern crate wolkenwelten_client;
extern crate wolkenwelten_game;
extern crate wolkenwelten_scripting;

use winit::event::{DeviceEvent, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Fullscreen, Window, WindowBuilder};

use winit::dpi::PhysicalPosition;
use wolkenwelten_client::{prepare_frame, render_frame, ClientState, RENDER_DISTANCE};
use wolkenwelten_common::{GameEvent, Message, ParticleEvent, SyncEvent};
use wolkenwelten_game::GameState;
use wolkenwelten_input_winit::InputState;
use wolkenwelten_scripting::Runtime;

#[cfg(feature = "sound")]
use wolkenwelten_sound::SfxList;

/// Stores everything necessary to run a WolkenWelten instance
pub struct AppState {
    pub game_state: GameState,
    pub render_state: ClientState,
    pub input: InputState,
    pub event_loop: EventLoop<()>,
    pub runtime: Runtime,

    #[cfg(feature = "sound")]
    pub sfx: SfxList,
}

/// Try and grab the cursor, first by locking, then by confiningg it
/// This also makes the cursor invisible
pub fn grab_cursor(window: &Window) {
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
pub fn ungrab_cursor(window: &Window) {
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

    let cb = glium::glutin::ContextBuilder::new().with_vsync(true);

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
    let mut runtime = state.runtime;

    #[cfg(feature = "sound")]
    let sfx = state.sfx;

    let mut msgs: Vec<Message> = vec![];
    game.set_render_distance(RENDER_DISTANCE * RENDER_DISTANCE);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => {}

            #[cfg(not(target_os = "macos"))]
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { .. },
                ..
            } => {
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
                runtime.tick(game.get_millis());
                let mut frame = render.display.draw();
                prepare_frame(&mut render, &game).expect("Error during frame preparation");
                render_frame(&mut frame, &render, &game).expect("Error during rendering");
                frame.finish().expect("Error during frame finish");
            }

            Event::MainEventsCleared => {
                msgs.push(SyncEvent::DrawFrame(render.ticks()).into());
                msgs.extend(input.tick(&game));

                msgs.extend(game.tick(&msgs));

                let mut emissions: Vec<Message> = vec![];
                msgs.iter().for_each(|e| {
                    if let Message::GameEvent(m) = e {
                        match m {
                            GameEvent::CharacterDeath(_) => {
                                game.player_rebirth();
                            }
                            GameEvent::GameQuit => *control_flow = ControlFlow::Exit,
                            GameEvent::BlockMine(pos, b) => {
                                let color = game.world.get_block_type(*b).colors();
                                emissions.push(ParticleEvent::BlockBreak(*pos, color).into());
                            }
                            GameEvent::BlockPlace(pos, b) => {
                                let color = game.world.get_block_type(*b).colors();
                                emissions.push(ParticleEvent::BlockPlace(*pos, color).into());
                            }
                            GameEvent::EntityCollision(pos) => {
                                game.world.add_explosion(pos, 5.0);
                                emissions.push(ParticleEvent::Explosion(*pos, 4.0).into());
                            }
                            _ => (),
                        }
                    }
                });
                msgs.extend(emissions);

                #[cfg(feature = "sound")]
                sfx.msg_sink(&msgs);

                render.particles.msg_sink(&msgs);
                msgs.clear();

                render.request_redraw();
            }
            _ => {}
        };
        input.handle_winit_event(event);
    });
}
