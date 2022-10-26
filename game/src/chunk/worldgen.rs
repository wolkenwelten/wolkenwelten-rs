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

impl ChunkBlockData {
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
}