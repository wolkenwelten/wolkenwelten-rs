// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::{Mat4, Vec3, Vec3Swizzles};
use glium::Display;
use rand::prelude::*;
use rand::Rng;
use rand_xorshift::XorShiftRng;
use std::rc::Rc;
use std::{cell::RefCell, time::Instant};
use wolkenwelten_client::{ClientState, Frustum, RenderInitArgs, RenderPassArgs, VoxelMesh};
use wolkenwelten_core::{BlockItem, Chungus, Entity, Health, Item, Message, Reactor, SfxId};

const MOB_ACCELERATION: f32 = 0.005;
const MOB_STOP_RATE: f32 = MOB_ACCELERATION * 2.0;

#[derive(Copy, Clone, Debug)]
enum MobAnimationState {
    Idle(Instant),
    Walk(Instant),
    Run(Instant),
    WalkBack(Instant),
    TurnRight(Instant),
    TurnLeft(Instant),
}

impl Default for MobAnimationState {
    fn default() -> Self {
        Self::Idle(Instant::now())
    }
}

#[derive(Clone, Default, Debug)]
struct Mob {
    ent: Entity,
    model_index: i32,
    animation_state: MobAnimationState,
    health: Health,
}

fn mob_load_meshes(display: &Display) -> Result<Vec<Vec<VoxelMesh>>> {
    Ok(vec![
        vec![
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_1.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_2.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/walk_1.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_1.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/walk_2.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_1.vox"))?,
        ],
        vec![
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/king_crab/idle_1.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/king_crab/idle_2.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/king_crab/walk_1.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/king_crab/idle_1.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/king_crab/walk_2.vox"))?,
            VoxelMesh::from_vox_data(display, include_bytes!("../assets/king_crab/idle_1.vox"))?,
        ],
    ])
}

impl Mob {
    pub fn new(pos: Vec3, mut rot: Vec3, model_index: i32) -> Self {
        let mut ent = Entity::new();
        ent.set_pos(pos);
        rot.x = 0.0;
        rot.z = 0.0;
        ent.set_rot(rot);
        ent.set_size(2.0);
        Self {
            ent,
            model_index,
            animation_state: MobAnimationState::Walk(Instant::now()),
            health: Health::new(12),
        }
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
    pub fn set_vel(&mut self, vel: Vec3) {
        self.ent.set_vel(vel);
    }
    #[inline]
    pub fn model_index(&self) -> i32 {
        self.model_index
    }

    pub fn anime_index(&self) -> usize {
        match self.animation_state {
            MobAnimationState::Idle(t) => (t.elapsed().as_millis() as usize / 1000) % 2,
            MobAnimationState::TurnLeft(t)
            | MobAnimationState::TurnRight(t)
            | MobAnimationState::WalkBack(t)
            | MobAnimationState::Walk(t) => 2 + (t.elapsed().as_millis() as usize / 200) % 4,
            MobAnimationState::Run(t) => 2 + (t.elapsed().as_millis() as usize / 100) % 4,
        }
    }

    #[inline]
    pub fn tick(&mut self, world: &Chungus, rng: &mut XorShiftRng) {
        if !world.is_loaded(self.ent.pos) {
            return; // Just freeze the mob until we have loaded the area, this shouldn't happen if at all possible
        }

        let mut goal_vel = Vec3::ZERO;
        match self.animation_state {
            MobAnimationState::Idle(_t) => {
                if rng.gen_range(0..10000) == 0 {
                    self.animation_state = MobAnimationState::Run(Instant::now())
                }
                if rng.gen_range(0..10000) == 0 {
                    self.animation_state = MobAnimationState::WalkBack(Instant::now())
                }
                if rng.gen_range(0..5000) == 0 {
                    self.animation_state = MobAnimationState::Walk(Instant::now())
                }
                if rng.gen_range(0..500) == 0 {
                    self.animation_state = MobAnimationState::TurnLeft(Instant::now())
                }
                if rng.gen_range(0..500) == 0 {
                    self.animation_state = MobAnimationState::TurnRight(Instant::now())
                }
            }
            MobAnimationState::Run(_t) => {
                if rng.gen_range(0..400) == 0 {
                    self.animation_state = MobAnimationState::Idle(Instant::now())
                };
                goal_vel = self.ent.walk_direction();
            }
            MobAnimationState::Walk(_t) => {
                if rng.gen_range(0..4000) == 0 {
                    self.animation_state = MobAnimationState::Idle(Instant::now())
                };
                goal_vel = self.ent.walk_direction() * 0.5;
            }
            MobAnimationState::WalkBack(_t) => {
                if rng.gen_range(0..1000) == 0 {
                    self.animation_state = MobAnimationState::Idle(Instant::now())
                };
                goal_vel = self.ent.walk_direction() * -0.15;
            }
            MobAnimationState::TurnLeft(_t) => {
                if rng.gen_range(0..100) == 0 {
                    self.animation_state = MobAnimationState::Idle(Instant::now())
                };
                self.ent.set_rot(self.ent.rot() - Vec3::new(0.0, 0.1, 0.0));
            }
            MobAnimationState::TurnRight(_t) => {
                if rng.gen_range(0..100) == 0 {
                    self.animation_state = MobAnimationState::Idle(Instant::now())
                };
                self.ent.set_rot(self.ent.rot() + Vec3::new(0.0, 0.1, 0.0));
            }
        };

        let accel = if goal_vel.xz().length() > 0.01 {
            MOB_ACCELERATION
        } else {
            MOB_STOP_RATE
        };

        self.ent.set_vel(Vec3::new(
            self.ent.vel.x * (1.0 - accel) + (goal_vel.x * 0.02) * accel,
            self.ent.vel.y,
            self.ent.vel.z * (1.0 - accel) + (goal_vel.z * 0.02) * accel,
        ));
        self.ent.tick(world);
    }

    fn draw(
        &self,
        frame: &mut glium::Frame,
        fe: &ClientState,
        meshes: &[VoxelMesh],
        view: &Mat4,
        projection: &Mat4,
    ) -> Result<()> {
        let rot = self.rot();
        let pos = self.pos();
        let model = Mat4::from_scale(Vec3::new(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0));
        let model = Mat4::from_rotation_x(rot.x.to_radians()) * model;
        let model = Mat4::from_rotation_y(rot.y.to_radians()) * model;
        let model = Mat4::from_translation(pos) * model;
        let vp = projection.mul_mat4(view);
        let mvp = vp.mul_mat4(&model);

        meshes[self.anime_index()].draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
    }
}

#[derive(Clone, Default, Debug)]
struct MobList {
    mobs: Vec<Mob>,
}

impl MobList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, pos: Vec3, rot: Vec3, model_index: i32) {
        self.mobs.push(Mob::new(pos, rot, model_index));
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Mob> {
        self.mobs.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Mob> {
        self.mobs.iter_mut()
    }

    pub fn tick_all(
        &mut self,
        reactor: &Reactor<Message>,
        player_pos: Vec3,
        world: &Chungus,
        rng: &mut XorShiftRng,
    ) {
        self.mobs.retain_mut(|m| {
            m.tick(world, rng);
            let dist = m.pos() - player_pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;
            if m.health.is_dead() {
                let item = Item::Block(BlockItem::new(18, rng.gen_range(1..=3)));
                let pos = m.pos();
                reactor.defer(Message::ItemDropNew { pos, item });
                reactor.defer(Message::MobDied { pos });
            }
            m.health.is_alive() && dd < (256.0 * 256.0)
        });
    }
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    let mobs: Rc<RefCell<MobList>> = Rc::new(RefCell::new(MobList::new()));
    let rng = Rc::new(RefCell::new(XorShiftRng::from_entropy()));
    {
        let player = args.game.player_rc();
        let mobs = mobs.clone();
        let world = args.game.world_rc();
        let rng = rng.clone();
        let f = move |reactor: &Reactor<Message>, _msg: Message| {
            let mut rng = rng.borrow_mut();
            let player_pos = player.borrow().pos();
            mobs.borrow_mut()
                .tick_all(reactor, player_pos, &world.borrow(), &mut rng);
        };
        args.reactor
            .add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
    }
    {
        let mobs = mobs.clone();
        let f = move |reactor: &Reactor<Message>, msg: Message| {
            if let Message::CharacterAttack {
                char_pos,
                attack_pos,
                damage,
                ..
            } = msg
            {
                mobs.borrow_mut()
                    .iter_mut()
                    .filter(|m| (attack_pos - m.pos()).length_squared() < 2.0 * 2.0)
                    .for_each(|m| {
                        let mut dir = (char_pos - m.pos()).normalize();
                        dir.y = -0.5;
                        m.set_vel(dir * -0.04);
                        m.health -= damage;
                        if m.health.is_dead() {
                            reactor.defer(Message::CharacterGainExperience {
                                pos: m.pos(),
                                xp: 8,
                            });
                        }
                        let msg = Message::MobHurt {
                            pos: m.pos(),
                            damage,
                        };
                        reactor.reply(msg);
                        reactor.defer(msg);
                        reactor.defer(Message::SfxPlay {
                            pos: m.pos(),
                            volume: 0.3,
                            sfx: SfxId::Punch,
                        });
                    });
            }
        };
        args.reactor.add_sink(
            Message::CharacterAttack {
                char_pos: Vec3::ZERO,
                attack_pos: Vec3::ZERO,
                damage: 0,
            },
            Box::new(f),
        );
    }
    {
        let mobs = mobs.clone();
        let f = move |reactor: &Reactor<Message>, msg: Message| {
            if let Message::Explosion { pos, power } = msg {
                let p = power * power;
                mobs.borrow_mut()
                    .iter_mut()
                    .filter(|m| (pos - m.pos()).length_squared() < p)
                    .for_each(|m| {
                        let d = pos - m.pos();
                        let p = d.length() * 0.2;
                        let mut dir = d.normalize() * d * 0.2;
                        dir.y = -0.5;
                        m.set_vel(dir * -0.04);
                        let damage = p.ceil() as i16;
                        m.health -= damage;
                        reactor.defer(Message::MobHurt {
                            pos: m.pos(),
                            damage,
                        });
                    });
            }
        };
        args.reactor.add_sink(
            Message::Explosion {
                pos: Vec3::ZERO,
                power: 0.0,
            },
            Box::new(f),
        );
    }
    {
        let mobs = mobs.clone();
        let f = move |_reactor: &Reactor<Message>, msg: Message| {
            if let Message::WorldgenSpawnMob { pos, .. } = msg {
                let mut rng = rng.borrow_mut();
                let model_index = rng.gen_range(0..=1);
                mobs.borrow_mut().add(
                    pos,
                    Vec3::new(0.0, rng.gen_range(0.0..360.0), 0.0),
                    model_index,
                );
            }
        };
        args.reactor
            .add_sink(Message::WorldgenSpawnMob { pos: Vec3::ZERO }, Box::new(f));
    }
    {
        let mobs = mobs.clone();
        args.render_reactor.entity_provider.push(Box::new(move |v| {
            for e in mobs.borrow().iter() {
                v.push(e.ent.clone());
            }
        }));
    }
    {
        let meshes: Vec<Vec<VoxelMesh>> =
            mob_load_meshes(&args.fe.display).expect("Error loading crab mesh");
        args.render_reactor
            .world_render
            .push(Box::new(move |args: RenderPassArgs| {
                let mvp = args.projection * args.view;
                let frustum = Frustum::extract(&mvp);
                for entity in mobs.borrow().iter() {
                    if frustum
                        .contains_cube(entity.pos() - entity.ent.size(), entity.ent.size() * 2.0)
                    {
                        let _ = entity.draw(
                            args.frame,
                            args.fe,
                            &meshes[entity.model_index() as usize],
                            &args.view,
                            &args.projection,
                        );
                    }
                }
                args
            }));
    }
    args
}
