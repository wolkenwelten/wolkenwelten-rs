// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::CHUNK_SIZE;
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
    pub fn get_block(&self, (x, y, z): (i32, i32, i32)) -> u8 {
        self.data[x as usize][y as usize][z as usize]
    }

    pub fn set_block(&mut self, block: u8, (x, y, z): (i32, i32, i32)) {
        self.last_updated = Instant::now();
        self.data[x as usize][y as usize][z as usize] = block
    }

    pub fn set_sphere(&mut self, block: u8, (x, y, z): (i32, i32, i32), radius: i32) {
        self.last_updated = Instant::now();
        let rr = radius * radius;
        for cx in -radius..radius {
            for cy in -radius..radius {
                for cz in -radius..radius {
                    let dist = (cx * cx) + (cy * cy) + (cz * cz);
                    if dist < rr {
                        self.data[(x + cx) as usize][(y + cy) as usize][(z + cz) as usize] = block;
                    }
                }
            }
        }
    }

    pub fn set_box(&mut self, block: u8, (x, y, z): (i32, i32, i32), (w, h, d): (i32, i32, i32)) {
        self.last_updated = Instant::now();
        for cx in 0..w {
            for cy in 0..h {
                for cz in 0..d {
                    self.data[(x + cx) as usize][(y + cy) as usize][(z + cz) as usize] = block;
                }
            }
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
        assert_eq!(chunk.get_block((0, 0, 0)), 1);
        chunk.set_block(4, (1, 1, 1));
        assert_eq!(sum(&chunk), 5);
        assert_eq!(chunk.get_block((1, 1, 1)), 4);
        chunk.set_box(1, (0, 0, 0), (2, 2, 2));
        assert_eq!(sum(&chunk), 8);
        chunk.set_sphere(1, (8, 8, 8), 4);
        assert_eq!(sum(&chunk), 259);
    }
}
