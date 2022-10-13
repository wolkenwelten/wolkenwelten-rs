use crate::render::Program;

pub struct ShaderList {
	pub mesh: Program,
	pub text_shader: Program,
}

impl ShaderList {
	pub fn new() -> Self {

		let mesh = Program::from_shader_sources(
			include_str!("mesh.vert"),
			include_str!("mesh.frag"),
		).unwrap();

		let text_shader = Program::from_shader_sources(
			include_str!("text_shader.vert"),
			include_str!("text_shader.frag"),
		).unwrap();

		Self {
			mesh,
			text_shader
		}
	}
}
