/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use glam::IVec3;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub const CHUNK_BITS: i32 = 5;
pub const CHUNK_SIZE: usize = 1 << CHUNK_BITS;
pub const CHUNK_MASK: i32 = CHUNK_SIZE as i32 - 1;

#[derive(Clone, Debug, Default)]
pub struct ChunkLightData {
    pub data: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl ChunkLightData {
    pub fn new() -> Self {
        let mut data = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for (x, data) in data.iter_mut().enumerate().take(CHUNK_SIZE) {
            for (y, data) in data.iter_mut().enumerate().take(CHUNK_SIZE) {
                for (z, datum) in data.iter_mut().enumerate().take(CHUNK_SIZE) {
                    *datum = ((x ^ y ^ z) & 0xF) as u8;
                }
            }
        }

        Self { data }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ChunkBlockData {
    pub data: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl ChunkBlockData {
    pub fn new() -> Self {
        let data = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        Self { data }
    }

    fn worldgen_island(mut self, rng: &mut ChaCha8Rng) -> Self {
        if rng.gen_range(0..4) == 0 {
            self.set_block(3, (8, 15, 8));
        }
        self.set_sphere(
            2,
            (
                CHUNK_SIZE as i32 / 2,
                CHUNK_SIZE as i32 / 2 + 2,
                CHUNK_SIZE as i32 / 2,
            ),
            CHUNK_SIZE as i32 / 3,
        );
        self.set_sphere(
            1,
            (
                CHUNK_SIZE as i32 / 2,
                CHUNK_SIZE as i32 / 2 + 1,
                CHUNK_SIZE as i32 / 2,
            ),
            CHUNK_SIZE as i32 / 3,
        );
        self.set_sphere(
            3,
            (
                CHUNK_SIZE as i32 / 2,
                CHUNK_SIZE as i32 / 2,
                CHUNK_SIZE as i32 / 2,
            ),
            CHUNK_SIZE as i32 / 3,
        );
        if rng.gen_range(0..4) == 0 {
            self.set_box(15, (14, 3, 12), (2, 3, 3));
        }
        self
    }

    fn worldgen_block(mut self, rng: &mut ChaCha8Rng) -> Self {
        let ox = rng.gen_range(0..=CHUNK_SIZE / 8);
        let oy = rng.gen_range(0..=CHUNK_SIZE / 8);
        let oz = rng.gen_range(0..=CHUNK_SIZE / 8);
        let ow = rng.gen_range(0..=CHUNK_SIZE / 8);
        let oh = rng.gen_range(0..=CHUNK_SIZE / 8);
        let od = rng.gen_range(0..=CHUNK_SIZE / 8);
        let block = rng.gen_range(4..16);
        let pos = (
            (CHUNK_SIZE / 2 + ox) as i32,
            (CHUNK_SIZE / 2 + oy) as i32,
            (CHUNK_SIZE / 2 + oz) as i32,
        );
        let size = (
            (CHUNK_SIZE / 4 + ow) as i32,
            (CHUNK_SIZE / 4 + oh) as i32,
            (CHUNK_SIZE / 4 + od) as i32,
        );
        self.set_box(block, pos, size);
        self
    }

    pub fn worldgen(pos: IVec3) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(
            (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z)
                .try_into()
                .unwrap(),
        );
        match rng.gen_range(0..6) {
            0 | 1 => Self::new().worldgen_island(&mut rng),
            2 => Self::new().worldgen_block(&mut rng),
            _ => Self::new(),
        }
    }

    pub fn get_block(&self, (x, y, z): (i32, i32, i32)) -> u8 {
        self.data[x as usize][y as usize][z as usize]
    }
    pub fn set_block(&mut self, block: u8, (x, y, z): (i32, i32, i32)) {
        self.data[x as usize][y as usize][z as usize] = block
    }
    pub fn set_sphere(&mut self, block: u8, (x, y, z): (i32, i32, i32), radius: i32) {
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
