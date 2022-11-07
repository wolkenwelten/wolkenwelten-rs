// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::BlockMesh;
use glium::texture::{SrgbTexture2dArray, TextureCreationError};
use glium::{uniform, DrawError, Surface};
use std::time::Instant;
use wolkenwelten_common::{BlockType, ChunkBlockData, ChunkLightData};
use wolkenwelten_meshgen;
use wolkenwelten_meshgen::BlockVertex;

#[derive(Debug)]
pub enum VoxelMeshCreationError {
    MissingModel(),
    ReadError(vox_format::reader::Error),
    TextureCreationError(TextureCreationError),
    BufferCreationError(glium::vertex::BufferCreationError),
}

#[derive(Debug)]
pub enum VoxelDrawError {
    DrawError(DrawError),
    UnslicedIndexBuffer,
}

impl From<DrawError> for VoxelDrawError {
    fn from(err: DrawError) -> Self {
        Self::DrawError(err)
    }
}

impl From<vox_format::reader::Error> for VoxelMeshCreationError {
    fn from(err: vox_format::reader::Error) -> Self {
        Self::ReadError(err)
    }
}
impl From<TextureCreationError> for VoxelMeshCreationError {
    fn from(err: TextureCreationError) -> Self {
        Self::TextureCreationError(err)
    }
}
impl From<glium::vertex::BufferCreationError> for VoxelMeshCreationError {
    fn from(err: glium::vertex::BufferCreationError) -> Self {
        Self::BufferCreationError(err)
    }
}

#[derive(Debug)]
pub struct VoxelMesh {
    mesh: BlockMesh,
    texture: glium::texture::SrgbTexture2dArray,
    trans_pos: [f32; 3],
}

impl VoxelMesh {
    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        indeces: &glium::IndexBuffer<u16>,
        program: &glium::Program,
        mat_mvp: &glam::Mat4,
        color_alpha: f32,
    ) -> Result<(), VoxelDrawError> {
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
            Err(VoxelDrawError::UnslicedIndexBuffer)
        }
    }

    pub fn buffer(&self) -> &glium::VertexBuffer<BlockVertex> {
        self.mesh.buffer()
    }

    pub fn texture(&self) -> &glium::texture::SrgbTexture2dArray {
        &self.texture
    }

    pub fn trans_pos(&self) -> [f32; 3] {
        self.trans_pos
    }

    fn texture_from_palette(
        display: &glium::Display,
        palette: &vox_format::types::Palette,
    ) -> Result<SrgbTexture2dArray, VoxelMeshCreationError> {
        let tiles = palette
            .iter()
            .map(|(_i, c)| {
                let buf = [c.r, c.g, c.b, c.a];
                glium::texture::RawImage2d::from_raw_rgba_reversed(&buf[0..], (1, 1))
            })
            .collect();
        let ret = SrgbTexture2dArray::new(display, tiles)?;
        Ok(ret)
    }

    fn mesh_from_model(
        display: &glium::Display,
        model: &vox_format::types::Model,
    ) -> Result<BlockMesh, VoxelMeshCreationError> {
        let mut chunk = ChunkBlockData::new();
        model.voxels.iter().for_each(|vox| {
            let b = vox.color_index.into();
            let pos = (
                (vox.point.x + 1).into(),
                (vox.point.y + 1).into(),
                (vox.point.z + 1).into(),
            );
            chunk.set_block(b, pos);
        });
        let light = ChunkLightData::new(&chunk);

        let mut ret = BlockMesh::new(display)?;
        ret.update(
            display,
            &chunk,
            &light,
            &BlockType::get_vox_types(),
            Instant::now(),
        )?;
        Ok(ret)
    }

    pub fn from_vox_data(
        display: &glium::Display,
        data: &[u8],
    ) -> Result<Self, VoxelMeshCreationError> {
        let vox_data = vox_format::from_slice(data)?;

        if let Some(model) = vox_data.models.first() {
            let mesh = Self::mesh_from_model(display, model)?;
            let texture = Self::texture_from_palette(display, &vox_data.palette)?;
            let trans_pos = [
                model.size.x as f32 * -0.5,
                model.size.y as f32 * -0.5,
                model.size.z as f32 * -0.5,
            ];
            return Ok(Self {
                mesh,
                texture,
                trans_pos,
            });
        }
        Err(VoxelMeshCreationError::MissingModel())
    }
}
