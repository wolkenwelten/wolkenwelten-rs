// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Character, Chungus};
use glam::Vec3;
use wolkenwelten_common::{GameEvent, Message};

const ENTITY_SIZE: f32 = 0.6;
const ENTITY_BOUNCE_RATE: f32 = 0.6;

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
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn is_colliding(&self, world: &Chungus) -> bool {
        world.is_solid(self.pos() + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
            | world.is_solid(self.pos() + Vec3::new(ENTITY_SIZE, 0.0, 0.0))
            | world.is_solid(self.pos() + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
            | world.is_solid(self.pos() + Vec3::new(0.0, ENTITY_SIZE, 0.0))
            | world.is_solid(self.pos() + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
            | world.is_solid(self.pos() + Vec3::new(0.0, 0.0, ENTITY_SIZE))
    }

    pub fn tick(
        entities: &mut Vec<Self>,
        events: &mut Vec<Message>,
        player: &Character,
        world: &Chungus,
    ) {
        for index in (0..entities.len()).rev() {
            let mut bounce = false;

            if world.is_solid(entities[index].pos() + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
                | world.is_solid(entities[index].pos() + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
            {
                entities[index].vel.x *= -ENTITY_BOUNCE_RATE;
                entities[index].pos.x += entities[index].vel.x;
                bounce = true;
            }
            if world.is_solid(entities[index].pos() + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
                | world.is_solid(entities[index].pos() + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
            {
                entities[index].vel.y *= -ENTITY_BOUNCE_RATE;
                entities[index].pos.y += entities[index].vel.y;
                bounce = true;
            }
            if world.is_solid(entities[index].pos() + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
                | world.is_solid(entities[index].pos() + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
            {
                entities[index].vel.z *= -ENTITY_BOUNCE_RATE;
                entities[index].pos.z += entities[index].vel.z;
                bounce = true;
            }
            {
                let v = entities[index].vel;
                entities[index].pos += v;
                entities[index].vel.y -= 0.0005;
                entities[index].rot.y += 0.01;
            }

            let dist = entities[index].pos - player.pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;

            if bounce {
                events.push(GameEvent::EntityCollision(entities[index].pos).into());
            }
            if bounce || (dd > (128.0 * 128.0)) {
                entities.swap_remove(index); // Remove when far enough away
            }
        }
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
