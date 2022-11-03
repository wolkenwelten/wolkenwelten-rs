// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE};

pub struct ChunkPosIter {
    i: usize,
}
impl ChunkPosIter {
    pub fn new() -> Self {
        Self { i: 0 }
    }
}

impl Default for ChunkPosIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for ChunkPosIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.i >> (CHUNK_BITS * 2);
        if x >= CHUNK_SIZE {
            return None;
        }
        let y = (self.i >> CHUNK_BITS) & CHUNK_MASK as usize;
        let z = self.i & CHUNK_MASK as usize;
        self.i += 1;
        Some((x as usize, y as usize, z as usize))
    }
}
