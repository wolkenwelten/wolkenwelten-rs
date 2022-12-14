// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{blit_chunk_buffer, blit_chunk_data, ChunkBuffer, ChunkPosIter};
use crate::{
    worldgen, worldgen::WorldgenAssetList, BlockType, ChunkBlockData, ChunkFluidData,
    ChunkLightData, ChunkRequestQueue, GameState, Message, Reactor, CHUNK_BITS, CHUNK_MASK,
    CHUNK_SIZE,
};
use anyhow::Result;
use glam::f32::Vec3;
use glam::i32::IVec3;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Perlin, Seedable};
use rand::Rng;
use rand_xorshift::XorShiftRng;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::Instant;

thread_local! {
    pub static BLOCKS:RefCell<Vec<BlockType>> = RefCell::new(BlockType::new_default());
    pub static FLUIDS:RefCell<Vec<BlockType>> = RefCell::new(BlockType::new_default_fluids());
}

pub struct Chungus {
    chunks_block: HashMap<IVec3, ChunkBlockData>,
    chunks_fluid: HashMap<IVec3, ChunkFluidData>,
    chunks_simple_light: HashMap<IVec3, ChunkLightData>,
    chunks_complex_light: HashMap<IVec3, ChunkLightData>,
    elevation: NoiseMap,
    displacement: NoiseMap,
    noise_map: NoiseMap,
    assets: WorldgenAssetList,
}

impl Chungus {
    pub fn gc_block(&mut self, player_pos: Vec3, render_distance: f32) {
        let max_d = render_distance * render_distance * 4.0;
        self.chunks_block.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32)
                + Vec3::new(
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                )
                - player_pos;
            let d = diff.dot(diff);
            d < (max_d)
        });
    }

    pub fn gc_simple_light(&mut self, player_pos: Vec3, render_distance: f32) {
        let max_d = render_distance * render_distance * 4.0;
        self.chunks_simple_light.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32)
                + Vec3::new(
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                )
                - player_pos;
            let d = diff.dot(diff);
            d < (max_d)
        });
    }

    pub fn gc_complex_light(&mut self, player_pos: Vec3, render_distance: f32) {
        let max_d = render_distance * render_distance * 3.0;
        self.chunks_complex_light.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32)
                + Vec3::new(
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                    CHUNK_SIZE as f32 / 2.0,
                )
                - player_pos;
            let d = diff.dot(diff);
            d < (max_d)
        });
    }

    fn fluid_tick_remove_blocked(buf: &mut ChunkBuffer, blocks: &ChunkBlockData) {
        for (x, y, z) in ChunkPosIter::new() {
            if blocks.data[x][y][z] == 0 {
                continue;
            }
            buf[x + 1][y + 1][z + 1] = 0;
        }
    }

    fn fluid_tick_flow(buf: &mut ChunkBuffer, block: &ChunkBlockData) {
        for (x, y, z) in ChunkPosIter::new() {
            if block.data[x][y][z] != 0 {
                continue;
            }
            let x = x + 1;
            let y = y + 1;
            let z = z + 1;
            if buf[x][y][z] != 0 {
                continue;
            }

            if buf[x + 1][y][z] != 0 {
                buf[x][y][z] = buf[x + 1][y][z];
                continue;
            }
            if buf[x][y][z + 1] != 0 {
                buf[x][y][z] = buf[x][y][z + 1];
                continue;
            }
            if buf[x][y + 1][z] != 0 {
                buf[x][y][z] = buf[x][y + 1][z];
                continue;
            }
        }

        for (x, y, z) in ChunkPosIter::new() {
            let x = CHUNK_SIZE - 1 - x;
            let y = CHUNK_SIZE - 1 - y;
            let z = CHUNK_SIZE - 1 - z;
            if block.data[x][y][z] != 0 {
                continue;
            }
            let x = x + 1;
            let y = y + 1;
            let z = z + 1;
            if buf[x][y][z] != 0 {
                continue;
            }

            if buf[x - 1][y][z] != 0 {
                buf[x][y][z] = buf[x - 1][y][z];
                continue;
            }
            if buf[x][y][z - 1] != 0 {
                buf[x][y][z] = buf[x][y][z - 1];
                continue;
            }
        }
    }

    fn fluid_should_update(
        pos: &IVec3,
        luwc: Instant,
        fluids: &HashMap<IVec3, ChunkFluidData>,
    ) -> bool {
        let p = *pos + IVec3::new(-1, 0, 0);
        if let Some(f) = fluids.get(&p) {
            if f.last_updated() > luwc {
                return true;
            }
        }
        let p = *pos + IVec3::new(1, 0, 0);
        if let Some(f) = fluids.get(&p) {
            if f.last_updated() > luwc {
                return true;
            }
        }
        let p = *pos + IVec3::new(0, 0, -1);
        if let Some(f) = fluids.get(&p) {
            if f.last_updated() > luwc {
                return true;
            }
        }
        let p = *pos + IVec3::new(0, 0, 1);
        if let Some(f) = fluids.get(&p) {
            if f.last_updated() > luwc {
                return true;
            }
        }
        let p = *pos + IVec3::new(0, 1, 0);
        if let Some(f) = fluids.get(&p) {
            if f.last_updated() > luwc {
                return true;
            }
        }
        false
    }

    fn fluid_tick(&mut self, step: usize) {
        let fluids = &self.chunks_fluid;
        let blocks = &self.chunks_block;
        let mut q = vec![];
        for (i, (pos, chunk)) in self.chunks_fluid.iter().enumerate() {
            if (i & 15) != step {
                continue;
            }
            if let Some(block) = blocks.get(pos) {
                let luwc = chunk.last_update_without_changes();
                if luwc > chunk.last_updated()
                    && luwc > block.last_updated()
                    && !Self::fluid_should_update(pos, luwc, fluids)
                {
                    continue;
                }

                let mut buf: ChunkBuffer = [[[0; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
                blit_chunk_data(&mut buf, &chunk.data, [1; 3]);
                {
                    let p = *pos + IVec3::new(-1, 0, 0);
                    if let Some(f) = fluids.get(&p) {
                        blit_chunk_data(&mut buf, &f.data, [1 - CHUNK_SIZE as isize, 1, 1]);
                    }
                    let p = *pos + IVec3::new(1, 0, 0);
                    if let Some(f) = fluids.get(&p) {
                        blit_chunk_data(&mut buf, &f.data, [1 + CHUNK_SIZE as isize, 1, 1]);
                    }
                    let p = *pos + IVec3::new(0, 0, -1);
                    if let Some(f) = fluids.get(&p) {
                        blit_chunk_data(&mut buf, &f.data, [1, 1, 1 - CHUNK_SIZE as isize]);
                    }
                    let p = *pos + IVec3::new(0, 0, 1);
                    if let Some(f) = fluids.get(&p) {
                        blit_chunk_data(&mut buf, &f.data, [1, 1, 1 + CHUNK_SIZE as isize]);
                    }
                    let p = *pos + IVec3::new(0, 1, 0);
                    if let Some(f) = fluids.get(&p) {
                        blit_chunk_data(&mut buf, &f.data, [1, 1 + CHUNK_SIZE as isize, 1]);
                    }
                }
                Self::fluid_tick_flow(&mut buf, block);
                Self::fluid_tick_remove_blocked(&mut buf, block);

                q.push((*pos, buf));
            }
        }

        for (pos, buf) in q {
            if let Some(chunk) = self.chunks_fluid.get_mut(&pos) {
                let start_hash = {
                    let mut s = DefaultHasher::new();
                    chunk.data.hash(&mut s);
                    s.finish()
                };

                blit_chunk_buffer(&mut chunk.data, &buf);

                let end_hash = {
                    let mut s = DefaultHasher::new();
                    chunk.data.hash(&mut s);
                    s.finish()
                };

                if start_hash == end_hash {
                    chunk.set_last_update_without_changes();
                } else {
                    chunk.set_last_updated();
                };
            }
        }
    }

    pub fn add_handler(reactor: &mut Reactor<Message>, game: &GameState) {
        {
            let world = game.world_rc();
            let player = game.player_rc();
            let last_gc = RefCell::new(0);
            let which_gc = RefCell::new(0);
            let f = move |_reactor: &Reactor<Message>, msg: Message| {
                if let Message::FinishedFrame {
                    ticks,
                    render_distance,
                    ..
                } = msg
                {
                    let last = *last_gc.borrow();
                    if ticks > last {
                        let player_pos = player.borrow().pos();
                        last_gc.replace(ticks + 500);
                        let which = *which_gc.borrow() % 3;
                        // We GC each type separately so that the pauses are as small as possible
                        match which {
                            0 => world.borrow_mut().gc_block(player_pos, render_distance),
                            1 => world
                                .borrow_mut()
                                .gc_simple_light(player_pos, render_distance),
                            2 => world
                                .borrow_mut()
                                .gc_complex_light(player_pos, render_distance),
                            _ => (),
                        }
                        which_gc.replace(which + 1);
                    }
                }
            };
            reactor.add_sink(
                Message::FinishedFrame {
                    player_pos: Vec3::ZERO,
                    ticks: 0,
                    render_distance: 0.0,
                },
                Box::new(f),
            );
        }

        {
            let world = game.world_rc();
            let last_gc = RefCell::new(0);
            let i_cell = RefCell::new(0);
            let f = move |_reactor: &Reactor<Message>, msg: Message| {
                if let Message::FinishedFrame { ticks, .. } = msg {
                    if ticks > *last_gc.borrow() {
                        last_gc.replace(ticks + 50);
                        let i = (*i_cell.borrow() + 1) & 15;
                        world.borrow_mut().fluid_tick(i);
                        i_cell.replace(i);
                    }
                }
            };
            reactor.add_sink(
                Message::FinishedFrame {
                    player_pos: Vec3::ZERO,
                    ticks: 0,
                    render_distance: 0.0,
                },
                Box::new(f),
            );
        }
    }

    pub fn handle_requests(&mut self, request: &mut ChunkRequestQueue, reactor: &Reactor<Message>) {
        let mut simple_light_reqs: HashSet<IVec3> = HashSet::new();
        let mut block_reqs: HashSet<IVec3> = HashSet::new();

        request.get_complex_light_mut().retain(|pos| {
            if let Some(neighbors) = self.get_tri_simple_light(pos, &mut simple_light_reqs) {
                if let Some(chunk) = self.get(pos) {
                    let mut light = ChunkLightData::new();
                    light.calculate_complex(chunk, &neighbors);
                    self.chunks_complex_light.insert(*pos, light);
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });
        simple_light_reqs
            .iter()
            .for_each(|pos| request.simple_light(*pos));

        request.get_simple_light_mut().retain(|pos| {
            if let Some(chunk) = self.chunks_block.get(pos) {
                if let Some(light) = self.chunks_simple_light.get_mut(pos) {
                    if light.last_updated() <= chunk.last_updated() {
                        light.calculate(chunk);
                    }
                } else {
                    let mut light = ChunkLightData::new();
                    light.calculate(chunk);
                    self.chunks_simple_light.insert(*pos, light);
                }
                false
            } else {
                block_reqs.insert(*pos);
                true
            }
        });
        block_reqs.iter().for_each(|pos| request.block(*pos));

        for pos in request.get_block_mut().drain() {
            let chunk = self.chunks_block.get(&pos);
            if chunk.is_none() {
                let wg = worldgen::chunk(self, pos, reactor);
                self.chunks_block.insert(pos, wg.0);
                self.chunks_fluid.insert(pos, wg.1);
            }
        }
    }

    pub fn neighbor_off(x: usize, y: usize, z: usize) -> usize {
        (x * 3 * 3) + (y * 3) + z
    }

    pub fn get_tri_chunk(
        &self,
        k: &IVec3,
        req: &mut HashSet<IVec3>,
    ) -> Option<[&ChunkBlockData; 27]> {
        let mut q = vec![];
        for cx in -1..=1 {
            for cy in -1..=1 {
                for cz in -1..=1 {
                    let k = IVec3::new(cx, cy, cz) + *k;
                    if let Some(c) = self.get(&k) {
                        q.push(c);
                    } else {
                        req.insert(k);
                    }
                }
            }
        }
        if let Ok(ret) = q.as_slice().try_into() {
            Some(ret)
        } else {
            None
        }
    }

    pub fn get_tri_fluids(
        &self,
        k: &IVec3,
        req: &mut HashSet<IVec3>,
    ) -> Option<[&ChunkFluidData; 27]> {
        let mut q = vec![];
        for cx in -1..=1 {
            for cy in -1..=1 {
                for cz in -1..=1 {
                    let k = IVec3::new(cx, cy, cz) + *k;
                    if let Some(c) = self.get_fluid(&k) {
                        q.push(c);
                    } else {
                        req.insert(k);
                    }
                }
            }
        }
        if let Ok(ret) = q.as_slice().try_into() {
            Some(ret)
        } else {
            None
        }
    }

    fn get_tri_simple_light(
        &self,
        k: &IVec3,
        req: &mut HashSet<IVec3>,
    ) -> Option<[&ChunkLightData; 27]> {
        let mut q = vec![];
        for cx in -1..=1 {
            for cy in -1..=1 {
                for cz in -1..=1 {
                    let k = IVec3::new(cx, cy, cz) + *k;
                    if let Some(c) = self.chunks_simple_light.get(&k) {
                        if let Some(b) = self.chunks_block.get(&k) {
                            if b.last_updated() < c.last_updated() {
                                q.push(c);
                                continue;
                            }
                        }
                    }
                    req.insert(k);
                }
            }
        }
        if let Ok(ret) = q.as_slice().try_into() {
            Some(ret)
        } else {
            None
        }
    }

    pub fn get_tri_complex_light(
        &self,
        k: &IVec3,
        req: &mut HashSet<IVec3>,
    ) -> Option<[&ChunkLightData; 27]> {
        let mut q = vec![];
        for cx in -1..=1 {
            for cy in -1..=1 {
                for cz in -1..=1 {
                    let k = IVec3::new(cx, cy, cz) + *k;
                    if let Some(c) = self.chunks_complex_light.get(&k) {
                        if let Some(b) = self.chunks_block.get(&k) {
                            if b.last_updated() < c.last_updated() {
                                q.push(c);
                                continue;
                            }
                        }
                    }
                    req.insert(k);
                }
            }
        }
        if let Ok(ret) = q.as_slice().try_into() {
            Some(ret)
        } else {
            None
        }
    }

    #[inline]
    pub fn elevation(&self) -> &NoiseMap {
        &self.elevation
    }

    #[inline]
    pub fn displacement(&self) -> &NoiseMap {
        &self.displacement
    }

    #[inline]
    pub fn noise_map(&self) -> &NoiseMap {
        &self.noise_map
    }

    #[inline]
    pub fn assets(&self) -> &WorldgenAssetList {
        &self.assets
    }

    #[inline]
    pub fn get_fluid(&self, k: &IVec3) -> Option<&ChunkFluidData> {
        self.chunks_fluid.get(k)
    }

    #[inline]
    pub fn get_fluid_mut(&mut self, k: &IVec3) -> Option<&mut ChunkFluidData> {
        self.chunks_fluid.get_mut(k)
    }

    #[inline]
    pub fn get(&self, k: &IVec3) -> Option<&ChunkBlockData> {
        self.chunks_block.get(k)
    }

    #[inline]
    pub fn get_mut(&mut self, k: &IVec3) -> Option<&mut ChunkBlockData> {
        self.chunks_block.get_mut(k)
    }

    #[inline]
    pub fn get_light(&self, k: &IVec3) -> Option<&ChunkLightData> {
        self.chunks_complex_light.get(k)
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks_block.len()
    }

    pub fn should_update(&self, k: &IVec3) -> bool {
        if let Some(chunk) = self.get(k) {
            if let Some(light) = self.chunks_complex_light.get(k) {
                if light.last_updated() > chunk.last_updated() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_loaded(&self, pos: Vec3) -> bool {
        let cp = pos.floor().as_ivec3() >> CHUNK_BITS;
        self.get(&cp).is_some()
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
            chnk.set_block(block, pos & CHUNK_MASK);
        }
    }

    pub fn get_block(&self, pos: IVec3) -> Option<u8> {
        let cp = pos >> CHUNK_BITS;
        self.get(&cp).map(|chnk| chnk.get_block(pos & CHUNK_MASK))
    }

    pub fn get_fluid_block(&self, pos: IVec3) -> Option<u8> {
        let cp = pos >> CHUNK_BITS;
        self.get_fluid(&cp).map(|chnk| chnk.get(pos & CHUNK_MASK))
    }

    pub fn add_explosion(
        &mut self,
        pos: Vec3,
        power: f32,
        rng: &mut XorShiftRng,
        reactor: &Reactor<Message>,
    ) {
        let pos = pos.floor().as_ivec3();
        let p = power.round() as i32;
        let pp = p * p;
        for x in -p..=p {
            for y in -p..=p {
                for z in -p..=p {
                    let cp = x * x + y * y + z * z;
                    if cp < pp {
                        let pos = pos + IVec3::new(x, y, z);
                        if let Some(block) = self.get_block(pos) {
                            if block != 0 {
                                if rng.gen_ratio(1, 100) {
                                    reactor.defer(Message::BlockBreak { pos, block });
                                }
                                self.set_block(pos, 0);
                            }
                        };
                    }
                }
            }
        }
    }

    pub fn new() -> Result<Self> {
        let simplex: Perlin = Perlin::default();
        simplex.set_seed(1234);
        let elevation: NoiseMap = PlaneMapBuilder::<Perlin, 2>::new(simplex)
            .set_size(2048, 2048)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();

        let simplex: Perlin = Perlin::default();
        simplex.set_seed(2345);
        let displacement: NoiseMap = PlaneMapBuilder::<Perlin, 2>::new(simplex)
            .set_size(128, 128)
            .build();

        let simplex: Perlin = Perlin::default();
        simplex.set_seed(3456);
        let noise_map: NoiseMap = PlaneMapBuilder::<Perlin, 2>::new(simplex)
            .set_size(128, 128)
            .build();

        let assets = WorldgenAssetList::new()?;

        Ok(Self {
            chunks_fluid: HashMap::with_capacity(1024),
            chunks_simple_light: HashMap::with_capacity(1024),
            chunks_complex_light: HashMap::with_capacity(1024),
            chunks_block: HashMap::with_capacity(1024),
            elevation,
            displacement,
            noise_map,
            assets,
        })
    }
}
