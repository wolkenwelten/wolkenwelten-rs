// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::BlockMesh;
use anyhow::{anyhow, Result};
use glium::texture::Texture2dArray;
use glium::{uniform, Surface};
use glutin::surface::WindowSurface;
use std::time::Instant;
use wolkenwelten_core::{BlockType, ChunkBlockData, ChunkLightData};
use wolkenwelten_meshgen;
use wolkenwelten_meshgen::BlockVertex;

#[derive(Debug)]
pub struct VoxelMesh {
    mesh: BlockMesh,
    texture: glium::texture::Texture2dArray,
    trans_pos: [f32; 3],
}

impl VoxelMesh {
    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        indeces: &glium::IndexBuffer<u32>,
        program: &glium::Program,
        mat_mvp: &glam::Mat4,
        color_alpha: f32,
    ) -> Result<()> {
        let trans_pos: [f32; 3] = self.trans_pos();
        let mat_mvp = mat_mvp.to_cols_array_2d();

        let index_count = (self.mesh.side_start[5] + self.mesh.side_square_count[5]) * 6;
        if let Some(indeces) = indeces.slice(..index_count) {
            frame.draw(
                self.mesh.buffer(),
                indeces,
                program,
                &uniform! {
                    mat_mvp: mat_mvp,
                    trans_pos: trans_pos,
                    color_alpha: color_alpha,
                    cur_tex: &self.texture,
                },
                &glium::DrawParameters {
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    blend: glium::draw_parameters::Blend::alpha_blending(),
                    depth: glium::draw_parameters::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )?;
            Ok(())
        } else {
            Err(anyhow!("Couldn't slice index buffer"))
        }
    }

    pub fn buffer(&self) -> &glium::VertexBuffer<BlockVertex> {
        self.mesh.buffer()
    }

    pub fn texture(&self) -> &glium::texture::Texture2dArray {
        &self.texture
    }

    pub fn trans_pos(&self) -> [f32; 3] {
        self.trans_pos
    }

    fn texture_from_palette(
        display: &glium::Display<WindowSurface>,
        palette: &vox_format::types::Palette,
    ) -> Result<Texture2dArray> {
        let tiles = palette
            .iter()
            .map(|(_i, c)| {
                let buf = [c.r, c.g, c.b, c.a];
                glium::texture::RawImage2d::from_raw_rgba_reversed(&buf[0..], (1, 1))
            })
            .collect();
        let ret = Texture2dArray::new(display, tiles)?;
        Ok(ret)
    }

    fn mesh_from_model(
        display: &glium::Display<WindowSurface>,
        model: &vox_format::types::Model,
    ) -> Result<BlockMesh> {
        let mut chunk = ChunkBlockData::new();
        model.voxels.iter().for_each(|vox| {
            let b = vox.color_index.into();
            let pos = [
                (vox.point.x + 1).into(),
                (vox.point.z + 1).into(),
                (vox.point.y + 1).into(),
            ]
            .into();
            chunk.set_block(b, pos);
        });
        let light = ChunkLightData::new_simple(&chunk);

        let mut ret = BlockMesh::new(display)?;
        ret.update_simple(
            display,
            &chunk,
            &light,
            &BlockType::get_vox_types(),
            Instant::now(),
        )?;
        Ok(ret)
    }

    pub fn from_vox_data(display: &glium::Display<WindowSurface>, data: &[u8]) -> Result<Self> {
        let vox_data = vox_format::from_slice(data)?;

        if let Some(model) = vox_data.models.first() {
            let mesh = Self::mesh_from_model(display, model)?;
            let texture = Self::texture_from_palette(display, &vox_data.palette)?;
            let trans_pos = [
                model.size.x as f32 * -0.5,
                model.size.z as f32 * -0.5,
                model.size.y as f32 * -0.5,
            ];
            return Ok(Self {
                mesh,
                texture,
                trans_pos,
            });
        }
        Err(anyhow!("Couldn't create mesh from .vox"))
    }
}
