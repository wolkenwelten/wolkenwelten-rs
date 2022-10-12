use crate::render::Program;

pub struct ShaderList {
    pub colored_mesh:Program,
    pub mesh:Program,
}

impl ShaderList {
    pub fn new() -> ShaderList {

        let colored_mesh = Program::from_shader_sources(
            include_str!("colored_mesh.vert"),
            include_str!("colored_mesh.frag")
        ).unwrap();

        let mesh = Program::from_shader_sources(
            include_str!("mesh.vert"),
            include_str!("mesh.frag")
        ).unwrap();

        ShaderList {
            colored_mesh,
            mesh,
        }
    }
}