use crate::render::Program;

pub struct ShaderList {
	pub block: Program,
	pub mesh: Program,
	pub text: Program,
}

impl ShaderList {
	pub fn new() -> Self {

		let mesh = Program::from_shader_sources(
			"Mesh Shader",
			include_str!("mesh.vert"),
			include_str!("mesh.frag"),
		).unwrap();

		let text = Program::from_shader_sources(
			"Text Shader",
			include_str!("text.vert"),
			include_str!("text.frag"),
		).unwrap();

		let block = Program::from_shader_sources(
			"Block Shader",
			include_str!("block.vert"),
			include_str!("block.frag"),
		).unwrap();

		Self {
			block,
			mesh,
			text,
		}
	}
}
