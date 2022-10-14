use std::ffi::CStr;
use crate::game::GameState;
use crate::render::RenderState;

use glutin::event::{Event, DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{CursorGrabMode, WindowBuilder};
use glutin::ContextBuilder;
use crate::input::Key;


mod game;
mod input;
mod render;

pub fn main() {
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
	let _gl = gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);


	{
		let window = windowed_context.window();
		window.set_cursor_grab(CursorGrabMode::Confined)
			.or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
			.unwrap();
		window.set_cursor_visible(false);
	}

	let version = unsafe {
		let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
		String::from_utf8(data).unwrap()
	};
	println!("OpenGL version {}", version);

	let mut game_state = GameState::new();
	let mut render_state = RenderState::new();

	event_loop.run(move |event, _, control_flow| {
		//println!("{event:?}");
		match event {
			Event::LoopDestroyed => (),

			Event::DeviceEvent { event: DeviceEvent::Key(input), .. } |
			Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => match input {
				KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. } => *control_flow = ControlFlow::Exit,

				KeyboardInput { state: ElementState::Pressed , virtual_keycode: Some(VirtualKeyCode::W), .. } =>
					render_state.input.key_down(Key::Up),
				KeyboardInput { state: ElementState::Released , virtual_keycode: Some(VirtualKeyCode::W), .. } =>
					render_state.input.key_up(Key::Up),

				KeyboardInput { state: ElementState::Pressed , virtual_keycode: Some(VirtualKeyCode::S), .. } =>
					render_state.input.key_down(Key::Down),
				KeyboardInput { state: ElementState::Released , virtual_keycode: Some(VirtualKeyCode::S), .. } =>
					render_state.input.key_up(Key::Down),

				KeyboardInput { state: ElementState::Pressed , virtual_keycode: Some(VirtualKeyCode::A), .. } =>
					render_state.input.key_down(Key::Left),
				KeyboardInput { state: ElementState::Released , virtual_keycode: Some(VirtualKeyCode::A), .. } =>
					render_state.input.key_up(Key::Left),

				KeyboardInput { state: ElementState::Pressed , virtual_keycode: Some(VirtualKeyCode::D), .. } =>
					render_state.input.key_down(Key::Right),
				KeyboardInput { state: ElementState::Released , virtual_keycode: Some(VirtualKeyCode::D), .. } =>
					render_state.input.key_up(Key::Right),

				KeyboardInput { state: ElementState::Pressed , virtual_keycode: Some(VirtualKeyCode::Space), .. } =>
					render_state.input.key_down(Key::Jump),
				KeyboardInput { state: ElementState::Released , virtual_keycode: Some(VirtualKeyCode::Space), .. } =>
					render_state.input.key_up(Key::Jump),

				KeyboardInput { state: ElementState::Pressed , virtual_keycode: Some(VirtualKeyCode::C), .. } =>
					render_state.input.key_down(Key::Crouch),
				KeyboardInput { state: ElementState::Released , virtual_keycode: Some(VirtualKeyCode::C), .. } =>
					render_state.input.key_up(Key::Crouch),

				KeyboardInput { state: ElementState::Pressed , virtual_keycode: Some(VirtualKeyCode::LShift), .. } =>
					render_state.input.key_down(Key::Sprint),
				KeyboardInput { state: ElementState::Released , virtual_keycode: Some(VirtualKeyCode::LShift), .. } =>
					render_state.input.key_up(Key::Sprint),

				_ => (),
			},
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::Resized(physical_size) => {
					windowed_context.resize(physical_size);
					render_state.viewport.update_size(physical_size.width, physical_size.height);
				},
				_ => (),
			},
			Event::DeviceEvent { event, .. } => match event {
				DeviceEvent::MouseMotion { delta } => {
					render_state.input.mouse_motion(delta.0 as f32, delta.1 as f32);
				}
				_ => ()
			},
			Event::MainEventsCleared => {
				game_state.check_input(&render_state);
				game_state.tick();

				render_state.input.mouse_flush();
				render_state.prepare(&mut game_state)
					.draw(&game_state);
				windowed_context.swap_buffers().unwrap();
			}
			_ => {
				//println!("{:?}", event);
			},
		}
	});
}
