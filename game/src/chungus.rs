// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::block_types;
use super::{Character, Chunk};
use glam::f32::Vec3;
use glam::i32::IVec3;
use std::collections::HashMap;
use std::sync::RwLock;
use wolkenwelten_common::{
    BlockType, ChunkBlockData, ChunkLightData, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE,
};

#[derive(Debug)]
pub struct Chungus {
    blocks: RwLock<Vec<BlockType>>,
    chunks: HashMap<IVec3, Chunk>,
}

impl Chungus {
    pub fn gc(&mut self, player: &Character, render_distance: f32) {
        let max_d = render_distance * 1.5;
        self.chunks.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32) - player.pos;
            let d = diff.dot(diff);
            d < (max_d)
        });
    }

    #[inline]
    pub fn blocks(&self) -> &RwLock<Vec<BlockType>> {
        &self.blocks
    }

    #[inline]
    pub fn chunks(&self) -> &HashMap<IVec3, Chunk> {
        &self.chunks
    }

    #[inline]
    pub fn chunks_mut(&mut self) -> &mut HashMap<IVec3, Chunk> {
        &mut self.chunks
    }

    #[inline]
    pub fn get_chunk(&self, k: &IVec3) -> Option<&Chunk> {
        self.chunks.get(k)
    }

    #[inline]
    pub fn get_chunk_mut(&mut self, k: &IVec3) -> Option<&mut Chunk> {
        self.chunks.get_mut(k)
    }

    pub fn get(&self, k: &IVec3) -> Option<&ChunkBlockData> {
        match self.chunks.get(k) {
            Some(chunk) => Some(chunk.get_block()),
            None => None,
        }
    }

    pub fn get_mut(&mut self, k: &IVec3) -> Option<&mut ChunkBlockData> {
        match self.chunks.get_mut(k) {
            Some(chunk) => Some(chunk.get_block_mut()),
            None => None,
        }
    }

    pub fn get_light(&self, k: &IVec3) -> Option<&ChunkLightData> {
        match self.chunks.get(k) {
            Some(chunk) => Some(chunk.get_light()),
            None => None,
        }
    }

    pub fn is_loaded(&self, pos: Vec3) -> bool {
        let cp = pos.floor().as_ivec3() >> CHUNK_BITS;
        self.get(&cp).is_some()
    }

    pub fn is_solid(&self, pos: Vec3) -> bool {
        let cp = pos.floor().as_ivec3() >> CHUNK_BITS;
        if let Some(chnk) = self.get(&cp) {
            let cx = (pos.x.floor() as i32 & CHUNK_MASK) as usize;
            let cy = (pos.y.floor() as i32 & CHUNK_MASK) as usize;
            let cz = (pos.z.floor() as i32 & CHUNK_MASK) as usize;
            let b = chnk.data[cx][cy][cz];
            b != 0
        } else {
            false
        }
    }

    pub fn is_solid_i(&self, pos: IVec3) -> bool {
        let cp = pos >> CHUNK_BITS;
        if let Some(chnk) = self.get(&cp) {
            let IVec3 { x, y, z } = pos & CHUNK_MASK;
            let b = chnk.data[x as usize][y as usize][z as usize];
            b != 0
        } else {
            false
        }
    }

    pub fn set_block(&mut self, pos: IVec3, block: u8) {
        let cp = pos >> CHUNK_BITS;
        if let Some(chnk) = self.get_mut(&cp) {
            let pos = pos & CHUNK_MASK;
            chnk.set_block(block, (pos.x, pos.y, pos.z));
        }
    }

    pub fn get_block(&mut self, pos: IVec3) -> Option<u8> {
        let cp = pos >> CHUNK_BITS;
        if let Some(chnk) = self.get(&cp) {
            let pos = pos & CHUNK_MASK;
            Some(chnk.get_block((pos.x, pos.y, pos.z)))
        } else {
            None
        }
    }

    pub fn add_explosion(&mut self, pos: &Vec3, power: f32) {
        let pos = pos.floor().as_ivec3();
        let p = power.round() as i32;
        let pp = p * p;
        for x in -p..=p {
            for y in -p..=p {
                for z in -p..=p {
                    let cp = x * x + y * y + z * z;
                    if cp < pp {
                        self.set_block(pos + IVec3::new(x, y, z), 0);
                    }
                }
            }
        }
    }
}

impl Default for Chungus {
    fn default() -> Self {
        Self {
            blocks: RwLock::new(block_types::load_all()),
            chunks: HashMap::with_capacity(512),
        }
    }
}
