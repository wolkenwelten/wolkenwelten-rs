use super::{Character, Chungus};
use glam::Vec3;

#[derive(Clone, Debug, Default)]
pub struct Entity {
    pub pos: Vec3,
    pub rot: Vec3,
    pub vel: Vec3,
}

impl Entity {
    pub fn new() -> Self {
        let pos = Vec3::ZERO;
        let rot = Vec3::ZERO;
        let vel = Vec3::ZERO;
        Self { pos, rot, vel }
    }
    pub fn pos(&self) -> Vec3 {
        self.pos
    }
    pub fn rot(&self) -> Vec3 {
        self.rot
    }

    pub fn set_vel(&mut self, vel: &Vec3) {
        self.vel = *vel;
    }
    pub fn set_pos(&mut self, pos: &Vec3) {
        self.pos = *pos;
    }

    pub fn tick(entities: &mut Vec<Self>, player: &Character, world: &Chungus) {
        for index in (0..entities.len()).rev() {
            {
                let v = entities[index].vel;
                entities[index].pos += v;
                entities[index].vel.y -= 0.001;
                entities[index].rot.y += 0.05;
            }

            let dist = entities[index].pos - player.pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;

            if dd > (128.0 * 128.0) {
                // Remove when far enough away
                entities.swap_remove(index);
            } else if world.is_solid(entities[index].pos()) {
                entities[index].vel = Vec3::ZERO;
            }
        }
    }
}
