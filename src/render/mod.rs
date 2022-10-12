use std::ffi::CString;

mod vertex;
mod colored_mesh;
mod mesh;
mod shader;
mod program;

mod state;

pub use self::vertex::{f32_f32_f32, Colored_Mesh_Vertex};
pub use self::shader::Shader;
pub use self::program::Program;
pub use self::mesh::Mesh;
pub use self::colored_mesh::ColoredMesh;

pub use self::state::viewport::Viewport;
pub use self::state::RenderState;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
