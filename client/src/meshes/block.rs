// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glium::texture::Texture2dArray;
use glium::uniforms::Sampler;
use glium::{uniform, Surface};
use glutin::surface::WindowSurface;
use std::time::Instant;
use wolkenwelten_core::{BlockType, ChunkBlockData, ChunkFluidData, ChunkLightData};
use wolkenwelten_meshgen;
use wolkenwelten_meshgen::BlockVertex;

use crate::{ClientState, QueueEntry, RENDER_DISTANCE};

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
        display: &glium::Display<WindowSurface>,
        square_count: usize,
    ) -> Result<glium::IndexBuffer<u32>, glium::index::BufferCreationError> {
        let mut v: Vec<u32> = Vec::with_capacity(square_count * 6);
        for i in 0..square_count {
            v.push((i * 4) as u32);
            v.push((i * 4 + 1) as u32);
            v.push((i * 4 + 2) as u32);

            v.push((i * 4 + 2) as u32);
            v.push((i * 4 + 3) as u32);
            v.push((i * 4) as u32);
        }
        glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            v.as_slice(),
        )
    }

    pub fn last_updated(&self) -> Instant {
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

    pub fn new(display: &glium::Display<WindowSurface>) -> Result<BlockMesh> {
        let buffer: glium::VertexBuffer<BlockVertex> = glium::VertexBuffer::empty(display, 0)?;
        Ok(Self {
            buffer,
            side_square_count: [0; 6],
            side_start: [0; 6],
            first_created: Instant::now(),
            last_updated: Instant::now(),
        })
    }

    pub fn update(
        &mut self,
        display: &glium::Display<WindowSurface>,
        chunks: &[&ChunkBlockData; 27],
        lights: &[&ChunkLightData; 27],
        block_types: &Vec<BlockType>,
        now: Instant,
    ) -> Result<()> {
        self.last_updated = now;

        let (vertices, side_start_count) =
            wolkenwelten_meshgen::generate(chunks, lights, block_types);
        self.side_square_count = side_start_count;
        self.side_start[0] = 0;
        for i in 1..6 {
            self.side_start[i] = self.side_start[i - 1] + self.side_square_count[i - 1];
        }
        self.buffer = glium::VertexBuffer::dynamic(display, &vertices)?;
        Ok(())
    }

    pub fn update_simple(
        &mut self,
        display: &glium::Display<WindowSurface>,
        chunk: &ChunkBlockData,
        light: &ChunkLightData,
        block_types: &Vec<BlockType>,
        now: Instant,
    ) -> Result<()> {
        self.last_updated = now;

        let (vertices, side_start_count) =
            wolkenwelten_meshgen::generate_simple(chunk, light, block_types);
        self.side_square_count = side_start_count;
        self.side_start[0] = 0;
        for i in 1..6 {
            self.side_start[i] = self.side_start[i - 1] + self.side_square_count[i - 1];
        }
        self.buffer = glium::VertexBuffer::dynamic(display, &vertices)?;
        Ok(())
    }

    pub fn update_fluid(
        &mut self,
        display: &glium::Display<WindowSurface>,
        chunks: &[&ChunkBlockData; 27],
        lights: &[&ChunkLightData; 27],
        fluids: &[&ChunkFluidData; 27],
        fluid_types: &Vec<BlockType>,
        now: Instant,
    ) -> Result<()> {
        self.last_updated = now;

        let (vertices, side_start_count) =
            wolkenwelten_meshgen::generate_fluid(chunks, lights, fluids, fluid_types);
        self.side_square_count = side_start_count;
        self.side_start[0] = 0;
        for i in 1..6 {
            self.side_start[i] = self.side_start[i - 1] + self.side_square_count[i - 1];
        }
        self.buffer = glium::VertexBuffer::dynamic(display, &vertices)?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        fe: &ClientState,
        entry: &QueueEntry,
        mat_mv: [[f32; 4]; 4],
        mat_mvp: [[f32; 4]; 4],
        cur_tex: Sampler<Texture2dArray>,
        alpha: f32,
    ) -> Result<()> {
        let mask = entry.mask;
        let trans_pos = [entry.trans.x, entry.trans.y, entry.trans.z];
        let uniforms = uniform! {
            color_alpha: alpha,
            mat_mvp: mat_mvp,
            mat_mv: mat_mv,
            fade_distance: RENDER_DISTANCE,
            trans_pos: trans_pos,
            cur_tex: cur_tex,
        };
        let draw_parameters = glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::draw_parameters::Blend {
                color: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::One,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            depth: glium::draw_parameters::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        if mask == 0b111111 {
            let index_count = (self.side_start[5] + self.side_square_count[5]) * 6;
            if let Some(indeces) = fe.block_indeces().slice(..index_count) {
                frame.draw(
                    self.buffer(),
                    indeces,
                    &fe.shaders.block,
                    &uniforms,
                    &draw_parameters,
                )?;
            }
        } else {
            for i in (0..6).filter(|i| (mask & (1 << i)) != 0) {
                let start_offset = self.side_start[i] * 6;
                let index_count = start_offset + (self.side_square_count[i] * 6);
                if index_count == 0 {
                    continue;
                }
                if let Some(indeces) = fe.block_indeces().slice(start_offset..index_count) {
                    frame.draw(
                        self.buffer(),
                        indeces,
                        &fe.shaders.block,
                        &uniforms,
                        &draw_parameters,
                    )?;
                }
            }
        }
        Ok(())
    }
}
