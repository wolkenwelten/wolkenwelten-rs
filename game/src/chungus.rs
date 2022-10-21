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
use super::chunk::CHUNK_SIZE;
use super::{BlockType, Character, ChunkBlockData};
use crate::{CHUNK_BITS, CHUNK_MASK};
use glam::f32::Vec3;
use glam::i32::IVec3;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Chungus {
    pub blocks: Vec<BlockType>,
    pub block_data: HashMap<IVec3, ChunkBlockData>,
}

impl Chungus {
    pub fn gc(&mut self, player: &Character) {
        let mut removal_queue: Vec<IVec3> = Vec::new();
        for pos in self.block_data.keys() {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32) - player.pos;
            let d = diff.dot(diff);
            if d > (384.0 * 384.0) {
                removal_queue.push(*pos);
            }
        }
        for pos in removal_queue {
            self.block_data.remove(&pos);
        }
    }
    pub fn get(&self, k: &IVec3) -> Option<&ChunkBlockData> {
        self.block_data.get(k)
    }
    pub fn insert(&mut self, k: IVec3, v: ChunkBlockData) {
        self.block_data.insert(k, v);
    }

    pub fn get_block_type(&self, i: u8) -> &BlockType {
        &self.blocks[i as usize]
    }

    pub fn is_solid(&self, pos: Vec3) -> bool {
        let cp = IVec3::new(
            pos.x.floor() as i32 >> CHUNK_BITS,
            pos.y.floor() as i32 >> CHUNK_BITS,
            pos.z.floor() as i32 >> CHUNK_BITS,
        );
        let chnk = self.get(&cp);
        if let Some(chnk) = chnk {
            let cx = (pos.x.floor() as i32 & CHUNK_MASK) as usize;
            let cy = (pos.y.floor() as i32 & CHUNK_MASK) as usize;
            let cz = (pos.z.floor() as i32 & CHUNK_MASK) as usize;
            let b = chnk.data[cx][cy][cz];
            b != 0
        } else {
            false
        }
    }
}

impl Default for Chungus {
    fn default() -> Self {
        Self {
            blocks: BlockType::load_all(),
            block_data: HashMap::with_capacity(4096),
        }
    }
}
