// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use std::time::Instant;
use wolkenwelten_common::{BlockType, ChunkBlockData, ChunkLightData};
use wolkenwelten_meshgen;
use wolkenwelten_meshgen::BlockVertex;

#[derive(Debug)]
pub struct BlockMesh {
    buffer: glium::VertexBuffer<BlockVertex>,
    first_created: Instant,
    last_updated: Instant,
    pub side_square_count: [usize; 6],
    pub side_start: [usize; 6],
}

impl BlockMesh {
    pub fn buffer(&self) -> &glium::VertexBuffer<BlockVertex> {
        &self.buffer
    }

    pub fn gen_index_buffer(
        display: &glium::Display,
        square_count: usize,
    ) -> glium::IndexBuffer<u16> {
        let mut v: Vec<u16> = Vec::with_capacity(square_count * 6);
        for i in 0..square_count {
            v.push((i * 4) as u16);
            v.push((i * 4 + 1) as u16);
            v.push((i * 4 + 2) as u16);

            v.push((i * 4 + 2) as u16);
            v.push((i * 4 + 3) as u16);
            v.push((i * 4) as u16);
        }
        let buffer: glium::IndexBuffer<u16> = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            v.as_slice(),
        )
        .unwrap();
        buffer
    }

    pub fn get_last_updated(&self) -> Instant {
        self.last_updated
    }

    pub fn get_first_created(&self) -> Instant {
        self.first_created
    }

    pub fn calc_mask(x_offset: i32, y_offset: i32, z_offset: i32) -> u8 {
        (if z_offset <= 0 { 1 << 0 } else { 0 }
            | if z_offset >= 0 { 1 << 1 } else { 0 }
            | if y_offset <= 0 { 1 << 2 } else { 0 }
            | if y_offset >= 0 { 1 << 3 } else { 0 }
            | if x_offset >= 0 { 1 << 4 } else { 0 }
            | if x_offset <= 0 { 1 << 5 } else { 0 })
    }

    pub fn index_count(&self) -> u32 {
        ((self.side_start[5] + self.side_square_count[5]) * 6) as u32
    }

    pub fn new(display: &glium::Display) -> Self {
        let buffer: glium::VertexBuffer<BlockVertex> =
            glium::VertexBuffer::empty(display, 0).unwrap();
        Self {
            buffer,
            side_square_count: [0; 6],
            side_start: [0; 6],
            first_created: Instant::now(),
            last_updated: Instant::now(),
        }
    }

    pub fn update(
        &mut self,
        display: &glium::Display,
        chunk: &ChunkBlockData,
        light: &ChunkLightData,
        block_types: &Vec<BlockType>,
        now: Instant,
    ) {
        self.last_updated = now;

        let (vertices, side_start_count) =
            wolkenwelten_meshgen::generate(chunk, light, block_types);
        self.side_square_count = side_start_count;
        self.side_start[0] = 0;
        for i in 1..6 {
            self.side_start[i] = self.side_start[i - 1] + self.side_square_count[i - 1];
        }
        self.buffer = glium::VertexBuffer::dynamic(display, &vertices).unwrap();
    }
}
