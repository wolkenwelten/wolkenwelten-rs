extern crate glam;
extern crate wolkenwelten_game;

use std::ffi::CString;

pub static mut GL_VERSION: (i32, i32) = (0, 0);

pub fn can_use_object_labels() -> bool {
    unsafe { (GL_VERSION.0 > 4) || ((GL_VERSION.0 == 4) && (GL_VERSION.1 >= 3)) }
}

pub use self::frustum::Frustum;
pub use self::input::{input_tick, InputState, Key};
pub use self::meshes::Mesh;
pub use self::render::{prepare_frame, render_frame, render_init, set_viewport, VIEW_STEPS};
pub use self::shader::Program;
pub use self::state::ClientState;
pub use self::texture::{Texture, TextureArray};

mod frustum;
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
