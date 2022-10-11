extern crate sdl2;
extern crate gl;

use crate::render::{Mesh, Vertex};

mod render;
mod input;

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let gl_attr = video_subsystem.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(3, 1);

	let window = video_subsystem
		.window("RostRegen", 800, 600)
		.opengl()
		.position_centered()
		.resizable()
		.build()
		.unwrap();

	let _gl_context = window.gl_create_context().unwrap();
	let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

	let mesh_shader = render::Program::from_shader_sources(
		include_str!("triangle.vert"),
		include_str!("triangle.frag")
	).unwrap();

	let vertices: Vec<Vertex> = vec![
		Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
		Vertex { pos: (0.5, -0.5, 0.0).into(),  clr: (0.0, 1.0, 0.0).into() },
		Vertex { pos: (0.0, 0.5, 0.0).into(),   clr: (0.0, 0.0, 1.0).into() }
	];
	let tri = Mesh::from_vec(&vertices).unwrap();
	render::refresh_gl();

	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut i:i32 = 0;
	'running: loop {
		if input::check_events(&mut event_pump) {
			break 'running;
		}

		i = i + 1;
		render::render_frame(i);
		mesh_shader.set_used();
		tri.draw();
		window.gl_swap_window();
	}
}
