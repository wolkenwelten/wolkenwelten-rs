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

mod block;
mod light;
mod worldgen;

pub use block::ChunkBlockData;
pub use light::ChunkLightData;
