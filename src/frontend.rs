use std::ffi::CString;

pub use self::meshes::Mesh;

pub use self::input::{input_tick, InputState, Key};
pub use self::render::{prepare_frame, render_frame, render_init, set_viewport};
pub use self::shader::Program;
pub use self::state::FrontendState;
pub use self::texture::{Texture, TextureArray};

mod input;
mod meshes;
mod render;
mod shader;
mod state;
mod texture;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
