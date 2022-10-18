use super::ChunkBlockData;
use crate::BlockType;
use crate::Entity;
use glam::f32::Vec3;
use glam::i32::IVec3;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct GameState {
    pub running: bool,
    pub ticks_elapsed: u64,
    pub player_position: Vec3,
    pub player_rotation: Vec3,

    pub entities: Vec<Entity>,
    pub blocks: Vec<BlockType>,

    pub world: HashMap<IVec3, ChunkBlockData>,
}

impl GameState {
    pub fn new() -> Self {
        let running = true;
        let ticks_elapsed: u64 = 0;
        let player_position: Vec3 = Vec3::new(0.0, 9.0, 0.0);
        let player_rotation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut entities: Vec<Entity> = Vec::with_capacity(16);
        let mut rng = rand::thread_rng();
        for x in -2..=2 {
            for z in -2..=2 {
                let y: f32 = (rng.gen::<f32>() * 5.0) + 8.5;
                let mut e = Entity::new(Vec3::new(x as f32, y, z as f32));
                let vx: f32 = (rng.gen::<f32>() - 0.5) * 0.5;
                let vy: f32 = (rng.gen::<f32>() - 0.1) * 0.1;
                let vz: f32 = (rng.gen::<f32>() - 0.5) * 0.5;
                let vel = Vec3::new(vx, vy, vz);
                e.set_vel(&vel);

                entities.push(e);
            }
        }
        let blocks = BlockType::load_all();
        let world = HashMap::new();

        Self {
            running,
            ticks_elapsed,
            player_position,
            player_rotation,

            blocks,
            entities,
            world,
        }
    }
    pub fn _push_entity(&mut self, e: Entity) {
        self.entities.push(e);
    }

    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn tick(&mut self) {
        self.ticks_elapsed += 1;
        for index in (0..self.entities.len()).rev() {
            {
                let v = self.entities[index].vel;
                self.entities[index].pos += v;
                self.entities[index].vel.y -= 0.001;
                self.entities[index].rot.y += 0.05;
            }

            let dist = self.entities[index].pos - self.player_position;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;

            if dd > (128.0 * 128.0) {
                // Remove when far enough away
                self.entities.swap_remove(index);
            } else if self.is_solid(self.entities[index].pos()) {
                self.entities[index].vel = Vec3::ZERO;
            }
        }
    }

    pub fn worldgen_chunk(&mut self, pos: IVec3) {
        let chnk = self.world.get(&pos);
        if chnk.is_none() {
            self.world.insert(pos, ChunkBlockData::worldgen(pos));
        };
    }

    pub fn get_chunk_block(&self, pos: IVec3) -> Option<&ChunkBlockData> {
        self.world.get(&pos)
    }

    pub fn get_single_block(&self, (x, y, z): (i32, i32, i32)) -> u8 {
        let pos = IVec3::new(x / 16, y / 16, z / 16);
        let chunk = self.get_chunk_block(pos);
        if let Some(chnk) = chunk {
            chnk.data[(x & 15) as usize][(y & 15) as usize][(z & 15) as usize]
        } else {
            0
        }
    }

    pub fn prepare_world(&mut self, view_steps: i32) {
        let px = (self.player_position.x as i32) / 16;
        let py = (self.player_position.y as i32) / 16;
        let pz = (self.player_position.z as i32) / 16;
        for cx in -view_steps..=view_steps {
            for cy in -view_steps..=view_steps {
                for cz in -view_steps..=view_steps {
                    let pos = IVec3::new(cx + px, cy + py, cz + pz);
                    self.worldgen_chunk(pos);
                }
            }
        }
    }

    pub fn get_block_type(&self, i: u8) -> &BlockType {
        &self.blocks[i as usize]
    }
    pub fn is_solid(&self, pos: Vec3) -> bool {
        let cp = IVec3::new(pos.x as i32 >> 4, pos.y as i32 >> 4, pos.z as i32 >> 4);
        let chnk = self.world.get(&cp);
        if let Some(chnk) = chnk {
            let cx = (pos.x as i32 & 15) as usize;
            let cy = (pos.y as i32 & 15) as usize;
            let cz = (pos.z as i32 & 15) as usize;
            let b = chnk.data[cx][cy][cz];
            b != 0
        } else {
            false
        }
    }
}
