use glam::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Entity {
    pos: Vec3,
    rot: Vec3,
    vel: Vec3,
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

    pub fn tick(&mut self) -> bool {
        self.pos += self.vel;
        self.vel.y -= 0.001;
        self.rot.y += 0.05;

        self.pos.y < -16.0
    }
}
