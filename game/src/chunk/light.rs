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
use super::*;

#[derive(Clone, Debug, Default)]
pub struct ChunkLightData {
    pub data: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl ChunkLightData {
    pub fn new(chunk: &ChunkBlockData) -> Self {
        let mut ret = Self {
            data: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        };
        ret.calculate(chunk);
        ret
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

    fn _block(&mut self, chunk: &ChunkBlockData) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if chunk.data[x][y][z] == 0 {
                        self.data[x][y][z] >>= 1;
                    }
                }
            }
        }
    }

    pub fn calculate(&mut self, chunk: &ChunkBlockData) {
        self.sunlight(chunk);
        self.blur();
        //self.block(chunk);
    }
}
