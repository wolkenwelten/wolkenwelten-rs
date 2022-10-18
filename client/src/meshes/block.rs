use super::vertex::vertex_attrib_int_pointer;
use super::{Vao, Vbo};
use gl::types::GLvoid;
use std::fmt;
use wolkenwelten_game::{ChunkBlockData, GameState, Side};

#[derive(Debug, Default)]
pub struct BlockMesh {
    vao: Vao,
    vertex_count: u16,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
struct BlockVertex {
    xyz: u16,           // We've got 1 bit left here
    texture_index: u8, // Right now we don't really use 256 distinct block faces, ~32 should suffice for a long time
    side_and_light: u8, // And another one here as well
}

impl fmt::Display for BlockVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.xyz & 0x1F;
        let y = (self.xyz >> 5) & 0x1F;
        let z = (self.xyz >> 10) & 0x1F;
        let bt = self.texture_index;
        let l = self.side_and_light;
        write!(
            f,
            "<BlockVertex X:{} Y:{} Z:{} BT:{} L:{} />",
            x, y, z, bt, l
        )
    }
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

        vertices.push(Self::new(x + w, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x, y, z, texture_index, side, light));
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

        vertices.push(Self::new(x + w, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z, texture_index, side, light));
        vertices.push(Self::new(x, y, z, texture_index, side, light));
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

        vertices.push(Self::new(x + w, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x + w, y, z, texture_index, side, light));
        vertices.push(Self::new(x, y, z, texture_index, side, light));
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

        vertices.push(Self::new(x + w, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y, z, texture_index, side, light));
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

        vertices.push(Self::new(x, y + h, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y + h, z, texture_index, side, light));
        vertices.push(Self::new(x, y, z, texture_index, side, light));
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

        vertices.push(Self::new(x, y + h, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y, z + d, texture_index, side, light));
        vertices.push(Self::new(x, y, z, texture_index, side, light));
    }

    fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();
        unsafe {
            let offset = vertex_attrib_int_pointer(stride, 0, 0, gl::UNSIGNED_SHORT, 2);
            let offset = vertex_attrib_int_pointer(stride, 1, offset, gl::UNSIGNED_BYTE, 1);
            vertex_attrib_int_pointer(stride, 2, offset, gl::UNSIGNED_BYTE, 1);
        }
    }
}

impl BlockMesh {
    pub fn draw(&self) {
        self.vao.draw(self.vertex_count.into());
    }

    pub fn new() -> Self {
        let vao = Vao::new_empty("BlockMesh");
        BlockVertex::vertex_attrib_pointers();
        Self {
            vao,
            vertex_count: 0,
        }
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
        self.vertex_count = vertices.len().try_into().unwrap();
    }
}
