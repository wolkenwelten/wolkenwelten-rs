// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::{Mat4, Vec3};
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::cell::RefCell;
use wolkenwelten_client::{ClientState, RenderInitArgs, RenderPassArgs, VoxelMesh};
use wolkenwelten_core::{Chungus, Entity, Message, Reactor};

thread_local! {
    pub static GRENADES:RefCell<Vec<Grenade>> = RefCell::new(vec![])
}

#[derive(Clone, Debug, Default)]
pub struct Grenade {
    ent: Entity,
}

fn draw(
    mesh: &VoxelMesh,
    frame: &mut glium::Frame,
    fe: &ClientState,
    entity: &Grenade,
    view: &Mat4,
    projection: &Mat4,
) -> Result<()> {
    let rot = entity.rot();
    let pos = entity.pos();

    let model = Mat4::from_scale(Vec3::new(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0));
    let model = Mat4::from_rotation_x(rot.x.to_radians()) * model;
    let model = Mat4::from_rotation_y(rot.y.to_radians()) * model;
    let model = Mat4::from_translation(pos) * model;
    let vp = projection.mul_mat4(view);
    let mvp = vp.mul_mat4(&model);
    mesh.draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
}

impl Grenade {
    #[inline]
    pub fn new() -> Self {
        Self { ent: Entity::new() }
    }

    #[inline]
    pub fn rot(&self) -> Vec3 {
        self.ent.rot()
    }

    #[inline]
    pub fn pos(&self) -> Vec3 {
        self.ent.pos()
    }

    #[inline]
    pub fn set_pos(&mut self, pos: Vec3) {
        self.ent.set_pos(pos);
    }

    #[inline]
    pub fn set_vel(&mut self, vel: Vec3) {
        self.ent.set_vel(vel);
    }

    #[inline]
    pub fn tick(&mut self, world: &Chungus) -> bool {
        self.ent.tick(world)
    }
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    {
        let player = args.game.player_rc();
        let clock = args.game.clock_rc();
        let f = move |reactor: &Reactor<Message>, _msg: Message| {
            let mut player = player.borrow_mut();
            let now = clock.borrow().elapsed().as_millis() as u64;
            if player.may_act(now) {
                player.set_animation_hit();
                player.set_cooldown(now + 600);
                let mut e = Grenade::new();
                e.set_pos(player.pos());
                e.set_vel(player.direction() * 0.4);
                GRENADES.with(|grenades| {
                    grenades.borrow_mut().push(e);
                });
                reactor.dispatch(Message::CharacterShoot { pos: player.pos() });
            }
        };
        args.reactor.add_sink(Message::PlayerShoot, Box::new(f));
    }
    {
        let player = args.game.player_rc();
        let world = args.game.world_rc();
        let f = move |reactor: &Reactor<Message>, _msg: Message| {
            GRENADES.with(|grenades| {
                let mut grenades = grenades.borrow_mut();
                let world = world.borrow();
                let player_pos = player.borrow().pos();
                grenades.retain_mut(|g| {
                    let bounce = g.tick(&world);

                    if bounce {
                        reactor.defer(Message::EntityCollision { pos: g.pos() })
                    }

                    let dist = g.pos() - player_pos;
                    let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;
                    !bounce && (dd < (256.0 * 256.0))
                });
            });
        };
        args.reactor
            .add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
    }
    {
        let world = args.game.world_rc();
        let rng = RefCell::new(XorShiftRng::from_entropy());
        let f = move |reactor: &Reactor<Message>, msg: Message| {
            if let Message::EntityCollision { pos, .. } = msg {
                world
                    .borrow_mut()
                    .add_explosion(pos, 7.0, &mut rng.borrow_mut(), reactor);
                reactor.defer(Message::Explosion { pos, power: 7.0 });
            }
        };
        args.reactor
            .add_sink(Message::EntityCollision { pos: Vec3::ZERO }, Box::new(f));
    }

    args.render_reactor.entity_provider.push(Box::new(move |v| {
        GRENADES.with(|grenades| {
            for e in grenades.borrow().iter() {
                v.push(e.ent.clone());
            }
        });
    }));

    let mesh: VoxelMesh =
        VoxelMesh::from_vox_data(&args.fe.display, include_bytes!("../assets/grenade.vox"))
            .expect("Error while loading grenade.vox");
    args.render_reactor
        .world_render
        .push(Box::new(move |args: RenderPassArgs| {
            GRENADES.with(|grenades| {
                for entity in grenades.borrow().iter() {
                    let _ = draw(
                        &mesh,
                        args.frame,
                        args.fe,
                        entity,
                        &args.view,
                        &args.projection,
                    );
                }
            });
            args
        }));

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity() {
        let mut e = Grenade::new();
        assert_eq!(e.pos(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(e.rot(), Vec3::new(0.0, 0.0, 0.0));
        e.set_pos(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), e.clone().pos());
        e.set_vel(Vec3::new(1.0, 1.0, 1.0));
    }
}
