/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
extern crate glam;
extern crate winit;
extern crate wolkenwelten_game;

use std::ffi::CString;

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
