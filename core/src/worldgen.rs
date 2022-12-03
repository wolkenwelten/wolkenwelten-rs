// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Chungus;
use crate::{ChunkBlockData, Message, Reactor, CHUNK_SIZE};
use glam::IVec3;
use rand::prelude::*;
use rand::Rng;
use rand_xorshift::XorShiftRng;

mod asset;
pub use asset::*;

pub fn chunk(world: &Chungus, pos: IVec3, reactor: &Reactor<Message>) -> ChunkBlockData {
    let ele = world.elevation();
    let dis = world.displacement();
    let noi = world.noise_map();
    let assets = world.assets();

    let mut rng = XorShiftRng::seed_from_u64(
        (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z)
            .try_into()
            .unwrap(),
    );
    let px = pos.x * CHUNK_SIZE as i32;
    let py = pos.y * CHUNK_SIZE as i32;
    let pz = pos.z * CHUNK_SIZE as i32;

    let mut r = ChunkBlockData::default();
    for x in 0..CHUNK_SIZE as i32 {
        for z in 0..CHUNK_SIZE as i32 {
            let nx = ((x + px) + 1028) & 4095;
            let nx = if nx >= 2048 { 4095 - nx } else { nx };
            let nz = ((z + pz) + 1028) & 4095;
            let nz = if nz >= 2048 { 4095 - nz } else { nz };
            let v = ele.get_value(nx as usize, nz as usize) * 512.0;
            let y = if v < 0.0 { v * 0.1 } else { v * 0.3 };

            let nx = ((x + px) + 64) & 255;
            let nx = if nx >= 128 { 255 - nx } else { nx };
            let nz = ((z + pz) + 64) & 255;
            let nz = if nz >= 128 { 255 - nz } else { nz };
            let d = dis.get_value(nx as usize, nz as usize);
            let y = y + d * 8.0;

            let nx = ((x + px) + 64) & 255;
            let nx = if nx >= 128 { 255 - nx } else { nx };
            let nz = ((z + pz) + 64) & 255;
            let nz = if nz >= 128 { 255 - nz } else { nz };
            let d = noi.get_value(nx as usize, nz as usize);
            let y = y + d * 4.0;

            let stone_y = if y > 8.0 {
                (y * 2.0) as i32 + rng.gen_range(-1..=1)
            } else {
                (y * 2.0) as i32
            };
            let grass_y = y as i32 + 6;
            let snow_y = if y > 96.0 { (y * 2.0 + 2.0) as i32 } else { 0 };
            r.set_pillar(1, [x, stone_y - py, z].into(), grass_y - py);
            if grass_y > (stone_y + 4) {
                r.set_pillar(2, [x, grass_y - py - 1, z].into(), grass_y - py);
            } else {
                r.set_pillar(13, [x, stone_y - py, z].into(), snow_y - py);
            }
            r.set_pillar(3, [x, (-(1 << 30)) - py, z].into(), stone_y - py);
            if grass_y > (stone_y + 8) {
                if rng.gen_range(1..400) == 1 {
                    let i = rng.gen_range(0..assets.bushes.len());
                    let pos = IVec3::new(
                        x - assets.bushes[i].size.x / 2,
                        grass_y - py,
                        z - assets.bushes[i].size.z / 2,
                    );
                    if assets.bushes[i].fits(pos) {
                        r.blit(&assets.bushes[i], pos);
                    }
                } else if rng.gen_range(1..10000) == 1 {
                    let mut pos = ((pos * CHUNK_SIZE as i32) + IVec3::new(x, 0, z)).as_vec3();
                    pos.y = grass_y as f32 + 1.0;
                    reactor.dispatch(Message::WorldgenSpawnMob { pos });
                } else if rng.gen_range(1..1000) == 1 {
                    let i = rng.gen_range(0..assets.rocks.len());
                    let pos = IVec3::new(
                        x - assets.rocks[i].size.x / 2,
                        grass_y - py - 2,
                        z - assets.rocks[i].size.z / 2,
                    );
                    if assets.rocks[i].fits(pos) {
                        r.blit(&assets.rocks[i], pos);
                    }
                } else if rng.gen_range(1..150) == 1 {
                    let i = rng.gen_range(0..assets.trees.len());
                    let pos = IVec3::new(
                        x - assets.trees[i].size.x / 2,
                        grass_y - py - 3,
                        z - assets.trees[i].size.z / 2,
                    );
                    if assets.trees[i].fits(pos) {
                        r.blit(&assets.trees[i], pos);
                    }
                } else if rng.gen_range(1..150) == 1 {
                    let i = rng.gen_range(0..assets.spruce_trees.len());
                    let pos = IVec3::new(
                        x - assets.spruce_trees[i].size.x / 2,
                        grass_y - py - 2,
                        z - assets.spruce_trees[i].size.z / 2,
                    );
                    if assets.spruce_trees[i].fits(pos) {
                        r.blit(&assets.spruce_trees[i], pos);
                    }
                }
            }
        }
    }
    r
}
