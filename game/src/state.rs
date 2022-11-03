// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Character, Chungus, Chunk, Entity};
use glam::{IVec3, Vec3};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;
use wolkenwelten_common::{ChunkBlockData, ChunkLightData, GameEvent, InputEvent, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE};

const MS_PER_TICK: u64 = 4;

#[cfg(debug_assertions)]
const MAX_CHUNKS_GENERATED_PER_FRAME: usize = 1;
#[cfg(not(debug_assertions))]
const MAX_CHUNKS_GENERATED_PER_FRAME: usize = 8;

#[derive(Debug, Default)]
struct QueueEntry {
    dist: i64,
    pos: IVec3,
}

impl QueueEntry {
    fn new(pos: IVec3, dist: i64) -> Self {
        Self { dist, pos }
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueueEntry {}
impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

#[derive(Debug)]
pub struct GameState {
    pub clock: Instant,
    pub ticks_elapsed: u64,
    pub last_gc: u64,
    pub running: bool,
    pub player: Character,
    pub entities: Vec<Entity>,
    pub world: Chungus,
}

impl Default for GameState {
    fn default() -> Self {
        let running = true;
        let entities = Vec::new();
        let mut player = Character::new();
        player.set_pos(Vec3::new(15.0, 0.0, -15.0));

        Self {
            clock: Instant::now(),
            running,
            player,
            entities,
            ticks_elapsed: 0,
            last_gc: 0,
            world: Chungus::default(),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_millis(&self) -> u64 {
        self.clock.elapsed().as_millis().try_into().unwrap()
    }

    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn push_entity(&mut self, e: Entity) {
        self.entities.push(e);
    }

    pub fn tick(&mut self, render_distance: f32, input_events: Vec<InputEvent>) -> Vec<GameEvent> {
        let mut events: Vec<GameEvent> = Vec::new();
        let now = self.get_millis();
        let ticks_goal = now / MS_PER_TICK;
        let to_run = ticks_goal - self.ticks_elapsed;
        let mut player_movement = Vec3::ZERO;

        self.player.wrap_rot();

        input_events.iter().for_each(|e| match e  {
            InputEvent::PlayerMove(v) => {
                if v.y > 0.0 && self.player.may_jump(&self.world) {
                    self.player.jump();
                    events.push(GameEvent::CharacterJump(self.player.pos))
                }
                player_movement = *v;
            }
            InputEvent::PlayerFly(v) => {
                self.player.vel = *v * 0.15
            }
            InputEvent::PlayerShoot() => {
                if self.player.may_act(now) {
                    self.player.set_cooldown(now + 600);
                    let mut e = Entity::new();
                    e.set_pos(self.player.pos());
                    e.set_vel(self.player.direction() * 0.4);
                    self.push_entity(e);
                    events.push(GameEvent::CharacterShoot(self.player.pos))
                }
            }
            InputEvent::PlayerBlockMine(pos) => {
                if self.player.may_act(now) {
                    if let Some(b) = self.world.get_block(*pos) {
                        self.player.set_cooldown(now + 300);
                        self.world.set_block(*pos, 0);
                        events.push(GameEvent::BlockMine(*pos, b))
                    }
                }
            }
            InputEvent::PlayerBlockPlace(pos) => {
                if self.player.may_act(now) {
                    if self.world.get_block(*pos).unwrap_or(0) == 0 {
                        self.player.set_cooldown(now + 300);
                        let b = self.player.block_selection();
                        self.world.set_block(*pos, b);
                        events.push(GameEvent::BlockPlace(*pos, b))
                    }
                }
            }
        });

        for _ in 0..to_run {
            self.ticks_elapsed += 1;
            Entity::tick(&mut self.entities, &mut events, &self.player, &self.world);
            self.player.tick(player_movement, &mut events, &self.world);
        }
        if self.ticks_elapsed > self.last_gc {
            self.world.gc(&self.player, render_distance);
            self.last_gc = self.ticks_elapsed + 50;
        }

        events
    }

    pub fn worldgen_chunk(&mut self, pos: IVec3) -> bool {
        match self.world.get_chunk_mut(&pos) {
            None => {
                self.world.chunks.insert(pos, Chunk::new(pos));
                true
            }
            Some(chunk) => {
                chunk.tick();
                false
            }
        }
    }

    pub fn has_chunk(&self, pos: IVec3) -> bool {
        self.world.get(&pos).is_some()
    }

    pub fn should_update(&self, pos: IVec3) -> bool {
        if let Some(chunk) = self.world.get_chunk(&pos) {
            chunk.should_update()
        } else {
            true
        }
    }

    pub fn get_chunk_block(&self, pos: IVec3) -> Option<&ChunkBlockData> {
        self.world.get(&pos)
    }

    pub fn get_chunk_light(&self, pos: IVec3) -> Option<&ChunkLightData> {
        self.world.get_light(&pos)
    }

    pub fn get_single_block(&self, (x, y, z): (i32, i32, i32)) -> u8 {
        let pos = IVec3::new(
            x / CHUNK_SIZE as i32,
            y / CHUNK_SIZE as i32,
            z / CHUNK_SIZE as i32,
        );
        let chunk = self.get_chunk_block(pos);
        if let Some(chnk) = chunk {
            chnk.data[(x & CHUNK_MASK) as usize][(y & CHUNK_MASK) as usize]
                [(z & CHUNK_MASK) as usize]
        } else {
            0
        }
    }

    pub fn prepare_world(&mut self, view_steps: i32, render_distance: f32) {
        let mut heap: BinaryHeap<QueueEntry> = BinaryHeap::new();

        let px = (self.player.pos.x as i32) >> CHUNK_BITS;
        let py = (self.player.pos.y as i32) >> CHUNK_BITS;
        let pz = (self.player.pos.z as i32) >> CHUNK_BITS;

        self.worldgen_chunk(IVec3::new(px, py, pz));

        for cx in -view_steps..=view_steps {
            for cy in -view_steps..=view_steps {
                for cz in -view_steps..=view_steps {
                    let pos = IVec3::new(cx + px, cy + py, cz + pz);
                    let d = (pos.as_vec3() * CHUNK_SIZE as f32) - self.player.pos;
                    let d = d.dot(d);
                    if d < render_distance && self.should_update(pos) {
                        heap.push(QueueEntry::new(pos, (d * 256.0) as i64));
                    }
                }
            }
        }
        heap.iter()
            .take(MAX_CHUNKS_GENERATED_PER_FRAME)
            .for_each(|e| {
                self.worldgen_chunk(e.pos);
            });
    }
}
