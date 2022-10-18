use glam::Vec3;

#[derive(Clone, Debug, Default)]
pub struct Entity {
    pub pos: Vec3,
    pub rot: Vec3,
    pub vel: Vec3,
}

impl Entity {
    pub fn new(pos: Vec3) -> Self {
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
    pub fn _set_pos(&mut self, pos: &Vec3) {
        self.pos = *pos;
    }
}
