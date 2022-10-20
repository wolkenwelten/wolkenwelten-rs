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
use wolkenwelten_game::{ChunkBlockData, GameState, Side};

#[derive(Debug, Default)]
pub struct BlockMesh {
    vao: Vao,
    last_updated_at: u64,
    side_square_count: [u32; 6],
    side_start: [u32; 6],
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
struct BlockVertex {
    xyz: u16,           // We've got 1 bit left here
    texture_index: u8, // Right now we don't really use 256 distinct block faces, ~32 should suffice for a long time
    side_and_light: u8, // And another one here as well
}

impl BlockVertex {
    fn new(x: u16, y: u16, z: u16, texture_index: u8, side: u8, light: u8) -> Self {
        let xyz: u16 = x | (y << 5) | (z << 10);
        let side_and_light = side | (light << 4);
        Self {
            xyz,
            texture_index,
            side_and_light,
        }
    }

    fn add_front(
        vertices: &mut Vec<Self>,
        (x, y, z): (u16, u16, u16),
        (w, h, d): (u16, u16, u16),
        texture_index: u8,
        light: u8,
    ) {
        let side: u8 = Side::Front.into();
        let z = z + d;
        vertices.push(Self::new(x, y, z, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z, texture_index, side, light));
        vertices.push(Self::new(x + w, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z, texture_index, side, light));
    }

    fn add_back(
        vertices: &mut Vec<Self>,
        (x, y, z): (u16, u16, u16),
        (w, h, _): (u16, u16, u16),
        texture_index: u8,
        light: u8,
    ) {
        let side: u8 = Side::Back.into();
        vertices.push(Self::new(x, y, z, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x + w, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z, texture_index, side, light));
    }

    fn add_top(
        vertices: &mut Vec<Self>,
        (x, y, z): (u16, u16, u16),
        (w, h, d): (u16, u16, u16),
        texture_index: u8,
        light: u8,
    ) {
        let side: u8 = Side::Top.into();
        let y = y + h;
        vertices.push(Self::new(x, y, z, texture_index, side, light));
        vertices.push(Self::new(x, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z, texture_index, side, light));
    }

    fn add_bottom(
        vertices: &mut Vec<Self>,
        (x, y, z): (u16, u16, u16),
        (w, _, d): (u16, u16, u16),
        texture_index: u8,
        light: u8,
    ) {
        let side: u8 = Side::Bottom.into();
        vertices.push(Self::new(x, y, z, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y, z + d, texture_index, side, light));
    }

    fn add_left(
        vertices: &mut Vec<Self>,
        (x, y, z): (u16, u16, u16),
        (_, h, d): (u16, u16, u16),
        texture_index: u8,
        light: u8,
    ) {
        let side: u8 = Side::Left.into();
        vertices.push(Self::new(x, y, z, texture_index, side, light));
        vertices.push(Self::new(x, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z, texture_index, side, light));
    }

    fn add_right(
        vertices: &mut Vec<Self>,
        (x, y, z): (u16, u16, u16),
        (w, h, d): (u16, u16, u16),
        texture_index: u8,
        light: u8,
    ) {
        let side: u8 = Side::Right.into();
        let x = x + w;
        vertices.push(Self::new(x, y, z, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y, z + d, texture_index, side, light));
    }

    fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();
        unsafe {
            let offset = util::vertex_attrib_int_pointer(stride, 0, 0, gl::UNSIGNED_SHORT, 2);
            let offset = util::vertex_attrib_int_pointer(stride, 1, offset, gl::UNSIGNED_BYTE, 1);
            util::vertex_attrib_int_pointer(stride, 2, offset, gl::UNSIGNED_BYTE, 1);
        }
    }
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
            self.vao
                .draw_elements(0, (self.side_start[5] + self.side_square_count[5]) * 6);
        } else {
            for i in 0..6 {
                if (mask & (1 << i)) != 0 {
                    let start_offset = self.side_start[i] * 6 * 2;
                    let index_count = self.side_square_count[i] * 6;
                    if index_count > 0 {
                        self.vao.draw_elements(start_offset, index_count);
                    }
                }
            }
        }
    }

    pub fn new(index_vbo: &Vbo) -> Self {
        let vao = Vao::new_empty("BlockMesh");
        BlockVertex::vertex_attrib_pointers();
        index_vbo.bind_element();
        Self {
            vao,
            side_square_count: [0; 6],
            side_start: [0; 6],
            last_updated_at: 0,
        }
    }

    pub fn last_updated_at(&self) -> u64 { self.last_updated_at }

    fn update_front(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk.data[x][y][z];
                    if block == 0 {
                        continue;
                    };
                    let pos = (x as u16, y as u16, z as u16);
                    let b = game.world.get_block_type(block);

                    if (z >= 15) || (chunk.data[x][y][z + 1] == 0) {
                        BlockVertex::add_front(vertices, pos, size, b.tex_front(), light);
                    }
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_back(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk.data[x][y][z];
                    if block == 0 {
                        continue;
                    };
                    let pos = (x as u16, y as u16, z as u16);
                    let b = game.world.get_block_type(block);

                    if (z == 0) || (chunk.data[x][y][z - 1] == 0) {
                        BlockVertex::add_back(vertices, pos, size, b.tex_back(), light);
                    }
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_top(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk.data[x][y][z];
                    if block == 0 {
                        continue;
                    };
                    let pos = (x as u16, y as u16, z as u16);
                    let b = game.world.get_block_type(block);

                    if (y >= 15) || (chunk.data[x][y + 1][z] == 0) {
                        BlockVertex::add_top(vertices, pos, size, b.tex_top(), light);
                    }
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_bottom(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk.data[x][y][z];
                    if block == 0 {
                        continue;
                    };
                    let pos = (x as u16, y as u16, z as u16);
                    let b = game.world.get_block_type(block);

                    if (y == 0) || (chunk.data[x][y - 1][z] == 0) {
                        BlockVertex::add_bottom(vertices, pos, size, b.tex_bottom(), light);
                    }
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_left(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk.data[x][y][z];
                    if block == 0 {
                        continue;
                    };
                    let pos = (x as u16, y as u16, z as u16);
                    let b = game.world.get_block_type(block);

                    if (x == 0) || (chunk.data[x - 1][y][z] == 0) {
                        BlockVertex::add_left(vertices, pos, size, b.tex_left(), light);
                    }
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_right(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk.data[x][y][z];
                    if block == 0 {
                        continue;
                    };
                    let pos = (x as u16, y as u16, z as u16);
                    let b = game.world.get_block_type(block);

                    if (x >= 15) || (chunk.data[x + 1][y][z] == 0) {
                        BlockVertex::add_right(vertices, pos, size, b.tex_right(), light);
                    }
                }
            }
        }
        (vertices.len() - start) / 4
    }

    pub fn update(&mut self, chunk: &ChunkBlockData, game: &GameState, now: u64) {
        self.last_updated_at = now;
        let mut vertices: Vec<BlockVertex> = Vec::with_capacity(65536);
        self.side_square_count[0] = Self::update_front(&mut vertices, chunk, game)
            .try_into()
            .unwrap();
        self.side_square_count[1] = Self::update_back(&mut vertices, chunk, game)
            .try_into()
            .unwrap();
        self.side_square_count[2] = Self::update_top(&mut vertices, chunk, game)
            .try_into()
            .unwrap();
        self.side_square_count[3] = Self::update_bottom(&mut vertices, chunk, game)
            .try_into()
            .unwrap();
        self.side_square_count[4] = Self::update_left(&mut vertices, chunk, game)
            .try_into()
            .unwrap();
        self.side_square_count[5] = Self::update_right(&mut vertices, chunk, game)
            .try_into()
            .unwrap();
        self.side_start[0] = 0;
        for i in 1..6 {
            self.side_start[i] = self.side_start[i - 1] + self.side_square_count[i - 1];
        }

        self.vao.bind();
        let vbo_size: u32 = (vertices.len() * std::mem::size_of::<BlockVertex>())
            .try_into()
            .unwrap();
        Vbo::buffer_data(vertices.as_ptr() as *const GLvoid, vbo_size);
        BlockVertex::vertex_attrib_pointers();
    }
}
