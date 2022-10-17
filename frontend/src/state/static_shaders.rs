use crate::Program;

pub struct ShaderList {
    pub block: Program,
    pub mesh: Program,
    pub text: Program,
}

impl ShaderList {
    pub fn new() -> Self {
        let mesh = Program::from_shader_sources(
            "Mesh Shader",
            include_str!("../shaders/mesh.vert"),
            include_str!("../shaders/mesh.frag"),
        )
        .unwrap();

        let text = Program::from_shader_sources(
            "Text Shader",
            include_str!("../shaders/text.vert"),
            include_str!("../shaders/text.frag"),
        )
        .unwrap();

        let block = Program::from_shader_sources(
            "Block Shader",
            include_str!("../shaders/block.vert"),
            include_str!("../shaders/block.frag"),
        )
        .unwrap();

        Self { block, mesh, text }
    }
}
