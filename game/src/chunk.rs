// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::worldgen;
use crate::Chungus;
use glam::IVec3;
use wolkenwelten_common::{ChunkBlockData, ChunkLightData};

#[derive(Clone, Debug)]
pub struct Chunk {
    block: ChunkBlockData,
    light: ChunkLightData,
}

impl Chunk {
    pub fn new(world: &Chungus, pos: IVec3) -> Self {
        let block = worldgen::chunk(world, pos);
        let light = ChunkLightData::new(&block);
        Self { block, light }
    }

    #[inline]
    pub fn get_block(&self) -> &ChunkBlockData {
        &self.block
    }
    #[inline]
    pub fn get_block_mut(&mut self) -> &mut ChunkBlockData {
        &mut self.block
    }
    #[inline]
    pub fn get_light(&self) -> &ChunkLightData {
        &self.light
    }
    #[inline]
    pub fn get_light_mut(&mut self) -> &mut ChunkLightData {
        &mut self.light
    }

    #[inline]
    pub fn should_update(&self) -> bool {
        self.block.get_last_updated() >= self.light.get_last_updated()
    }
    #[inline]
    pub fn tick(&mut self) {
        if self.should_update() {
            self.light.calculate(&self.block);
        }
    }
}
