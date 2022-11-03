// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::BlockMesh;
use std::time::Instant;
use glium::texture::TextureCreationError;
use wolkenwelten_common::{ChunkBlockData, ChunkLightData, BlockType};
use wolkenwelten_meshgen;
use wolkenwelten_meshgen::BlockVertex;

#[derive(Debug)]
pub struct VoxelMesh {
    mesh: BlockMesh,
    texture: glium::texture::SrgbTexture2dArray,
    trans_pos: [f32; 3],
}

impl VoxelMesh {
    pub fn buffer(&self) -> &glium::VertexBuffer<BlockVertex> {
        self.mesh.buffer()
    }

    pub fn texture(&self) -> &glium::texture::SrgbTexture2dArray {
        &self.texture
    }

    pub fn trans_pos(&self) -> [f32; 3] {
        self.trans_pos
    }

    fn texture_from_palette(display: &glium::Display, palette: &vox_format::types::Palette) -> Result<glium::texture::SrgbTexture2dArray, TextureCreationError> {
        let tiles = palette.iter().map(|(_i,c)| {
            let buf = [c.r, c.g, c.b, c.a];
            glium::texture::RawImage2d::from_raw_rgba_reversed(&buf[0..], (1,1))
        }).collect();
        glium::texture::SrgbTexture2dArray::new(display, tiles)
    }

    fn mesh_from_model(display: &glium::Display, model: &vox_format::types::Model) -> BlockMesh {
        let mut chunk = ChunkBlockData::new();
        model.voxels.iter().for_each(|vox| {
            let b = vox.color_index.into();
            let pos = ((vox.point.x + 1).into(), (vox.point.y + 1).into(), (vox.point.z + 1).into());
            chunk.set_block(b, pos);
        });
        let light = ChunkLightData::new(&chunk);

        let mut ret = BlockMesh::new(display);
        ret.update(display, &chunk, &light, &BlockType::get_vox_types(), Instant::now());
        ret
    }

    pub fn from_vox_data(display: &glium::Display, data: &[u8]) -> Result<Self, ()> {
        let vox_data = vox_format::from_slice(data).unwrap();

        if let Some(model) = vox_data.models.first() {
            let mesh = Self::mesh_from_model(display, model);
            let texture = Self::texture_from_palette(display, &vox_data.palette).unwrap();
            let trans_pos = [model.size.x as f32 * -0.5, model.size.y as f32 * -0.5, model.size.z as f32 * -0.5];
            return Ok(Self { mesh, texture, trans_pos });
        }
        Err(())
    }
}
