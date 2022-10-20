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
    element_count: u32,
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
    pub fn gen_index_buffer(square_count:usize) -> Vbo {
        let mut v:Vec<u16> = Vec::with_capacity(square_count*6);
        for i in 0..square_count {
            v.push((i*4) as u16);
            v.push((i*4+1) as u16);
            v.push((i*4+2) as u16);

            v.push((i*4+2) as u16);
            v.push((i*4+3) as u16);
            v.push((i*4) as u16);
        }
        let vbo_size:u32 = square_count as u32 * 6 * 2;
        Vbo::new("BlockMesh Index Buffer", v.as_ptr() as *const GLvoid, vbo_size)
    }

    pub fn draw(&self) {
        self.vao.draw_elements(self.element_count);
    }

    pub fn new(index_vbo:&Vbo) -> Self {
        let vao = Vao::new_empty("BlockMesh");
        BlockVertex::vertex_attrib_pointers();
        index_vbo.bind_element();
        Self { vao, element_count: 0 }

    }

    pub fn update(&mut self, chunk: &ChunkBlockData, game: &GameState) {
        let mut vertices: Vec<BlockVertex> = Vec::with_capacity(65536);
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
                        BlockVertex::add_front(&mut vertices, pos, size, b.tex_front(), light);
                    }
                    if (z == 0) || (chunk.data[x][y][z - 1] == 0) {
                        BlockVertex::add_back(&mut vertices, pos, size, b.tex_back(), light);
                    }
                    if (y >= 15) || (chunk.data[x][y + 1][z] == 0) {
                        BlockVertex::add_top(&mut vertices, pos, size, b.tex_top(), light);
                    }
                    if (y == 0) || (chunk.data[x][y - 1][z] == 0) {
                        BlockVertex::add_bottom(&mut vertices, pos, size, b.tex_bottom(), light);
                    }
                    if (x == 0) || (chunk.data[x - 1][y][z] == 0) {
                        BlockVertex::add_left(&mut vertices, pos, size, b.tex_left(), light);
                    }
                    if (x >= 15) || (chunk.data[x + 1][y][z] == 0) {
                        BlockVertex::add_right(&mut vertices, pos, size, b.tex_right(), light);
                    }
                }
            }
        }
        self.vao.bind();
        let vbo_size: u32 = (vertices.len() * std::mem::size_of::<BlockVertex>())
            .try_into()
            .unwrap();
        Vbo::buffer_data(vertices.as_ptr() as *const GLvoid, vbo_size);
        BlockVertex::vertex_attrib_pointers();
        self.element_count = (vertices.len() as u32 / 4)*6;
    }
}
