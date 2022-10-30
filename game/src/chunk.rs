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
use super::worldgen;
use glam::IVec3;
use wolkenwelten_common::{ChunkBlockData, ChunkLightData};

#[derive(Clone, Debug)]
pub struct Chunk {
    block: ChunkBlockData,
    light: ChunkLightData,
}

impl Chunk {
    pub fn new(pos: IVec3) -> Self {
        let block = worldgen::chunk(pos);
        let light = ChunkLightData::new(&block);
        Self { block, light }
    }

    pub fn get_block(&self) -> &ChunkBlockData {
        &self.block
    }
    pub fn get_block_mut(&mut self) -> &mut ChunkBlockData {
        &mut self.block
    }
    pub fn get_light(&self) -> &ChunkLightData {
        &self.light
    }
    pub fn get_light_mut(&mut self) -> &mut ChunkLightData {
        &mut self.light
    }

    pub fn should_update(&self) -> bool {
        self.block.get_last_updated() >= self.light.get_last_updated()
    }

    pub fn tick(&mut self) {
        if self.should_update() {
            self.light.calculate(&self.block);
        }
    }
}
