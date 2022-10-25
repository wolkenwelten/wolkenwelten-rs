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
use super::{Character, Chungus, ChunkBlockData, Entity};
use crate::{ChunkLightData, CHUNK_MASK, CHUNK_SIZE};
use glam::f32::Vec3;
use glam::i32::IVec3;
use rand::Rng;
use std::time::Instant;

const MS_PER_TICK: u64 = 4;

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
        let entities = Self::test_entities();
        let mut player = Character::new();
        player.set_pos(&Vec3::new(15.0, 0.0, -15.0));

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

    fn test_entities() -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::with_capacity(16);
        let mut rng = rand::thread_rng();
        for x in -2..=2 {
            for z in -2..=2 {
                let y: f32 = (rng.gen::<f32>() * 5.0) + 8.5;
                let mut e = Entity::new();
                e.set_pos(Vec3::new(x as f32, y, z as f32));
                let vx: f32 = (rng.gen::<f32>() - 0.5) * 0.1;
                let vy: f32 = (rng.gen::<f32>() - 0.1) * 0.02;
                let vz: f32 = (rng.gen::<f32>() - 0.5) * 0.1;
                e.set_vel(Vec3::new(vx, vy, vz));

                entities.push(e);
            }
        }
        entities
    }

    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }
    pub fn push_entity(&mut self, e: Entity) {
        self.entities.push(e);
    }

    pub fn tick(&mut self, render_distance: f32) {
        let ticks_goal = self.clock.elapsed().as_millis() as u64 / MS_PER_TICK;
        let to_run = ticks_goal - self.ticks_elapsed;

        for _ in 0..to_run {
            self.ticks_elapsed += 1;
            Entity::tick(&mut self.entities, &self.player, &self.world);
            self.player.tick(&self.world);
        }
        if self.ticks_elapsed > self.last_gc {
            self.world.gc(&self.player, render_distance);
            self.last_gc = self.ticks_elapsed + 50;
        }
    }

    pub fn worldgen_chunk(&mut self, pos: IVec3) {
        let chnk = self.world.get(&pos);
        if chnk.is_none() {
            self.world.insert(pos, ChunkBlockData::worldgen(pos));
            self.world.insert_light(pos, ChunkLightData::new());
        };
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
        let px = (self.player.pos.x as i32) / CHUNK_SIZE as i32;
        let py = (self.player.pos.y as i32) / CHUNK_SIZE as i32;
        let pz = (self.player.pos.z as i32) / CHUNK_SIZE as i32;
        for cx in -view_steps..=view_steps {
            for cy in -view_steps..=view_steps {
                for cz in -view_steps..=view_steps {
                    let pos = IVec3::new(cx + px, cy + py, cz + pz);
                    let d = (pos.as_vec3() * CHUNK_SIZE as f32) - self.player.pos;
                    let d = d.dot(d);
                    if d < render_distance {
                        self.worldgen_chunk(pos);
                    }
                }
            }
        }
    }
}
