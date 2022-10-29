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
use super::{BlockType, Character, Chunk, ChunkBlockData};
use crate::ChunkLightData;
use glam::f32::Vec3;
use glam::i32::IVec3;
use std::collections::HashMap;
use wolkenwelten_common::{CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE};

#[derive(Debug)]
pub struct Chungus {
    pub blocks: Vec<BlockType>,
    pub chunks: HashMap<IVec3, Chunk>,
}

impl Chungus {
    pub fn gc(&mut self, player: &Character, render_distance: f32) {
        let max_d = render_distance * 1.5;
        self.chunks.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32) - player.pos;
            let d = diff.dot(diff);
            d < (max_d)
        });
    }

    pub fn get_chunk(&self, k: &IVec3) -> Option<&Chunk> {
        self.chunks.get(k)
    }

    pub fn get_chunk_mut(&mut self, k: &IVec3) -> Option<&mut Chunk> {
        self.chunks.get_mut(k)
    }

    pub fn get(&self, k: &IVec3) -> Option<&ChunkBlockData> {
        match self.chunks.get(k) {
            Some(chunk) => Some(chunk.get_block()),
            None => None,
        }
    }

    pub fn get_mut(&mut self, k: &IVec3) -> Option<&mut ChunkBlockData> {
        match self.chunks.get_mut(k) {
            Some(chunk) => Some(chunk.get_block_mut()),
            None => None,
        }
    }

    pub fn get_light(&self, k: &IVec3) -> Option<&ChunkLightData> {
        match self.chunks.get(k) {
            Some(chunk) => Some(chunk.get_light()),
            None => None,
        }
    }

    pub fn get_block_type(&self, i: u8) -> &BlockType {
        &self.blocks[i as usize]
    }

    pub fn is_solid(&self, pos: Vec3) -> bool {
        let cp = pos.floor().as_ivec3() >> CHUNK_BITS;
        if let Some(chnk) = self.get(&cp) {
            let cx = (pos.x.floor() as i32 & CHUNK_MASK) as usize;
            let cy = (pos.y.floor() as i32 & CHUNK_MASK) as usize;
            let cz = (pos.z.floor() as i32 & CHUNK_MASK) as usize;
            let b = chnk.data[cx][cy][cz];
            b != 0
        } else {
            false
        }
    }

    pub fn is_solid_i(&self, pos: IVec3) -> bool {
        let cp = pos >> CHUNK_BITS;
        if let Some(chnk) = self.get(&cp) {
            let IVec3 { x, y, z } = pos & CHUNK_MASK;
            let b = chnk.data[x as usize][y as usize][z as usize];
            b != 0
        } else {
            false
        }
    }

    pub fn set_block(&mut self, pos: IVec3, block: u8) {
        let cp = pos >> CHUNK_BITS;
        if let Some(chnk) = self.get_mut(&cp) {
            let pos = pos & CHUNK_MASK;
            chnk.set_block(block, (pos.x, pos.y, pos.z));
        }
    }

    pub fn add_explosion(&mut self, pos: &Vec3, power: f32) {
        let pos = pos.floor().as_ivec3();
        let p = power.round() as i32;
        let pp = p * p;
        for x in -p..=p {
            for y in -p..=p {
                for z in -p..=p {
                    let cp = x * x + y * y + z * z;
                    if cp < pp {
                        self.set_block(pos + IVec3::new(x, y, z), 0);
                    }
                }
            }
        }
    }
}

impl Default for Chungus {
    fn default() -> Self {
        Self {
            blocks: BlockType::load_all(),
            chunks: HashMap::with_capacity(512),
        }
    }
}
