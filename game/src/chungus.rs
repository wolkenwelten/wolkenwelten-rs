// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::block_types;
use super::Character;
use crate::worldgen;
use crate::worldgen::WorldgenAssetList;
use anyhow::Result;
use glam::f32::Vec3;
use glam::i32::IVec3;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Perlin, Seedable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wolkenwelten_common::ChunkRequestQueue;
use wolkenwelten_common::{
    BlockType, ChunkBlockData, ChunkLightData, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE,
};

pub struct Chungus {
    pub blocks: Rc<RefCell<Vec<BlockType>>>,
    pub chunks_block: HashMap<IVec3, ChunkBlockData>,
    pub chunks_light: HashMap<IVec3, ChunkLightData>,
    elevation: NoiseMap,
    displacement: NoiseMap,
    noise_map: NoiseMap,
    assets: WorldgenAssetList,
}

impl Chungus {
    pub fn gc(&mut self, player: &Character, render_distance: f32) {
        let max_d = render_distance * 4.0;
        self.chunks_block.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32)
                + Vec3::new(
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                )
                - player.pos;
            let d = diff.dot(diff);
            d < (max_d)
        });
        self.chunks_light.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32)
                + Vec3::new(
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                )
                - player.pos;
            let d = diff.dot(diff);
            d < (max_d)
        });
    }

    pub fn handle_requests(&mut self, request: &mut ChunkRequestQueue) {
        let mut block_reqs = vec![];
        request.get_block().iter().for_each(|pos| {
            let chunk = self.chunks_block.get(pos);
            if chunk.is_none() {
                self.chunks_block.insert(*pos, worldgen::chunk(self, *pos));
            }
        });
        request.get_block_mut().clear();

        request.get_simple_light_mut().retain(|pos| {
            if let Some(chunk) = self.chunks_block.get(pos) {
                if let Some(light) = self.chunks_light.get_mut(pos) {
                    if light.get_last_updated() <= chunk.get_last_updated() {
                        light.calculate(chunk);
                    }
                } else {
                    self.chunks_light.insert(*pos, ChunkLightData::new(chunk));
                }
                false
            } else {
                block_reqs.push(*pos);
                true
            }
        });
        block_reqs.iter().for_each(|pos| request.block(*pos));
    }

    pub fn get_tri_chunk(&self, k: &IVec3, req: &mut Vec<IVec3>) -> Option<[&ChunkBlockData; 27]> {
        let mut q = vec![];
        for cx in -1..=1 {
            for cy in -1..=1 {
                for cz in -1..=1 {
                    let k = IVec3::new(cx, cy, cz) + *k;
                    if let Some(c) = self.get(&k) {
                        q.push(c);
                    } else {
                        req.push(k);
                    }
                }
            }
        }
        if let Ok(ret) = q.as_slice().try_into() {
            Some(ret)
        } else {
            None
        }
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

    #[inline]
    pub fn assets(&self) -> &WorldgenAssetList {
        &self.assets
    }

    #[inline]
    pub fn get(&self, k: &IVec3) -> Option<&ChunkBlockData> {
        self.chunks_block.get(k)
    }

    #[inline]
    pub fn get_mut(&mut self, k: &IVec3) -> Option<&mut ChunkBlockData> {
        self.chunks_block.get_mut(k)
    }

    #[inline]
    pub fn get_light(&self, k: &IVec3) -> Option<&ChunkLightData> {
        self.chunks_light.get(k)
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks_block.len()
    }

    pub fn should_update(&self, k: &IVec3) -> bool {
        if let Some(chunk) = self.get(k) {
            if let Some(light) = self.chunks_light.get(k) {
                if light.get_last_updated() > chunk.get_last_updated() {
                    return false;
                }
            }
        }
        true
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

impl Chungus {
    pub fn new() -> Result<Self> {
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

        let assets = WorldgenAssetList::new()?;

        Ok(Self {
            blocks: Rc::new(RefCell::new(block_types::load_all())),
            chunks_light: HashMap::with_capacity(1024),
            chunks_block: HashMap::with_capacity(1024),
            elevation,
            displacement,
            noise_map,
            assets,
        })
    }
}
