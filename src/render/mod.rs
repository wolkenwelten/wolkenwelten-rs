use std::ffi::CString;

pub use self::mesh::Mesh;

pub use self::program::Program;
pub use self::shader::Shader;
pub use self::state::RenderState;
pub use self::state::viewport::Viewport;
pub use self::texture::Texture;
pub use self::texture_array::TextureArray;

mod mesh;
mod shader;
mod program;
mod texture;
mod texture_array;
mod state;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
	let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
	buffer.extend([b' '].iter().cycle().take(len));
	unsafe { CString::from_vec_unchecked(buffer) }
}
