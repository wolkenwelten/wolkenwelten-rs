// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;
use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use wolkenwelten_common::{ChunkBlockData, CHUNK_SIZE};

fn worldgen_island(rng: &mut ChaCha8Rng) -> ChunkBlockData {
    let mut chunk = ChunkBlockData::new();
    if rng.gen_range(0..4) == 0 {
        chunk.set_block(3, (8, 15, 8));
    }
    chunk.set_sphere(
        2,
        (
            CHUNK_SIZE as i32 / 2,
            CHUNK_SIZE as i32 / 2 + 2,
            CHUNK_SIZE as i32 / 2,
        ),
        CHUNK_SIZE as i32 / 3,
    );
    chunk.set_sphere(
        1,
        (
            CHUNK_SIZE as i32 / 2,
            CHUNK_SIZE as i32 / 2 + 1,
            CHUNK_SIZE as i32 / 2,
        ),
        CHUNK_SIZE as i32 / 3,
    );
    chunk.set_sphere(
        3,
        (
            CHUNK_SIZE as i32 / 2,
            CHUNK_SIZE as i32 / 2,
            CHUNK_SIZE as i32 / 2,
        ),
        CHUNK_SIZE as i32 / 3,
    );
    if rng.gen_range(0..4) == 0 {
        chunk.set_box(15, (14, 3, 12), (2, 3, 3));
    }
    chunk
}

fn worldgen_block(rng: &mut ChaCha8Rng) -> ChunkBlockData {
    let mut chunk = ChunkBlockData::new();
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
    chunk.set_box(block, pos, size);
    chunk
}

pub fn chunk(pos: IVec3) -> ChunkBlockData {
    let mut rng = ChaCha8Rng::seed_from_u64(
        (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z)
            .try_into()
            .unwrap(),
    );
    match rng.gen_range(0..6) {
        0 | 1 => worldgen_island(&mut rng),
        2 => worldgen_block(&mut rng),
        _ => ChunkBlockData::new(),
    }
}
