use super::vertex::vertex_attrib_int_pointer;
use super::{VAO, VBO};
use gl::types::GLvoid;
use rostregen_backend_lib::{ChunkBlockData, GameState, Side};
use std::fmt;

pub struct BlockMesh {
    vao: VAO,
    vertex_count: u16,
}

#[derive(Copy, Clone, Debug)]
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
        write!(f, "<BlockVertex X:{x} Y:{y} Z:{z} BT:{bt} L:{l} />")
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
        x: u16,
        y: u16,
        z: u16,
        w: u16,
        h: u16,
        d: u16,
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
        x: u16,
        y: u16,
        z: u16,
        w: u16,
        h: u16,
        _d: u16,
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
        x: u16,
        y: u16,
        z: u16,
        w: u16,
        h: u16,
        d: u16,
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
        x: u16,
        y: u16,
        z: u16,
        w: u16,
        _h: u16,
        d: u16,
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
        x: u16,
        y: u16,
        z: u16,
        _w: u16,
        h: u16,
        d: u16,
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
        x: u16,
        y: u16,
        z: u16,
        w: u16,
        h: u16,
        d: u16,
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
        let vao = VAO::new_empty("BlockMesh");
        BlockVertex::vertex_attrib_pointers();
        Self {
            vao,
            vertex_count: 0,
        }
    }

    pub fn update(&mut self, chunk: &ChunkBlockData, game: &GameState) {
        let mut vertices: Vec<BlockVertex> = Vec::with_capacity(65536);
        for x in 0..16_u16 {
            for y in 0..16_u16 {
                for z in 0..16_u16 {
                    let block = chunk.get_block(x.into(), y.into(), z.into());
                    if block > 0 {
                        let b = game.get_block_type(block);
                        let light = 0x0F;
                        BlockVertex::add_front(
                            &mut vertices,
                            x,
                            y,
                            z,
                            1,
                            1,
                            1,
                            b.tex_front(),
                            light,
                        );
                        BlockVertex::add_back(&mut vertices, x, y, z, 1, 1, 1, b.tex_back(), light);
                        BlockVertex::add_top(&mut vertices, x, y, z, 1, 1, 1, b.tex_top(), light);
                        BlockVertex::add_bottom(
                            &mut vertices,
                            x,
                            y,
                            z,
                            1,
                            1,
                            1,
                            b.tex_bottom(),
                            light,
                        );
                        BlockVertex::add_left(&mut vertices, x, y, z, 1, 1, 1, b.tex_left(), light);
                        BlockVertex::add_right(
                            &mut vertices,
                            x,
                            y,
                            z,
                            1,
                            1,
                            1,
                            b.tex_right(),
                            light,
                        );
                    }
                }
            }
        }
        self.vao.bind();
        let vbo_size: u32 = (vertices.len() * std::mem::size_of::<BlockVertex>())
            .try_into()
            .unwrap();
        VBO::buffer_data(vertices.as_ptr() as *const GLvoid, vbo_size);
        BlockVertex::vertex_attrib_pointers();
        self.vertex_count = vertices.len().try_into().unwrap();
    }
}
