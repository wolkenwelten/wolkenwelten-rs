// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Chungus, Entity, GameState};
use glam::Vec3;
use wolkenwelten_common::{Message, Reactor};

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

    pub fn add_handler(reactor: &mut Reactor<Message>, game: &GameState) {
        {
            let player = game.player_ref();
            let clock = game.clock_ref();
            let grenades = game.grenades_ref();
            let f = move |reactor: &Reactor<Message>, _msg: Message| {
                let mut player = player.borrow_mut();
                let now = clock.borrow().elapsed().as_millis() as u64;
                if player.may_act(now) {
                    player.set_animation_hit();
                    player.set_cooldown(now + 600);
                    let mut e = Grenade::new();
                    e.set_pos(player.pos());
                    e.set_vel(player.direction() * 0.4);
                    grenades.borrow_mut().push(e);
                    reactor.dispatch(Message::CharacterShoot(player.pos()));
                }
            };
            reactor.add_sink(Message::PlayerShoot, Box::new(f));
        }
        {
            let player = game.player_ref();
            let world = game.world_ref();
            let grenades = game.grenades_ref();
            let f = move |reactor: &Reactor<Message>, _msg: Message| {
                let mut grenades = grenades.borrow_mut();
                let world = world.borrow();
                let player_pos = player.borrow().pos();
                grenades.retain_mut(|g| {
                    let bounce = g.tick(&world);

                    if bounce {
                        reactor.defer(Message::EntityCollision(g.pos()))
                    }

                    let dist = g.pos() - player_pos;
                    let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;
                    !bounce && (dd < (256.0 * 256.0))
                });
            };
            reactor.add_sink(Message::GameTick(0), Box::new(f));
        }
        {
            let world = game.world_ref();
            let f = move |_reactor: &Reactor<Message>, msg: Message| {
                if let Message::EntityCollision(pos) = msg {
                    world.borrow_mut().add_explosion(pos, 7.0);
                }
            };
            reactor.add_sink(Message::EntityCollision(Vec3::ZERO), Box::new(f));
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
