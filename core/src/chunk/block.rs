// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::CHUNK_SIZE;
use glam::IVec3;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ChunkBlockData {
    last_updated: Instant,
    pub data: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Default for ChunkBlockData {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkBlockData {
    pub fn new() -> Self {
        let data = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let last_updated = Instant::now();
        Self { data, last_updated }
    }

    #[inline]
    pub fn get_last_updated(&self) -> Instant {
        self.last_updated
    }

    #[inline]
    pub fn get_block(&self, pos: IVec3) -> u8 {
        self.data[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn set_block(&mut self, block: u8, pos: IVec3) {
        self.last_updated = Instant::now();
        self.data[pos.x as usize][pos.y as usize][pos.z as usize] = block
    }

    pub fn set_sphere(&mut self, block: u8, pos: IVec3, radius: i32) {
        self.last_updated = Instant::now();
        let rr = radius * radius;
        for cx in -radius..radius {
            for cy in -radius..radius {
                for cz in -radius..radius {
                    let dist = (cx * cx) + (cy * cy) + (cz * cz);
                    if dist < rr {
                        self.data[(pos.x + cx) as usize][(pos.y + cy) as usize]
                            [(pos.z + cz) as usize] = block;
                    }
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sum(chunk: &ChunkBlockData) -> i32 {
        let mut acc: i32 = 0;
        for x in chunk.data.iter() {
            for y in x.iter() {
                for z in y.iter() {
                    acc += *z as i32;
                }
            }
        }
        acc
    }

    #[test]
    fn test_chunk() {
        let chunk = ChunkBlockData::new();
        assert_eq!(sum(&chunk), 0);
        let chunk = chunk.clone();
        assert_eq!(sum(&chunk), 0);
        let mut chunk = ChunkBlockData::default();
        assert_eq!(sum(&chunk), 0);
        chunk.data[0][0][0] = 1;
        assert_eq!(sum(&chunk), 1);
        assert_eq!(chunk.get_block(IVec3::new(0, 0, 0)), 1);
        chunk.set_block(4, IVec3::new(1, 1, 1));
        assert_eq!(sum(&chunk), 5);
        assert_eq!(chunk.get_block(IVec3::new(1, 1, 1)), 4);
        chunk.set_box(1, IVec3::new(0, 0, 0), IVec3::new(2, 2, 2));
        assert_eq!(sum(&chunk), 8);
        chunk.set_sphere(1, IVec3::new(8, 8, 8), 4);
        assert_eq!(sum(&chunk), 259);
    }
}
