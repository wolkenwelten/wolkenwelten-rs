use glam::IVec3;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Debug, Default)]
pub struct ChunkBlockData {
    pub data: [[[u8; 16]; 16]; 16],
}

impl ChunkBlockData {
    pub fn new() -> Self {
        let data = [[[0; 16]; 16]; 16];
        Self { data }
    }

    fn worldgen_island(mut self, rng: &mut ChaCha8Rng) -> Self {
        if rng.gen_range(0..4) == 0 {
            self.set_block(3, (8, 15, 8));
        }
        self.set_sphere(2, (8, 10, 8), 5);
        self.set_sphere(1, (8, 9, 8), 5);
        self.set_sphere(3, (8, 8, 8), 5);
        if rng.gen_range(0..4) == 0 {
            self.set_box(15, (14, 3, 12), (2, 3, 3));
        }

        self
    }

    fn worldgen_block(mut self, rng: &mut ChaCha8Rng) -> Self {
        let ox = rng.gen_range(0..=2);
        let oy = rng.gen_range(0..=2);
        let oz = rng.gen_range(0..=2);
        let ow = rng.gen_range(0..=2);
        let oh = rng.gen_range(0..=2);
        let od = rng.gen_range(0..=2);
        let block = rng.gen_range(4..16);
        self.set_box(block, (8 + ox, 8 + oy, 8 + oz), (4 + ow, 4 + oh, 4 + od));
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
