// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::*;
use crate::{ChunkPosIter, CHUNK_MASK, CHUNK_SIZE};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ChunkLightData {
    last_updated: Instant,
    pub data: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl ChunkLightData {
    pub fn new(chunk: &ChunkBlockData) -> Self {
        let mut ret = Self {
            last_updated: Instant::now(),
            data: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        };
        ret.calculate(chunk);
        ret
    }

    pub fn get_last_updated(&self) -> Instant {
        self.last_updated
    }

    fn sunlight(&mut self, chunk: &ChunkBlockData) {
        let mut light: [[u8; CHUNK_SIZE]; CHUNK_SIZE] = [[15; CHUNK_SIZE]; CHUNK_SIZE];
        for y in (0..CHUNK_SIZE).rev() {
            for (x, light) in light.iter_mut().enumerate().take(CHUNK_SIZE) {
                for (z, light) in light.iter_mut().enumerate().take(CHUNK_SIZE) {
                    let b = chunk.data[x][y][z];
                    if b != 0 {
                        *light = 0;
                        self.data[x][y][z] = 0; // blockLight[b]
                    } else {
                        *light = (*light + 1).min(15);
                        self.data[x][y][z] = *light;
                    }
                }
            }
        }
    }

    fn blur_x(&mut self) {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let mut a: i8 = 0;
                let mut b: i8 = 0;
                for x in 0..CHUNK_SIZE {
                    a = a.max(self.data[x][y][z] as i8);
                    self.data[x][y][z] = a as u8;
                    a = (a - 1).max(0);

                    b = b.max(self.data[CHUNK_MASK as usize - x][y][z] as i8);
                    self.data[CHUNK_MASK as usize - x][y][z] = b as u8;
                    b = (b - 1).max(0);
                }
            }
        }
    }

    fn blur_y(&mut self) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let mut a: i8 = 0;
                let mut b: i8 = 0;
                for y in 0..CHUNK_SIZE {
                    a = a.max(self.data[x][y][z] as i8);
                    self.data[x][y][z] = a as u8;
                    a = (a - 1).max(0);

                    b = b.max(self.data[x][CHUNK_MASK as usize - y][z] as i8);
                    self.data[x][CHUNK_MASK as usize - y][z] = b as u8;
                    b = (b - 1).max(0);
                }
            }
        }
    }

    fn blur_z(&mut self) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let mut a: i8 = 0;
                let mut b: i8 = 0;
                for z in 0..CHUNK_SIZE {
                    a = a.max(self.data[x][y][z] as i8);
                    self.data[x][y][z] = a as u8;
                    a = (a - 1).max(0);

                    b = b.max(self.data[x][y][CHUNK_MASK as usize - z] as i8);
                    self.data[x][y][CHUNK_MASK as usize - z] = b as u8;
                    b = (b - 1).max(0);
                }
            }
        }
    }

    fn blur(&mut self) {
        self.blur_z();
        self.blur_x();
        self.blur_y();
    }

    fn ambient_occlusion(&mut self, chunk: &ChunkBlockData) {
        for (x, y, z) in ChunkPosIter::new() {
            if chunk.data[x][y][z] != 0 {
                self.data[x][y][z] /= 2;
            }
        }
    }

    pub fn calculate(&mut self, chunk: &ChunkBlockData) {
        self.sunlight(chunk);
        self.blur();
        self.ambient_occlusion(chunk);
        self.last_updated = Instant::now();
    }
}
