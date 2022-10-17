use glutin::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use rostregen_client::{
    input_tick, prepare_frame, render_frame, render_init, set_viewport, ClientState, Key,
};

use rostregen_game::GameState;

mod ui;

pub fn main() {
    let mut game_state = GameState::new();
    let (event_loop, windowed_context) = ui::init_glutin();
    let mut render_state = ClientState::new();
    render_init();

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => (),

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
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size) => {
                windowed_context.resize(physical_size);
                render_state.set_window_size(physical_size.width, physical_size.height);
                set_viewport(&render_state);
            }
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
                render_state
                    .input
                    .mouse_motion(delta.0 as f32, delta.1 as f32);
            }
            _ => (),
        },
        Event::MainEventsCleared => {
            input_tick(&mut game_state, &render_state);
            render_state.input.mouse_flush();

            game_state.tick();
            game_state.prepare_world();

            prepare_frame(&mut render_state, &game_state);
            render_frame(&render_state, &game_state);
            windowed_context.swap_buffers().unwrap();
        }
        _ => {}
    });
}
