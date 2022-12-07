// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::*;
use crate::{Chungus, ChunkData, ChunkPosIter, CHUNK_MASK, CHUNK_SIZE};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ChunkLightData {
    last_updated: Instant,
    pub data: ChunkData,
}

impl Default for ChunkLightData {
    fn default() -> Self {
        Self {
            last_updated: Instant::now(),
            data: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

impl ChunkLightData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_simple(chunk: &ChunkBlockData) -> Self {
        let mut r = Self::new();
        r.calculate(chunk);
        r
    }

    #[inline]
    pub fn last_updated(&self) -> Instant {
        self.last_updated
    }

    fn sunlight(&mut self, chunk: &ChunkBlockData, light: &mut [[u8; CHUNK_SIZE]; CHUNK_SIZE]) {
        for y in (0..CHUNK_SIZE).rev() {
            for (x, light) in light.iter_mut().enumerate() {
                for (z, light) in light.iter_mut().enumerate() {
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
        let mut light = [[0; CHUNK_SIZE]; CHUNK_SIZE];
        self.sunlight(chunk, &mut light);
        self.blur();
        self.ambient_occlusion(chunk);
        self.last_updated = Instant::now();
    }

    pub fn calculate_complex(&mut self, chunk: &ChunkBlockData, neighbors: &[&ChunkLightData; 27]) {
        let mut light = [[0; CHUNK_SIZE]; CHUNK_SIZE];
        let src = neighbors[Chungus::neighbor_off(1, 2, 1)];
        for (x, light) in light.iter_mut().enumerate() {
            for (z, light) in light.iter_mut().enumerate() {
                *light = src.data[x][0][z];
            }
        }
        self.sunlight(chunk, &mut light);
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                self.data[x][0][z] = self.data[x][0][z].max(
                    (neighbors[Chungus::neighbor_off(1, 0, 1)].data[x][CHUNK_SIZE - 1][z] as i8 - 1)
                        .max(0) as u8,
                );
                self.data[x][CHUNK_SIZE - 1][z] = self.data[x][CHUNK_SIZE - 1][z].max(
                    (neighbors[Chungus::neighbor_off(1, 2, 1)].data[x][0][z] as i8 - 1).max(0)
                        as u8,
                );

                self.data[x][z][0] = self.data[x][z][0].max(
                    (neighbors[Chungus::neighbor_off(1, 1, 0)].data[x][z][CHUNK_SIZE - 1] as i8 - 1)
                        .max(0) as u8,
                );
                self.data[x][z][CHUNK_SIZE - 1] = self.data[x][z][CHUNK_SIZE - 1].max(
                    (neighbors[Chungus::neighbor_off(1, 1, 2)].data[x][z][0] as i8 - 1).max(0)
                        as u8,
                );

                self.data[0][x][z] = self.data[0][x][z].max(
                    (neighbors[Chungus::neighbor_off(0, 1, 1)].data[CHUNK_SIZE - 1][x][z] as i8 - 1)
                        .max(0) as u8,
                );
                self.data[CHUNK_SIZE - 1][x][z] = self.data[CHUNK_SIZE - 1][x][z].max(
                    (neighbors[Chungus::neighbor_off(2, 1, 1)].data[0][x][z] as i8 - 1).max(0)
                        as u8,
                );
            }
        }
        self.blur();
        self.ambient_occlusion(chunk);
        self.last_updated = Instant::now();
    }
}
