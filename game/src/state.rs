// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Character, Chungus, Entity};
use anyhow::Result;
use glam::{IVec3, Vec3};
use std::time::Instant;
use wolkenwelten_common::{
    ChunkBlockData, ChunkLightData, ChunkRequestQueue, GameEvent, InputEvent, Message, SyncEvent,
    CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE,
};
use wolkenwelten_scripting::Runtime;

const MS_PER_TICK: u64 = 4;

pub struct GameState {
    pub clock: Instant,
    pub ticks_elapsed: u64,
    pub last_gc: u64,
    pub running: bool,
    pub entities: Vec<Entity>,
    pub runtime: Runtime,
    pub world: Chungus,

    player: Character,
    render_distance: f32,
}

impl GameState {
    pub fn new() -> Result<Self> {
        let running = true;
        let entities = Vec::new();
        let player = Character::new();
        let world = Chungus::new()?;
        let mut ret = Self {
            clock: Instant::now(),
            running,
            player,
            entities,
            ticks_elapsed: 0,
            last_gc: 0,
            render_distance: 128.0 * 128.0,
            runtime: Runtime::new(),
            world,
        };
        ret.player_rebirth();
        Ok(ret)
    }

    #[inline]
    pub fn get_millis(&self) -> u64 {
        self.clock.elapsed().as_millis().try_into().unwrap()
    }

    #[inline]
    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }

    #[inline]
    pub fn push_entity(&mut self, e: Entity) {
        self.entities.push(e);
    }

    #[inline]
    pub fn player(&self) -> &Character {
        &self.player
    }

    #[inline]
    pub fn mut_player(&mut self) -> &mut Character {
        &mut self.player
    }

    pub fn player_rebirth(&mut self) {
        let mut player = Character::new();
        player.set_pos(Vec3::new(-32.0, -16.0, 338.0));
        player.set_rot(Vec3::new(-130.0, 0.0, 0.0));
        self.player = player;
    }

    #[inline]
    pub fn render_distance(&self) -> f32 {
        self.render_distance
    }

    #[inline]
    pub fn set_render_distance(&mut self, render_distance: f32) {
        self.render_distance = render_distance;
    }

    pub fn view_steps(&self) -> i32 {
        (self.render_distance().sqrt() as i32 / CHUNK_SIZE as i32) + 1
    }

    #[inline]
    pub fn ticks(&self) -> u64 {
        self.ticks_elapsed
    }

    pub fn tick(&mut self, msg: &Vec<Message>, request: &mut ChunkRequestQueue) -> Vec<Message> {
        let mut events: Vec<Message> = Vec::new();
        let now = self.get_millis();
        let ticks_goal = now / MS_PER_TICK;
        let to_run = ticks_goal - self.ticks_elapsed;
        let mut player_movement = Vec3::ZERO;

        msg.iter().for_each(|e| match e {
            Message::InputEvent(msg) => match msg {
                InputEvent::PlayerMove(v) => {
                    if v.y > 0.0 && self.player.may_jump(&self.world) {
                        self.player.jump();
                        events.push(GameEvent::CharacterJump(self.player.pos).into())
                    }
                    player_movement = *v;
                }
                InputEvent::PlayerSwitchSelection(d) => {
                    self.mut_player().switch_block_selection(*d)
                }
                InputEvent::PlayerNoClip(b) => self.mut_player().set_no_clip(*b),
                InputEvent::PlayerTurn(v) => {
                    self.player.rot += *v;
                    self.player.wrap_rot();
                }
                InputEvent::PlayerFly(v) => self.player.vel = *v * 0.15,
                InputEvent::PlayerShoot => {
                    if self.player.may_act(now) {
                        self.player.set_cooldown(now + 600);
                        let mut e = Entity::new();
                        e.set_pos(self.player.pos());
                        e.set_vel(self.player.direction() * 0.4);
                        self.push_entity(e);
                        events.push(GameEvent::CharacterShoot(self.player.pos).into())
                    }
                }
                InputEvent::PlayerBlockMine(pos) => {
                    if self.player.may_act(now) {
                        if let Some(b) = self.world.get_block(*pos) {
                            self.player.set_cooldown(now + 300);
                            self.world.set_block(*pos, 0);
                            events.push(GameEvent::BlockMine(*pos, b).into())
                        }
                    }
                }
                InputEvent::PlayerBlockPlace(pos) => {
                    if self.player.may_act(now) {
                        if self.world.get_block(*pos).unwrap_or(0) == 0 {
                            self.player.set_cooldown(now + 300);
                            let b = self.player.block_selection();
                            self.world.set_block(*pos, b);
                            events.push(GameEvent::BlockPlace(*pos, b).into())
                        }
                    }
                }
            },
            _ => (),
        });

        for _ in 0..to_run {
            events.push(SyncEvent::GameTick(self.ticks_elapsed).into());
            self.ticks_elapsed += 1;
            Entity::tick(&mut self.entities, &mut events, &self.player, &self.world);
            self.player.tick(player_movement, &mut events, &self.world);
            self.runtime.tick(self.get_millis());
        }
        if self.ticks_elapsed > self.last_gc {
            self.world.gc(&self.player, self.render_distance);
            self.last_gc = self.ticks_elapsed + 50;
        }
        self.prepare_world(request);
        events
    }

    #[inline]
    pub fn has_chunk(&self, pos: IVec3) -> bool {
        self.world.get(&pos).is_some()
    }

    #[inline]
    pub fn get_chunk_block(&self, pos: IVec3) -> Option<&ChunkBlockData> {
        self.world.get(&pos)
    }

    #[inline]
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

    pub fn prepare_world(&mut self, request: &mut ChunkRequestQueue) {
        let px = (self.player.pos.x as i32) >> CHUNK_BITS;
        let py = (self.player.pos.y as i32) >> CHUNK_BITS;
        let pz = (self.player.pos.z as i32) >> CHUNK_BITS;

        for cx in -1..=1 {
            for cy in -1..=1 {
                for cz in -1..=1 {
                    let pos = IVec3::new(cx + px, cy + py, cz + pz);
                    if self.world.get(&pos).is_none() {
                        request.block(pos);
                    }
                }
            }
        }
    }
}
