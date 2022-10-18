use super::{Character, Chungus, ChunkBlockData, Entity};
use glam::f32::Vec3;
use glam::i32::IVec3;
use rand::Rng;

#[derive(Debug, Default)]
pub struct GameState {
    pub running: bool,
    pub ticks_elapsed: u64,
    pub player: Character,
    pub entities: Vec<Entity>,
    pub world: Chungus,
}

impl GameState {
    pub fn new() -> Self {
        let running = true;
        let entities = Self::test_entities();
        let mut player = Character::new();
        player.set_pos(&Vec3::new(9.0, 9.0, 25.0));

        Self {
            running,
            player,
            entities,
            ..Default::default()
        }
    }

    fn test_entities() -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::with_capacity(16);
        let mut rng = rand::thread_rng();
        for x in -2..=2 {
            for z in -2..=2 {
                let y: f32 = (rng.gen::<f32>() * 5.0) + 8.5;
                let mut e = Entity::new();
                e.set_pos(&Vec3::new(x as f32, y, z as f32));
                let vx: f32 = (rng.gen::<f32>() - 0.5) * 0.5;
                let vy: f32 = (rng.gen::<f32>() - 0.1) * 0.1;
                let vz: f32 = (rng.gen::<f32>() - 0.5) * 0.5;
                e.set_vel(&Vec3::new(vx, vy, vz));

                entities.push(e);
            }
        }
        entities
    }

    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn tick(&mut self) {
        self.ticks_elapsed += 1;
        Entity::tick(&mut self.entities, &self.player, &self.world);
        self.player.tick(&self.world);
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
        let px = (self.player.pos.x as i32) / 16;
        let py = (self.player.pos.y as i32) / 16;
        let pz = (self.player.pos.z as i32) / 16;
        for cx in -view_steps..=view_steps {
            for cy in -view_steps..=view_steps {
                for cz in -view_steps..=view_steps {
                    let pos = IVec3::new(cx + px, cy + py, cz + pz);
                    self.worldgen_chunk(pos);
                }
            }
        }
    }
}
