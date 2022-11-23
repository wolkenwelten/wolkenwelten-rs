// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::Chungus;
use glam::Vec3;

const ENTITY_SIZE: f32 = 0.4;
const ENTITY_BOUNCE_RATE: f32 = 0.4;
const ENTITY_SLIDE_RATE: f32 = 0.95;

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
    #[inline]
    pub fn pos(&self) -> Vec3 {
        self.pos
    }
    #[inline]
    pub fn rot(&self) -> Vec3 {
        self.rot
    }
    #[inline]
    pub fn vel(&self) -> Vec3 {
        self.vel
    }

    #[inline]
    pub fn set_vel(&mut self, vel: Vec3) {
        self.vel = vel;
    }
    #[inline]
    pub fn set_rot(&mut self, rot: Vec3) {
        self.rot = rot;
    }
    #[inline]
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn is_colliding(&self, world: &Chungus) -> bool {
        let pos = self.pos();
        world.is_solid(pos + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
            | world.is_solid(pos + Vec3::new(ENTITY_SIZE, 0.0, 0.0))
            | world.is_solid(pos + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
            | world.is_solid(pos + Vec3::new(0.0, ENTITY_SIZE, 0.0))
            | world.is_solid(pos + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
            | world.is_solid(pos + Vec3::new(0.0, 0.0, ENTITY_SIZE))
    }

    pub fn tick(&mut self, world: &Chungus) -> bool {
        let mut bounce = false;

        if world.is_solid(self.pos + Vec3::new(ENTITY_SIZE, 0.0, 0.0))
            | world.is_solid(self.pos + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
        {
            self.vel *= ENTITY_SLIDE_RATE;
            self.vel.x *= -ENTITY_BOUNCE_RATE;
            self.pos.x += self.vel.x;
            bounce = true;
        }
        if world.is_solid(self.pos + Vec3::new(0.0, ENTITY_SIZE, 0.0))
            | world.is_solid(self.pos + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
        {
            self.vel *= ENTITY_SLIDE_RATE;
            self.vel.y *= -ENTITY_BOUNCE_RATE;
            self.pos.y += self.vel.y;
            bounce = true;
        }
        if world.is_solid(self.pos + Vec3::new(0.0, 0.0, ENTITY_SIZE))
            | world.is_solid(self.pos + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
        {
            self.vel *= ENTITY_SLIDE_RATE;
            self.vel.z *= -ENTITY_BOUNCE_RATE;
            self.pos.z += self.vel.z;
            bounce = true;
        }
        let v = self.vel;
        self.pos += v;
        self.vel.y -= 0.0005;
        self.rot.y += 0.2;

        bounce
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity() {
        let mut e = Entity::new();
        assert_eq!(e.pos(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(e.rot(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(e.vel(), Vec3::new(0.0, 0.0, 0.0));
        e.set_pos(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), e.clone().pos());
        e.set_vel(Vec3::new(1.0, 1.0, 1.0));
    }
}
