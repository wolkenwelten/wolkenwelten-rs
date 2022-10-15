use std::ffi::CString;

pub use self::mesh::Mesh;

pub use self::shader::Program;
pub use self::state::RenderState;
pub use self::state::viewport::Viewport;
pub use self::texture::{Texture, TextureArray};
pub use self::input::{InputState, Key};

mod mesh;
mod shader;
mod texture;
mod state;
mod input;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
	let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
	buffer.extend([b' '].iter().cycle().take(len));
	unsafe { CString::from_vec_unchecked(buffer) }
}
