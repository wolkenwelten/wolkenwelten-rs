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
