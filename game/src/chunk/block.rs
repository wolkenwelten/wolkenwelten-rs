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
use wolkenwelten_common::CHUNK_SIZE;

#[derive(Clone, Debug, Default)]
pub struct ChunkBlockData {
    pub data: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl ChunkBlockData {
    pub fn new() -> Self {
        let data = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        Self { data }
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
