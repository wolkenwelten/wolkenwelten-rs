/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
extern crate glutin;
extern crate wolkenwelten_client;
extern crate wolkenwelten_game;

use glutin::dpi::PhysicalPosition;
use glutin::event::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::{CursorGrabMode, Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use wolkenwelten_client::{
    input_tick, prepare_frame, render_frame, set_viewport, ClientState, Key, RENDER_DISTANCE,
    VIEW_STEPS,
};

use wolkenwelten_game::GameState;

pub struct AppState {
    pub game_state: GameState,
    pub render_state: ClientState,
    pub event_loop: EventLoop<()>,
    pub windowed_context: ContextWrapper<PossiblyCurrent, Window>,
}

pub fn init_glutin() -> (EventLoop<()>, ContextWrapper<PossiblyCurrent, Window>) {
    let title = format!("WolkenWelten - {}", env!("CARGO_PKG_VERSION"));
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_decorations(false)
        .with_maximized(true)
        .with_title(title);

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        //.with_double_buffer(Some(true))
        .build_windowed(wb, &event_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    let window = windowed_context.window();
    let _ = window
        .set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked));
    window.set_cursor_visible(false);

    (event_loop, windowed_context)
}

pub fn run_event_loop(state: AppState) {
    let mut render_state = state.render_state;
    let mut game_state = state.game_state;
    let event_loop = state.event_loop;
    let windowed_context = state.windowed_context;

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => (),

        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            let (window_width, window_height) = render_state.window_size();
            let diffx = position.x as i32 - (window_width / 2) as i32;
            let diffy = position.y as i32 - (window_height / 2) as i32;
            game_state.player.rot.x += diffx as f32 * 0.025;
            game_state.player.rot.y += diffy as f32 * 0.025;
            let new_pos = PhysicalPosition::new(window_width / 2, window_height / 2);
            let _ = windowed_context.window().set_cursor_position(new_pos);
        }

        Event::WindowEvent {
            event:
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                },
            ..
        } => {
            render_state
                .input
                .set_left_mouse_button(state == ElementState::Pressed);
        }

        Event::WindowEvent {
            event: WindowEvent::Focused(b),
            ..
        } => {
            let window = windowed_context.window();
            window.set_cursor_visible(!b);
            if b {
                let _ = window
                    .set_cursor_grab(CursorGrabMode::Confined)
                    .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked));
            } else {
                let _ = window.set_cursor_grab(CursorGrabMode::None);
            }
        }

        Event::DeviceEvent {
            event: DeviceEvent::Key(input),
            ..
        }
        | Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => match input {
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => *control_flow = ControlFlow::Exit,

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::N),
                ..
            } => game_state.player.set_no_clip(true),
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::M),
                ..
            } => game_state.player.set_no_clip(false),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::O),
                ..
            } => render_state.set_wireframe(true),
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::P),
                ..
            } => render_state.set_wireframe(false),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::W),
                ..
            } => render_state.input.key_down(Key::Up),
            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::W),
                ..
            } => render_state.input.key_up(Key::Up),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::S),
                ..
            } => render_state.input.key_down(Key::Down),
            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::S),
                ..
            } => render_state.input.key_up(Key::Down),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::A),
                ..
            } => render_state.input.key_down(Key::Left),
            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::A),
                ..
            } => render_state.input.key_up(Key::Left),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::D),
                ..
            } => render_state.input.key_down(Key::Right),
            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::D),
                ..
            } => render_state.input.key_up(Key::Right),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
            } => render_state.input.key_down(Key::Jump),
            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
            } => render_state.input.key_up(Key::Jump),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::C),
                ..
            } => render_state.input.key_down(Key::Crouch),
            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::C),
                ..
            } => render_state.input.key_up(Key::Crouch),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::LShift),
                ..
            } => render_state.input.key_down(Key::Sprint),
            KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::LShift),
                ..
            } => render_state.input.key_up(Key::Sprint),

            _ => (),
        },
        Event::WindowEvent {
            event: WindowEvent::Resized(physical_size),
            ..
        } => {
            windowed_context.resize(physical_size);
            render_state.set_window_size((physical_size.width, physical_size.height));
            set_viewport(&render_state);
        }
        Event::MainEventsCleared => {
            input_tick(&mut game_state, &render_state);
            render_state.input.mouse_flush();

            let render_distance = RENDER_DISTANCE * RENDER_DISTANCE;
            game_state.tick(render_distance);
            game_state.prepare_world(VIEW_STEPS, render_distance);

            prepare_frame(&mut render_state, &game_state);
            render_frame(&render_state, &game_state);
            windowed_context.swap_buffers().unwrap();
        }
        _ => {}
    });
}
