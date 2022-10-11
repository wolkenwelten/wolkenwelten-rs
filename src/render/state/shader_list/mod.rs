use crate::render::Program;

pub struct ShaderList {
    pub mesh:Program,
}

impl ShaderList {
    pub fn new() -> ShaderList {

        let mesh = Program::from_shader_sources(
            include_str!("mesh.vert"),
            include_str!("mesh.frag")
        ).unwrap();

        ShaderList {
            mesh,
        }
    }
}