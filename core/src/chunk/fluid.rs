// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{ChunkData, CHUNK_SIZE};
use glam::IVec3;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ChunkFluidData {
    last_updated: Instant,
    last_update_without_changes: Instant,
    pub data: ChunkData,
}

impl Default for ChunkFluidData {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkFluidData {
    pub fn new() -> Self {
        let data = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let last_updated = Instant::now();
        let last_update_without_changes = last_updated;
        Self {
            data,
            last_updated,
            last_update_without_changes,
        }
    }

    pub fn last_updated(&self) -> Instant {
        self.last_updated
    }

    pub fn last_update_without_changes(&self) -> Instant {
        self.last_update_without_changes
    }

    pub fn set_last_updated(&mut self) {
        self.last_updated = Instant::now();
    }

    pub fn set_last_update_without_changes(&mut self) {
        self.last_update_without_changes = Instant::now();
    }

    pub fn get(&self, pos: IVec3) -> u8 {
        self.data[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn set(&mut self, block: u8, pos: IVec3) {
        self.last_updated = Instant::now();
        self.data[pos.x as usize][pos.y as usize][pos.z as usize] = block
    }

    pub fn set_box(&mut self, block: u8, pos: IVec3, size: IVec3) {
        self.last_updated = Instant::now();
        let [w, h, d] = size.to_array();
        for cx in 0..w {
            for cy in 0..h {
                for cz in 0..d {
                    self.data[(pos.x + cx) as usize][(pos.y + cy) as usize]
                        [(pos.z + cz) as usize] = block;
                }
            }
        }
    }

    pub fn set_pillar(&mut self, block: u8, pos: IVec3, goal_y: i32) {
        let y = pos.y.max(0);
        let goal_y = goal_y.min(CHUNK_SIZE as i32);
        for y in y..goal_y {
            self.data[pos.x as usize][y as usize][pos.z as usize] = block;
        }
    }
}
