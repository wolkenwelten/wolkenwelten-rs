use super::ChunkBlockData;
use crate::BlockType;
use crate::Entity;
use glam::Vec3;
use rand::Rng;

pub struct GameState {
    pub running: bool,
    pub ticks_elapsed: u64,
    pub player_position: Vec3,
    pub player_rotation: Vec3,

    pub entities: Vec<Entity>,
    pub blocks: Vec<BlockType>,
    pub world: ChunkBlockData,
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
                let vx: f32 = (rng.gen::<f32>() - 0.5) * 0.05;
                let vy: f32 = (rng.gen::<f32>() - 0.1) * 0.01;
                let vz: f32 = (rng.gen::<f32>() - 0.5) * 0.05;
                let vel = Vec3::new(vx, vy, vz);
                e.set_vel(&vel);

                entities.push(e);
            }
        }
        let blocks = BlockType::load_all();
        let world = ChunkBlockData::test();

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

    pub fn tick(&mut self) {
        self.ticks_elapsed += 1;
        for index in (0..self.entities.len()).rev() {
            if self.entities[index].tick() {
                self.entities.swap_remove(index);
            } else {
                if self.is_solid(&self.entities[index].pos()) {
                    self.entities[index].set_vel(&Vec3::ZERO);
                }
            }
        }
    }

    pub fn get_block_type(&self, i: u8) -> &BlockType {
        &self.blocks[i as usize]
    }
    pub fn is_solid(&self, pos: &Vec3) -> bool {
        if pos.x < -8.0 {
            return false;
        }
        if pos.y < -8.0 {
            return false;
        }
        if pos.z < -8.0 {
            return false;
        }

        if pos.x > 8.0 {
            return false;
        }
        if pos.y > 8.0 {
            return false;
        }
        if pos.z > 8.0 {
            return false;
        }

        let x: i32 = (pos.x as i32) + 8;
        let y: i32 = (pos.y as i32) + 8;
        let z: i32 = (pos.z as i32) + 8;
        let block = self.world.get_block(x, y, z);
        block != 0
    }
}
