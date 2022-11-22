// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Character, Chungus, Entity};
use glam::Vec3;
use wolkenwelten_common::{GameEvent, Message};

#[derive(Clone, Debug, Default)]
pub struct Grenade {
    ent: Entity,
}

impl Grenade {
    #[inline]
    pub fn new() -> Self {
        Self { ent: Entity::new() }
    }
    #[inline]
    pub fn pos(&self) -> Vec3 {
        self.ent.pos()
    }
    #[inline]
    pub fn rot(&self) -> Vec3 {
        self.ent.rot()
    }
    #[inline]
    pub fn vel(&self) -> Vec3 {
        self.ent.vel()
    }

    #[inline]
    pub fn set_vel(&mut self, vel: Vec3) {
        self.ent.set_vel(vel);
    }
    #[inline]
    pub fn set_rot(&mut self, rot: Vec3) {
        self.ent.set_rot(rot);
    }
    #[inline]
    pub fn set_pos(&mut self, pos: Vec3) {
        self.ent.set_pos(pos);
    }

    #[inline]
    pub fn is_colliding(&self, world: &Chungus) -> bool {
        self.ent.is_colliding(world)
    }

    #[inline]
    pub fn tick(&mut self, world: &Chungus) -> bool {
        self.ent.tick(world)
    }

    pub fn tick_all(
        grenades: &mut Vec<Self>,
        events: &mut Vec<Message>,
        player: &Character,
        world: &Chungus,
    ) {
        for index in (0..grenades.len()).rev() {
            let bounce = grenades[index].tick(world);

            if bounce {
                events.push(GameEvent::EntityCollision(grenades[index].pos()).into());
            }

            let dist = grenades[index].pos() - player.pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;
            if bounce || (dd > (256.0 * 256.0)) {
                grenades.swap_remove(index); // Remove when far enough away
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity() {
        let mut e = Grenade::new();
        assert_eq!(e.pos(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(e.rot(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(e.vel(), Vec3::new(0.0, 0.0, 0.0));
        e.set_pos(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), e.clone().pos());
        e.set_vel(Vec3::new(1.0, 1.0, 1.0));
    }
}
