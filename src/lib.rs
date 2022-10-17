extern crate glutin;
extern crate rostregen_client;
extern crate rostregen_game;

use glutin::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::{CursorGrabMode, Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use rostregen_client::{input_tick, prepare_frame, render_frame, set_viewport, ClientState, Key, VIEW_STEPS};

use rostregen_game::GameState;

pub struct AppState {
    pub game_state: GameState,
    pub render_state: ClientState,
    pub event_loop: EventLoop<()>,
    pub windowed_context: ContextWrapper<PossiblyCurrent, Window>,
}

pub fn init_glutin() -> (EventLoop<()>, ContextWrapper<PossiblyCurrent, Window>) {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_decorations(false)
        .with_maximized(true)
        .with_title("RostRegen");

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .with_double_buffer(Some(true))
        .build_windowed(wb, &event_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    {
        let window = windowed_context.window();
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        window.set_cursor_visible(false);
    }

    (event_loop, windowed_context)
}

pub fn run_event_loop(state: AppState) {
    let mut render_state = state.render_state;
    let mut game_state = state.game_state;
    let event_loop = state.event_loop;
    let windowed_context = state.windowed_context;

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
        Event::WindowEvent {
            event: WindowEvent::Resized(physical_size),
            ..
        } => {
            windowed_context.resize(physical_size);
            render_state.set_window_size(physical_size.width, physical_size.height);
            set_viewport(&render_state);
        }
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta },
            ..
        } => {
            render_state
                .input
                .mouse_motion(delta.0 as f32, delta.1 as f32);
        }
        Event::MainEventsCleared => {
            input_tick(&mut game_state, &render_state);
            render_state.input.mouse_flush();

            game_state.tick();
            game_state.prepare_world(VIEW_STEPS);

            prepare_frame(&mut render_state, &game_state);
            render_frame(&render_state, &game_state);
            windowed_context.swap_buffers().unwrap();
        }
        _ => {}
    });
}
