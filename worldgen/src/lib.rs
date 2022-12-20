// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;
use rand::prelude::*;
use rand::Rng;
use rand_xorshift::XorShiftRng;
use wolkenwelten_client::RenderInitArgs;
use wolkenwelten_core::{
    point_lies_within_chunk, ChunkBlockData, ChunkGeneratorResult, Message, Reactor, CHUNK_SIZE,
};

mod asset;
pub use asset::*;

fn gen_fluid(mut ret: ChunkGeneratorResult, water_y: i32) -> ChunkGeneratorResult {
    for x in 0..CHUNK_SIZE as i32 {
        for y in 0..CHUNK_SIZE as i32 {
            if y > water_y {
                continue;
            }
            for z in 0..CHUNK_SIZE as i32 {
                let x = x as usize;
                let y = y as usize;
                let z = z as usize;
                ret.fluid.data[x][y][z] = (ret.block.data[x][y][z] == 0) as u8;
            }
        }
    }
    ret
}

fn grass_height(x: i32, z: i32) -> i32 {
    let d = ((x * x + z * z) as f32).sqrt() as i32;
    let deg = (x as f32).atan2(z as f32);
    let dy = ((deg * 21.0).sin() * 56.0) as i32;
    let dy = dy + ((deg * 35.0).sin() * 16.0) as i32;
    let dy = dy + ((deg * 48.0).sin() * 8.0) as i32;

    let duy = ((deg * 56.0).sin() * 48.0) as i32;
    let duy = duy + ((deg * 61.0).sin() * 30.0) as i32;
    let duy = duy + ((deg * 78.0).sin() * 19.0) as i32;
    let duy = duy + ((deg * 98.0).sin() * 7.0) as i32;

    let y = (2048 - (d + dy)).min((d + dy / 2) - (2048 - 128 + duy));
    if y > 0 {
        (y as f32).sqrt() as i32
    } else {
        y
    }
}

pub fn island_test_primary(
    pos: IVec3,
    reactor: &Reactor<Message>,
    mut result: ChunkGeneratorResult,
) -> ChunkGeneratorResult {
    ASSETS.with(|assets| {
        let mut rng = XorShiftRng::seed_from_u64(
            (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z)
                .try_into()
                .unwrap(),
        );
        let px = pos.x * CHUNK_SIZE as i32;
        let py = pos.y * CHUNK_SIZE as i32;
        let pz = pos.z * CHUNK_SIZE as i32;

        for x in 0..CHUNK_SIZE as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                let pxx = px + x;
                let pzz = pz + z;
                let floor_y = (-28).max(grass_height(pxx, pzz));
                if floor_y < 3 {
                    result
                        .block
                        .set_pillar(23, IVec3::new(x, -(1 << 30) - py, z), floor_y - py);
                    if rng.gen_range(1..2000) == 1 {
                        let i = rng.gen_range(0..assets.rocks.len());
                        let pos = IVec3::new(
                            x - assets.rocks[i].size.x / 2,
                            floor_y - py - 2,
                            z - assets.rocks[i].size.z / 2,
                        );
                        if assets.rocks[i].fits(pos) {
                            result.block.blit(&assets.rocks[i], pos);
                        }
                    }
                } else {
                    result
                        .block
                        .set_pillar(1, IVec3::new(x, -(1 << 30) - py, z), floor_y - py);
                    result
                        .block
                        .set_pillar(2, IVec3::new(x, (floor_y - py) - 1, z), floor_y - py);

                    if rng.gen_range(1..400) == 1 {
                        let i = rng.gen_range(0..assets.bushes.len());
                        let pos = IVec3::new(
                            x - assets.bushes[i].size.x / 2,
                            floor_y - py,
                            z - assets.bushes[i].size.z / 2,
                        );
                        if assets.bushes[i].fits(pos) {
                            result.block.blit(&assets.bushes[i], pos);
                        }
                    } else if rng.gen_range(1..800) == 1 {
                        let mut mob_pos =
                            ((pos * CHUNK_SIZE as i32) + IVec3::new(x, 0, z)).as_vec3();
                        mob_pos.y = floor_y as f32 + 1.0;
                        if point_lies_within_chunk(mob_pos, pos) {
                            reactor.dispatch(Message::WorldgenSpawnMob { pos: mob_pos });
                        }
                    } else if rng.gen_range(1..1000) == 1 {
                        let i = rng.gen_range(0..assets.rocks.len());
                        let pos = IVec3::new(
                            x - assets.rocks[i].size.x / 2,
                            floor_y - py - 2,
                            z - assets.rocks[i].size.z / 2,
                        );
                        if assets.rocks[i].fits(pos) {
                            result.block.blit(&assets.rocks[i], pos);
                        }
                    } else if rng.gen_range(1..150) == 1 {
                        let i = rng.gen_range(0..assets.trees.len());
                        let pos = IVec3::new(
                            x - assets.trees[i].size.x / 2,
                            floor_y - py - 3,
                            z - assets.trees[i].size.z / 2,
                        );
                        if assets.trees[i].fits(pos) {
                            result.block.blit(&assets.trees[i], pos);
                        }
                    } else if rng.gen_range(1..150) == 1 {
                        let i = rng.gen_range(0..assets.spruce_trees.len());
                        let pos = IVec3::new(
                            x - assets.spruce_trees[i].size.x / 2,
                            floor_y - py - 2,
                            z - assets.spruce_trees[i].size.z / 2,
                        );
                        if assets.spruce_trees[i].fits(pos) {
                            result.block.blit(&assets.spruce_trees[i], pos);
                        }
                    }
                };
            }
        }

        gen_fluid(result, -2 - py)
    })
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    let mut world = args.game.world_mut();
    let wg = world.generator_mut();
    {
        wg.chunk_insert_primary("Root".to_owned(), island_test_primary);
    }
    args
}
