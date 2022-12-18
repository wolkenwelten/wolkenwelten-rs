// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::{Mat4, Vec3, Vec3Swizzles};
use glium::Display;
use rand::prelude::*;
use rand::Rng;
use rand_xorshift::XorShiftRng;
use std::f32::consts::PI;
use std::rc::Rc;
use std::{cell::RefCell, time::Instant};
use wolkenwelten_client::{ClientState, Frustum, RenderInitArgs, RenderPassArgs, VoxelMesh};
use wolkenwelten_core::Character;
use wolkenwelten_core::{BlockItem, Chungus, Entity, Health, Item, Message, Reactor, SfxId};

thread_local! {
    pub static MOBS:RefCell<MobList> = RefCell::new(MobList::new());
}

const MOB_SIZE: f32 = 0.4;
const MOB_ACCELERATION: f32 = 0.01;
const MOB_STOP_RATE: f32 = MOB_ACCELERATION * 2.0;
const MOB_STOP_FIGHTING_DISTANCE: f32 = 24.0;
const MOB_START_CHASING_DISTANCE: f32 = 12.0;

const COL_WIDTH: f32 = 0.8;
const COL_DEPTH: f32 = 0.8;

const COL_POINT_TOP: Vec3 = Vec3::new(0.0, MOB_SIZE, 0.0);
const COL_POINT_BOTTOM: Vec3 = Vec3::new(0.0, -MOB_SIZE * 1.5, 0.0);
const COL_POINT_LEFT: Vec3 = Vec3::new(-COL_WIDTH, 0.0, 0.0);
const COL_POINT_RIGHT: Vec3 = Vec3::new(COL_WIDTH, 0.0, 0.0);
const COL_POINT_FRONT: Vec3 = Vec3::new(0.0, 0.0, COL_DEPTH);
const COL_POINT_BACK: Vec3 = Vec3::new(0.0, 0.0, -COL_DEPTH);

#[derive(Copy, Clone, Debug)]
pub enum MobState {
    Idle(Instant),
    Walk(Instant),
    Run(Instant),
    WalkBack(Instant),
    TurnRight(Instant),
    TurnLeft(Instant),
    ChasePlayer(Instant),
    FightPlayer(Instant),
    InstantAttackPlayer(Instant),
    Dance(Instant),
}

impl Default for MobState {
    fn default() -> Self {
        Self::Idle(Instant::now())
    }
}

#[derive(Clone, Debug)]
pub struct Mob {
    pos: Vec3,
    vel: Vec3,
    rot: Vec3,
    movement: Vec3,
    model_index: i32,
    state: MobState,
    health: Health,
    cooldown: Instant,
}

fn mob_load_meshes(display: &Display) -> Result<Vec<Vec<VoxelMesh>>> {
    Ok(vec![vec![
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_1.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_2.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/walk_1.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_1.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/walk_2.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_1.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/idle_1.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/attack_1.vox"))?,
        VoxelMesh::from_vox_data(display, include_bytes!("../assets/crab/attack_2.vox"))?,
    ]])
}

impl Mob {
    pub fn new(pos: Vec3, mut rot: Vec3, model_index: i32) -> Self {
        let mut ent = Entity::new();
        ent.set_pos(pos);
        rot.x = 0.0;
        rot.z = 0.0;
        let vel = Vec3::ZERO;
        ent.set_size(2.0);
        Self {
            pos,
            rot,
            vel,
            movement: Vec3::ZERO,
            model_index,
            state: MobState::Walk(Instant::now()),
            health: Health::new(12),
            cooldown: Instant::now(),
        }
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
    pub fn set_rot(&mut self, rot: Vec3) {
        self.rot = rot;
    }
    #[inline]
    pub fn set_vel(&mut self, vel: Vec3) {
        self.vel = vel;
    }
    #[inline]
    pub fn model_index(&self) -> i32 {
        self.model_index
    }
    #[inline]
    pub fn set_state(&mut self, state: MobState) {
        self.state = state;
    }
    #[inline]
    pub fn set_idle_state(&mut self) {
        self.state = MobState::Idle(Instant::now());
    }

    #[inline]
    pub fn may_instant_attack(&self) -> bool {
        self.cooldown.elapsed().as_millis() > 1200
    }

    #[inline]
    pub fn cooldown(&mut self) {
        self.cooldown = Instant::now();
    }

    pub fn anime_index(&self) -> usize {
        match self.state {
            MobState::InstantAttackPlayer(_) => 8,
            MobState::Dance(t) | MobState::FightPlayer(t) => {
                6 + (t.elapsed().as_millis() as usize / 200) % 3
            }
            MobState::Idle(t) => (t.elapsed().as_millis() as usize / 1000) % 2,
            MobState::TurnLeft(t)
            | MobState::TurnRight(t)
            | MobState::WalkBack(t)
            | MobState::Walk(t) => 2 + (t.elapsed().as_millis() as usize / 200) % 4,
            MobState::ChasePlayer(t) | MobState::Run(t) => {
                2 + (t.elapsed().as_millis() as usize / 100) % 4
            }
        }
    }

    fn player_aggresive(&mut self, player: &Character) {
        if player.no_clip() || player.is_dead() {
            return;
        }
        match self.state {
            MobState::InstantAttackPlayer(_)
            | MobState::ChasePlayer(_)
            | MobState::FightPlayer(_) => return,
            _ => (),
        }
        let player_pos = player.pos();
        let diff = (player_pos - self.pos()).xz();
        let distance = diff.length_squared();
        if distance > MOB_START_CHASING_DISTANCE * MOB_START_CHASING_DISTANCE {
            return;
        }
        self.set_state(MobState::ChasePlayer(Instant::now()));
    }

    pub fn turn_towards(&mut self, goal: Vec3) {
        let a = goal.y - self.rot().y;
        let b = goal.y + 360.0 - self.rot().y;
        let mut rot = self.rot();
        let c = if a.abs() < b.abs() { a } else { b };
        if c > 0.0 {
            rot.y += 1.0;
        } else {
            rot.y -= 1.0;
        }
        self.set_rot(rot);
    }

    pub fn walk_direction(&self) -> Vec3 {
        let a = self.rot;
        Vec3::new(
            ((-a.y - 90.0) * PI / 180.0).cos(),
            0.0,
            ((-a.y - 90.0) * PI / 180.0).sin(),
        )
        .normalize()
    }

    pub fn direction(&self) -> Vec3 {
        let a = self.rot;
        Vec3::new(
            ((a.x - 90.0) * PI / 180.0).cos() * (-a.y * PI / 180.0).cos(),
            (-a.y * PI / 180.0).sin(),
            ((a.x - 90.0) * PI / 180.0).sin() * (-a.y * PI / 180.0).cos(),
        )
    }

    pub fn to_entity(&self) -> Entity {
        Entity {
            pos: self.pos,
            rot: self.rot,
            vel: self.vel,
            size: 2.0,
        }
    }

    pub fn would_collide_at(&self, world: &Chungus, pos: Vec3) -> bool {
        world.is_solid(pos + Vec3::new(-MOB_SIZE, 0.0, 0.0))
            | world.is_solid(pos + Vec3::new(MOB_SIZE, 0.0, 0.0))
            | world.is_solid(pos + Vec3::new(0.0, -MOB_SIZE, 0.0))
            | world.is_solid(pos + Vec3::new(0.0, MOB_SIZE, 0.0))
            | world.is_solid(pos + Vec3::new(0.0, 0.0, -MOB_SIZE))
            | world.is_solid(pos + Vec3::new(0.0, 0.0, MOB_SIZE))
    }

    pub fn is_colliding(&self, world: &Chungus) -> bool {
        self.would_collide_at(world, self.pos())
    }

    fn is_underwater_point(world: &Chungus, pos: Vec3) -> bool {
        if let Some(fluid) = world.get_fluid_block(pos.as_ivec3()) {
            fluid != 0
        } else {
            false
        }
    }

    fn is_solid_pillar(&self, pos: Vec3, world: &Chungus) -> bool {
        world.is_solid(pos)
            || world.is_solid(pos + Vec3::new(0.0, -0.4, 0.0))
            || world.is_solid(pos + Vec3::new(0.0, 0.8, 0.0))
    }

    pub fn is_underwater(&self, world: &Chungus) -> bool {
        Self::is_underwater_point(world, self.pos() + Vec3::new(0.0, -0.8, 0.0))
    }

    #[inline]
    pub fn may_jump(&self, world: &Chungus) -> bool {
        world.is_solid(self.pos + COL_POINT_BOTTOM)
    }

    pub fn tick_physics(&mut self, world: &Chungus) {
        if !world.is_loaded(self.pos) {
            return; // Just freeze the mob until we have loaded the area, this shouldn't happen if at all possible
        }
        let underwater = self.is_underwater(world);

        let accel = if self.movement.xz().length() > 0.01 {
            MOB_ACCELERATION
        } else {
            MOB_STOP_RATE
        };
        let accel = if underwater { accel * 0.5 } else { accel };

        self.vel.x = self.vel.x * (1.0 - accel) + (self.movement.x * 0.01) * accel;
        self.vel.z = self.vel.z * (1.0 - accel) + (self.movement.z * 0.01) * accel;

        self.vel.y -= if underwater { 0.0001 } else { 0.0005 };
        let old = self.vel;

        if underwater {
            self.vel *= 0.99;
            self.vel.y *= 0.997;
        }

        if self.is_solid_pillar(self.pos + COL_POINT_LEFT, world) {
            self.vel.x = self.vel.x.max(0.0);
        }
        if self.is_solid_pillar(self.pos + COL_POINT_RIGHT, world) {
            self.vel.x = self.vel.x.min(0.0);
        }

        if world.is_solid(self.pos + COL_POINT_BOTTOM) {
            self.vel.y = self.vel.y.max(0.0);
        }
        if world.is_solid(self.pos + COL_POINT_TOP) {
            self.vel.y = self.vel.y.min(0.0);
        }

        if self.is_solid_pillar(self.pos + COL_POINT_FRONT, world) {
            self.vel.z = self.vel.z.min(0.0);
        }
        if self.is_solid_pillar(self.pos + COL_POINT_BACK, world) {
            self.vel.z = self.vel.z.max(0.0);
        }

        let force = (old - self.vel).length();
        if force > 0.05 {
            let amount = (force * 14.0) as i16;
            if amount > 0 {
                let damage = amount * amount;
                self.health.damage(damage);
            }
        }

        let len = self.vel.length();
        if len > 0.5 {
            self.vel *= 1.0 - (len - 0.2).clamp(0.0001, 1.0);
        }
        self.pos += self.vel;
    }

    #[inline]
    pub fn tick(
        &mut self,
        world: &Chungus,
        rng: &mut XorShiftRng,
        player: &Character,
        reactor: &Reactor<Message>,
    ) {
        if !world.is_loaded(self.pos) {
            return; // Just freeze the mob until we have loaded the area, this shouldn't happen if at all possible
        }

        self.player_aggresive(player);
        match self.state {
            MobState::Idle(_t) => {
                if rng.gen_range(0..10000) == 0 {
                    self.state = MobState::Run(Instant::now())
                }
                if rng.gen_range(0..10000) == 0 {
                    self.state = MobState::WalkBack(Instant::now())
                }
                if rng.gen_range(0..5000) == 0 {
                    self.state = MobState::Walk(Instant::now())
                }
                if rng.gen_range(0..500) == 0 {
                    self.state = MobState::TurnLeft(Instant::now())
                }
                if rng.gen_range(0..500) == 0 {
                    self.state = MobState::TurnRight(Instant::now())
                }
                self.movement = Vec3::ZERO;
            }
            MobState::Run(_t) => {
                if rng.gen_range(0..400) == 0 {
                    self.set_idle_state();
                };
                self.movement = self.walk_direction() * 2.0;
            }
            MobState::Walk(_t) => {
                if rng.gen_range(0..4000) == 0 {
                    self.set_idle_state();
                };
                self.movement = self.walk_direction() * 1.5;
            }
            MobState::Dance(t) => {
                if t.elapsed().as_millis() > 500 {
                    self.set_idle_state();
                };
                self.set_rot(self.rot() - Vec3::new(0.0, 0.1, 0.0));
                self.movement = self.walk_direction() * -1.15;
            }
            MobState::WalkBack(_t) => {
                if rng.gen_range(0..1000) == 0 {
                    self.set_idle_state();
                };
                self.movement = self.walk_direction() * -1.15;
            }
            MobState::TurnLeft(_t) => {
                if rng.gen_range(0..100) == 0 {
                    self.set_idle_state();
                };
                self.set_rot(self.rot() - Vec3::new(0.0, 0.1, 0.0));
                self.movement = Vec3::ZERO;
            }
            MobState::TurnRight(_t) => {
                if rng.gen_range(0..100) == 0 {
                    self.set_idle_state();
                };
                self.set_rot(self.rot() + Vec3::new(0.0, 0.1, 0.0));
                self.movement = Vec3::ZERO;
            }
            MobState::InstantAttackPlayer(t) => {
                if t.elapsed().as_millis() > 200 {
                    self.set_state(MobState::FightPlayer(Instant::now()));
                }
                self.movement = Vec3::ZERO;
            }
            MobState::FightPlayer(_t) | MobState::ChasePlayer(_t) => {
                let player_pos = player.pos();
                let diff = player_pos - self.pos();
                let distance = diff.length_squared();
                if player.is_dead() {
                    self.set_state(MobState::Dance(Instant::now()));
                } else if distance > MOB_STOP_FIGHTING_DISTANCE * MOB_STOP_FIGHTING_DISTANCE {
                    self.set_idle_state();
                } else {
                    let diff_2d = (player_pos - self.pos()).xz();
                    let deg = diff_2d.y.atan2(diff_2d.x).to_degrees();
                    let rot = Vec3::new(0.0, -deg - 90.0, 0.0);
                    self.turn_towards(rot);
                    self.movement = Vec3::ZERO;
                    if distance > 2.0 * 2.0 {
                        self.movement = self.walk_direction() * 1.2;
                        if let MobState::FightPlayer(_) = self.state {
                            self.set_state(MobState::ChasePlayer(Instant::now()));
                        }
                    } else if let MobState::ChasePlayer(_) = self.state {
                        if self.may_instant_attack() {
                            self.cooldown();
                            reactor.defer(Message::MobStrike {
                                pos: self.pos(),
                                damage: 1,
                            });
                            self.set_state(MobState::InstantAttackPlayer(Instant::now()));
                        } else {
                            self.set_state(MobState::FightPlayer(Instant::now()));
                        }
                    } else if let MobState::FightPlayer(t) = self.state {
                        if distance > 1.3 * 1.3 {
                            self.movement = self.walk_direction() * 1.2;
                        }
                        if t.elapsed().as_millis() > 600 {
                            reactor.defer(Message::MobStrike {
                                pos: self.pos(),
                                damage: 3,
                            });
                            self.set_state(MobState::FightPlayer(Instant::now()));
                        }
                    }
                }
            }
        };

        match self.state {
            MobState::Run(_) | MobState::ChasePlayer(_) => {
                if self.may_jump(world) {
                    let pos = self.pos() + Vec3::new(0.0, 1.0, 0.0);
                    if !self.would_collide_at(world, pos)
                        && self.vel.length_squared() < self.movement.length_squared() * 0.000002
                        && rng.gen_ratio(1, 50)
                    {
                        self.vel.y = 0.04;
                        let accel = 0.03;
                        self.vel.x = self.vel.x * (1.0 - accel) + (self.movement.x * accel);
                        self.vel.z = self.vel.z * (1.0 - accel) + (self.movement.z * accel);
                    }
                }
            }
            _ => (),
        }

        self.tick_physics(world);
    }

    fn draw(
        &self,
        frame: &mut glium::Frame,
        fe: &ClientState,
        meshes: &[VoxelMesh],
        view: &Mat4,
        projection: &Mat4,
        color_alpha: f32,
    ) -> Result<()> {
        let rot = self.rot();
        let pos = self.pos() + Vec3::new(0.0, -8.0 / 32.0, 0.0);
        let model = Mat4::from_scale(Vec3::new(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0));
        let model = Mat4::from_rotation_x(rot.x.to_radians()) * model;
        let model = Mat4::from_rotation_y(rot.y.to_radians()) * model;
        let model = Mat4::from_translation(pos) * model;
        let vp = projection.mul_mat4(view);
        let mvp = vp.mul_mat4(&model);

        meshes[self.anime_index()].draw(
            frame,
            fe.block_indeces(),
            &fe.shaders.voxel,
            &mvp,
            color_alpha,
        )
    }
}

#[derive(Clone, Default, Debug)]
pub struct MobList {
    mobs: Vec<Mob>,
}

impl MobList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.mobs.clear();
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
        player: &Character,
        world: &Chungus,
        rng: &mut XorShiftRng,
    ) {
        let player_pos = player.pos();
        self.mobs.retain_mut(|m| {
            m.tick(world, rng, player, reactor);
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
    let rng = Rc::new(RefCell::new(XorShiftRng::from_entropy()));
    {
        let player = args.game.player_rc();
        let world = args.game.world_rc();
        let rng = rng.clone();
        let f = move |reactor: &Reactor<Message>, _msg: Message| {
            let mut rng = rng.borrow_mut();
            MOBS.with(|mobs| {
                let player = player.borrow();
                let world = world.borrow();
                mobs.borrow_mut()
                    .tick_all(reactor, &player, &world, &mut rng);
            });
        };
        args.reactor
            .add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
    }
    {
        let f = move |reactor: &Reactor<Message>, msg: Message| {
            if let Message::CharacterAttack {
                char_pos,
                attack_pos,
                damage,
                ..
            } = msg
            {
                MOBS.with(|mobs| {
                    mobs.borrow_mut()
                        .iter_mut()
                        .filter(|m| (attack_pos - m.pos()).length_squared() < 2.0 * 2.0)
                        .for_each(|m| {
                            let mut dir = (char_pos - m.pos()).normalize();
                            dir.y -= 0.2;
                            m.set_vel(m.vel + dir * -0.01);
                            m.health -= damage;
                            m.set_state(MobState::ChasePlayer(Instant::now()));
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
        let f = move |reactor: &Reactor<Message>, msg: Message| {
            if let Message::Explosion { pos, power } = msg {
                let p = power * power;
                MOBS.with(|mobs| {
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
        let f = move |_reactor: &Reactor<Message>, msg: Message| {
            if let Message::WorldgenSpawnMob { pos, .. } = msg {
                let mut rng = rng.borrow_mut();
                MOBS.with(|mobs| {
                    let model_index = 0;
                    mobs.borrow_mut().add(
                        pos,
                        Vec3::new(0.0, rng.gen_range(0.0..360.0), 0.0),
                        model_index,
                    );
                });
            }
        };
        args.reactor
            .add_sink(Message::WorldgenSpawnMob { pos: Vec3::ZERO }, Box::new(f));
    }

    args.reactor.add_sink(
        Message::ResetEverything,
        Box::new(move |_: &Reactor<Message>, _msg: Message| {
            MOBS.with(|mobs| {
                mobs.borrow_mut().clear();
            });
        }),
    );

    args.render_reactor.entity_provider.push(Box::new(move |v| {
        MOBS.with(|mobs| {
            for e in mobs.borrow().iter() {
                v.push(e.to_entity());
            }
        });
    }));
    {
        let meshes: Vec<Vec<VoxelMesh>> =
            mob_load_meshes(&args.fe.display).expect("Error loading crab mesh");
        args.render_reactor
            .world_render
            .push(Box::new(move |args: RenderPassArgs| {
                let mvp = args.projection * args.view;
                let frustum = Frustum::extract(&mvp);
                MOBS.with(|mobs| {
                    for mob in mobs.borrow().iter() {
                        if frustum.contains_cube(mob.pos() - MOB_SIZE, MOB_SIZE * 2.0) {
                            let player_pos = args.game.player().pos();
                            let dist = (mob.pos() - player_pos).length();
                            let color_alpha =
                                ((args.render_distance - dist) / 32.0).clamp(0.0, 1.0);
                            let _ = mob.draw(
                                args.frame,
                                args.fe,
                                &meshes[mob.model_index() as usize],
                                &args.view,
                                &args.projection,
                                color_alpha,
                            );
                        }
                    }
                });
                args
            }));
    }
    args
}
