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
use super::util;
use super::{Vao, Vbo};
use gl::types::GLvoid;
use wolkenwelten_game::{ChunkBlockData, ChunkLightData, GameState};
use wolkenwelten_meshgen;
use wolkenwelten_meshgen::BlockVertex;

#[derive(Debug, Default)]
pub struct BlockMesh {
    vao: Vao,
    last_updated_at: u64,
    side_square_count: [usize; 6],
    side_start: [usize; 6],
}

impl BlockMesh {
    pub fn gen_index_buffer(square_count: usize) -> Vbo {
        let mut v: Vec<u16> = Vec::with_capacity(square_count * 6);
        for i in 0..square_count {
            v.push((i * 4) as u16);
            v.push((i * 4 + 1) as u16);
            v.push((i * 4 + 2) as u16);

            v.push((i * 4 + 2) as u16);
            v.push((i * 4 + 3) as u16);
            v.push((i * 4) as u16);
        }
        let vbo_size: u32 = square_count as u32 * 6 * 2;
        Vbo::new(
            "BlockMesh Index Buffer",
            v.as_ptr() as *const GLvoid,
            vbo_size,
        )
    }

    pub fn calc_mask(x_offset: i32, y_offset: i32, z_offset: i32) -> u8 {
        (if z_offset <= 0 { 1 << 0 } else { 0 }
            | if z_offset >= 0 { 1 << 1 } else { 0 }
            | if y_offset <= 0 { 1 << 2 } else { 0 }
            | if y_offset >= 0 { 1 << 3 } else { 0 }
            | if x_offset >= 0 { 1 << 4 } else { 0 }
            | if x_offset <= 0 { 1 << 5 } else { 0 })
    }

    pub fn draw(&self, mask: u8) {
        if mask == 0b111111 {
            self.vao.draw_elements(
                0,
                ((self.side_start[5] + self.side_square_count[5]) * 6) as u32,
            );
        } else {
            (0..6).filter(|i| (mask & (1 << i)) != 0).for_each(|i| {
                let start_offset = self.side_start[i] * 6 * 2;
                let index_count = self.side_square_count[i] * 6;
                if index_count > 0 {
                    self.vao
                        .draw_elements(start_offset as u32, index_count as u32);
                }
            });
        }
    }

    pub fn new(index_vbo: &Vbo) -> Self {
        let vao = Vao::new_empty("BlockMesh");
        Self::vertex_attrib_pointers();
        index_vbo.bind_element();
        Self {
            vao,
            side_square_count: [0; 6],
            side_start: [0; 6],
            last_updated_at: 0,
        }
    }

    pub fn last_updated_at(&self) -> u64 {
        self.last_updated_at
    }

    pub fn update(
        &mut self,
        chunk: &ChunkBlockData,
        light: &ChunkLightData,
        game: &GameState,
        now: u64,
    ) {
        self.last_updated_at = now;

        let (vertices, side_start_count) = wolkenwelten_meshgen::generate(chunk, light, game);
        self.side_square_count = side_start_count;
        self.side_start[0] = 0;
        for i in 1..6 {
            self.side_start[i] = self.side_start[i - 1] + self.side_square_count[i - 1];
        }

        self.vao.bind();
        let vbo_size: usize = vertices.len() * std::mem::size_of::<BlockVertex>();
        Vbo::buffer_data(vertices.as_ptr() as *const GLvoid, vbo_size as u32);
        Self::vertex_attrib_pointers();
    }

    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<BlockVertex>();
        unsafe {
            let offset = util::vertex_attrib_int_pointer(stride, 0, 0, gl::UNSIGNED_BYTE, 3, 3);
            let offset =
                util::vertex_attrib_int_pointer(stride, 1, offset, gl::UNSIGNED_BYTE, 1, 1);
            util::vertex_attrib_int_pointer(stride, 2, offset, gl::UNSIGNED_BYTE, 1, 1);
        }
    }
}
