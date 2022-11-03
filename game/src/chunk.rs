// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::worldgen;
use glam::IVec3;
use wolkenwelten_common::{ChunkBlockData, ChunkLightData};

#[derive(Clone, Debug)]
pub struct Chunk {
    block: ChunkBlockData,
    light: ChunkLightData,
}

impl Chunk {
    pub fn new(pos: IVec3) -> Self {
        let block = worldgen::chunk(pos);
        let light = ChunkLightData::new(&block);
        Self { block, light }
    }

    pub fn get_block(&self) -> &ChunkBlockData {
        &self.block
    }
    pub fn get_block_mut(&mut self) -> &mut ChunkBlockData {
        &mut self.block
    }
    pub fn get_light(&self) -> &ChunkLightData {
        &self.light
    }
    pub fn get_light_mut(&mut self) -> &mut ChunkLightData {
        &mut self.light
    }

    pub fn should_update(&self) -> bool {
        self.block.get_last_updated() >= self.light.get_last_updated()
    }

    pub fn tick(&mut self) {
        if self.should_update() {
            self.light.calculate(&self.block);
        }
    }
}
