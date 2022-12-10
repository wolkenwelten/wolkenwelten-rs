// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
mod block;
mod fluid;
mod light;

use glam::{IVec3, Vec3};

pub use block::ChunkBlockData;
pub use fluid::ChunkFluidData;
pub use light::ChunkLightData;

use crate::{ChunkBuffer, ChunkData, CHUNK_SIZE};

pub fn point_lies_within_chunk(p: Vec3, chunk: IVec3) -> bool {
    let chunk = (chunk * CHUNK_SIZE as i32).as_vec3();
    !(p.x < chunk.x
        || p.x > chunk.x + CHUNK_SIZE as f32
        || p.y < chunk.y
        || p.y > chunk.y + CHUNK_SIZE as f32
        || p.z < chunk.z
        || p.z > chunk.z + CHUNK_SIZE as f32)
}

fn blit_chunk_data_end(off: isize) -> (usize, usize) {
    let start = off;
    let end = off + CHUNK_SIZE as isize;
    (
        start.clamp(0, CHUNK_SIZE as isize + 2) as usize,
        end.clamp(0, CHUNK_SIZE as isize + 2) as usize,
    )
}

pub fn blit_chunk_data(block_data: &mut ChunkBuffer, chunk: &ChunkData, off: [isize; 3]) {
    let (x_start, x_end) = blit_chunk_data_end(off[0]);
    let (y_start, y_end) = blit_chunk_data_end(off[1]);
    let (z_start, z_end) = blit_chunk_data_end(off[2]);

    for (x, block_data) in block_data.iter_mut().enumerate().take(x_end).skip(x_start) {
        let cx = (x as isize - off[0]) as usize;
        for (y, block_data) in block_data.iter_mut().enumerate().take(y_end).skip(y_start) {
            let cy = (y as isize - off[1]) as usize;
            for (z, block_data) in block_data.iter_mut().enumerate().take(z_end).skip(z_start) {
                let cz = (z as isize - off[2]) as usize;
                *block_data = chunk[cx][cy][cz];
            }
        }
    }
}

pub fn blit_chunk_buffer(chunk: &mut ChunkData, buf: &ChunkBuffer) {
    for (x, chunk) in chunk.iter_mut().enumerate() {
        let cx = (x as isize + 1) as usize;
        for (y, chunk) in chunk.iter_mut().enumerate() {
            let cy = (y as isize + 1) as usize;
            for (z, chunk) in chunk.iter_mut().enumerate() {
                let cz = (z as isize + 1) as usize;
                *chunk = buf[cx][cy][cz];
            }
        }
    }
}
