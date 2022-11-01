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
use glium::implement_vertex;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct BlockVertex {
    pos: [u8; 3],
    texture_index: u8, // Right now we don't really use 256 distinct block faces, ~32 should suffice for a long time
    side_and_light: u8, // And another one here as well
}
implement_vertex!(BlockVertex, pos, texture_index, side_and_light);

impl BlockVertex {
    pub fn new(x: u8, y: u8, z: u8, texture_index: u8, side: u8, light: u8) -> Self {
        let side_and_light = side | (light << 4);
        Self {
            pos: [x, y, z],
            texture_index,
            side_and_light,
        }
    }
}
