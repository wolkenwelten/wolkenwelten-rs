use crate::render::Program;

pub struct ShaderList {
    pub mesh:Program,
}

impl ShaderList {
    pub fn new() -> ShaderList {

        let mesh = Program::from_shader_sources(
            include_str!("triangle.vert"),
            include_str!("triangle.frag")
        ).unwrap();

        ShaderList {
            mesh,
        }
    }
}