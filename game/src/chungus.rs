// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::block_types;
use super::{Character, Chunk};
use glam::f32::Vec3;
use glam::i32::IVec3;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Perlin, Seedable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wolkenwelten_common::{
    BlockType, ChunkBlockData, ChunkLightData, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE,
};

pub struct Chungus {
    pub blocks: Rc<RefCell<Vec<BlockType>>>,
    pub chunks: HashMap<IVec3, Chunk>,
    elevation: NoiseMap,
    displacement: NoiseMap,
    noise_map: NoiseMap,
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
    pub fn get_chunk(&self, k: &IVec3) -> Option<&Chunk> {
        self.chunks.get(k)
    }

    #[inline]
    pub fn get_chunk_mut(&mut self, k: &IVec3) -> Option<&mut Chunk> {
        self.chunks.get_mut(k)
    }

    #[inline]
    pub fn elevation(&self) -> &NoiseMap {
        &self.elevation
    }

    #[inline]
    pub fn displacement(&self) -> &NoiseMap {
        &self.displacement
    }

    #[inline]
    pub fn noise_map(&self) -> &NoiseMap {
        &self.noise_map
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
            chnk.set_block(block, pos & CHUNK_MASK);
        }
    }

    pub fn get_block(&mut self, pos: IVec3) -> Option<u8> {
        let cp = pos >> CHUNK_BITS;
        if let Some(chnk) = self.get(&cp) {
            Some(chnk.get_block(pos & CHUNK_MASK))
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
        let simplex: Perlin = Perlin::default();
        simplex.set_seed(1234);
        let elevation: NoiseMap = PlaneMapBuilder::<Perlin, 2>::new(simplex)
            .set_size(2048, 2048)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();

        let simplex: Perlin = Perlin::default();
        simplex.set_seed(2345);
        let displacement: NoiseMap = PlaneMapBuilder::<Perlin, 2>::new(simplex)
            .set_size(128, 128)
            .build();

        let simplex: Perlin = Perlin::default();
        simplex.set_seed(3456);
        let noise_map: NoiseMap = PlaneMapBuilder::<Perlin, 2>::new(simplex)
            .set_size(128, 128)
            .build();

        Self {
            blocks: Rc::new(RefCell::new(block_types::load_all())),
            chunks: HashMap::with_capacity(512),
            elevation,
            displacement,
            noise_map,
        }
    }
}
