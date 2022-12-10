// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{BlockItem, Chungus, Experience, GameState, Health, Item, Message, Reactor};
use glam::{IVec3, Vec3, Vec3Swizzles};
use std::{f32::consts::PI, time::Instant};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum CharacterAnimation {
    #[default]
    None,
    Hit(Instant),
}

#[derive(Clone, Debug, Default)]
pub struct Character {
    pub pos: Vec3,
    pub rot: Vec3,
    pub vel: Vec3,

    movement: Vec3,
    mining: Option<(IVec3, u8)>,

    no_clip: bool,
    cooldown: u64,
    mining_cooldown: u64,
    health: Health,
    experience: Experience,

    inventory_active: usize,
    inventory: Vec<Item>,

    animation: CharacterAnimation,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum RaycastReturn {
    #[default]
    Within,
    Front,
}

const CHARACTER_ACCELERATION: f32 = 0.01;
const CHARACTER_STOP_RATE: f32 = CHARACTER_ACCELERATION * 3.0;

const COL_WIDTH: f32 = 0.4;
const COL_DEPTH: f32 = 0.4;

const COL_POINT_TOP: Vec3 = Vec3::new(0.0, 0.7, 0.0);
const COL_POINT_BOTTOM: Vec3 = Vec3::new(0.0, -1.7, 0.0);
const COL_POINT_LEFT: Vec3 = Vec3::new(-COL_WIDTH, -1.2, 0.0);
const COL_POINT_RIGHT: Vec3 = Vec3::new(COL_WIDTH, -1.2, 0.0);
const COL_POINT_FRONT: Vec3 = Vec3::new(0.0, -1.2, COL_DEPTH);
const COL_POINT_BACK: Vec3 = Vec3::new(0.0, -1.2, -COL_DEPTH);

impl Character {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rebirth(&mut self) {
        self.set_pos(Vec3::new(-32.0, -16.0, 338.0));
        self.set_rot(Vec3::new(-130.0, 0.0, 0.0));
        let inv = self.inventory_mut();
        inv.clear();
        inv.resize(10, Item::None);
        self.set_inventory_active(0);
        self.health.set_max_health(12);
        self.health.set_full_health();
        self.experience_mut().reset();
        self.experience_mut().gain(8);
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
    pub fn movement(&self) -> Vec3 {
        self.movement
    }

    #[inline]
    pub fn experience(&self) -> &Experience {
        &self.experience
    }
    #[inline]
    pub fn experience_mut(&mut self) -> &mut Experience {
        &mut self.experience
    }

    #[inline]
    pub fn mining(&self) -> Option<(IVec3, u8)> {
        self.mining
    }
    #[inline]
    pub fn set_mining(&mut self, m: Option<(IVec3, u8)>) {
        self.mining = m;
    }

    #[inline]
    pub fn no_clip(&self) -> bool {
        self.no_clip
    }
    #[inline]
    pub fn set_no_clip(&mut self, no_clip: bool) {
        self.no_clip = no_clip;
    }

    #[inline]
    pub fn set_vel(&mut self, vel: Vec3) {
        self.vel = vel;
    }
    #[inline]
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    #[inline]
    pub fn set_rot(&mut self, rot: Vec3) {
        self.rot = rot;
    }
    #[inline]
    pub fn set_movement(&mut self, v: Vec3) {
        self.movement = v;
    }

    #[inline]
    pub fn may_jump(&self, world: &Chungus) -> bool {
        world.is_solid(self.pos + COL_POINT_BOTTOM)
    }
    #[inline]
    pub fn may_act(&self, now: u64) -> bool {
        self.cooldown < now
    }
    #[inline]
    pub fn may_mine(&self, now: u64) -> bool {
        self.mining_cooldown < now
    }
    #[inline]
    pub fn jump(&mut self) {
        self.vel.y = 0.038;
    }
    #[inline]
    pub fn set_cooldown(&mut self, until: u64) {
        self.cooldown = until;
    }

    #[inline]
    pub fn health(&self) -> Health {
        self.health
    }

    #[inline]
    pub fn set_max_health(&mut self, amount: i16) {
        self.health.set_max_health(amount);
        self.health.set_full_health();
    }

    #[inline]
    pub fn item(&self) -> Item {
        self.inventory[self.inventory_active]
    }

    #[inline]
    pub fn item_at(&self, pos: usize) -> Item {
        self.inventory[pos]
    }

    #[inline]
    pub fn inventory(&self) -> &Vec<Item> {
        &self.inventory
    }

    #[inline]
    pub fn inventory_mut(&mut self) -> &mut Vec<Item> {
        &mut self.inventory
    }

    #[inline]
    pub fn inventory_active(&self) -> usize {
        self.inventory_active
    }

    #[inline]
    pub fn set_inventory_active(&mut self, v: usize) {
        self.inventory_active = v.clamp(0, self.inventory.len());
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.health.is_dead()
    }

    #[inline]
    pub fn animation(&self) -> CharacterAnimation {
        self.animation
    }

    #[inline]
    pub fn set_animation_none(&mut self) {
        self.animation = CharacterAnimation::None;
    }

    pub fn set_animation_hit(&mut self) {
        self.animation = CharacterAnimation::Hit(Instant::now());
    }

    pub fn check_animation(&mut self) {
        if let CharacterAnimation::Hit(t) = self.animation {
            if t.elapsed().as_millis() > 500 {
                self.set_animation_none();
            }
        }
    }

    pub fn direction(&self) -> Vec3 {
        let a = self.rot;
        Vec3::new(
            ((a.x - 90.0) * PI / 180.0).cos() * (-a.y * PI / 180.0).cos(),
            (-a.y * PI / 180.0).sin(),
            ((a.x - 90.0) * PI / 180.0).sin() * (-a.y * PI / 180.0).cos(),
        )
    }

    pub fn walk_direction(&self) -> Vec3 {
        let a = self.rot;
        Vec3::new(
            ((a.x - 90.0) * PI / 180.0).cos(),
            0.0,
            ((a.x - 90.0) * PI / 180.0).sin(),
        )
    }

    pub fn switch_selection(&mut self, delta: i32) {
        if delta < 0 {
            self.inventory_active = (self.inventory_active + 1) % self.inventory.len();
        } else {
            self.inventory_active = self.inventory_active.wrapping_sub(1);
            if self.inventory_active > self.inventory().len() {
                self.inventory_active = self.inventory.len() - 1;
            }
        }
    }

    pub fn remove_block_from_inventory(&mut self, block: u8) {
        for item in self.inventory_mut().iter_mut() {
            if let Item::Block(bi) = item {
                if bi.block != block {
                    continue;
                }
                bi.amount -= 1;
                if bi.amount == 0 {
                    *item = Item::None;
                }
                return;
            }
        }
    }

    pub fn add_block_to_inventory(&mut self, block: u8) {
        for item in self.inventory_mut().iter_mut() {
            if let Item::Block(bi) = item {
                if bi.block != block {
                    continue;
                }
                if bi.amount >= 99 {
                    continue;
                }
                bi.amount += 1;
                return;
            }
        }
        for item in self.inventory_mut().iter_mut() {
            if let Item::None = item {
                *item = Item::Block(BlockItem::new(block, 1));
                return;
            }
        }
    }

    pub fn drop_item(&mut self, pos: usize) -> Item {
        let item = self.item_at(pos);
        if let Item::Block(bi) = item {
            if bi.amount > 1 {
                let item = BlockItem::new(bi.block, 1).into();
                self.inventory[pos] = BlockItem::new(bi.block, bi.amount - 1).into();
                return item;
            }
        }
        self.inventory[pos] = Item::None;
        item
    }

    pub fn wrap_rot(&mut self) {
        if self.rot[0] < 0.0 {
            self.rot[0] += 360.0;
        }
        if self.rot[0] > 360.0 {
            self.rot[0] -= 360.0;
        }

        if self.rot[1] < -90.0 {
            self.rot[1] = -90.0;
        }
        if self.rot[1] > 90.0 {
            self.rot[1] = 90.0;
        }
    }

    fn is_solid_pillar(&self, pos: Vec3, world: &Chungus) -> bool {
        world.is_solid(pos)
            || world.is_solid(pos + Vec3::new(0.0, -0.4, 0.0))
            || world.is_solid(pos + Vec3::new(0.0, 0.8, 0.0))
    }

    fn is_underwater_point(world: &Chungus, pos: Vec3) -> bool {
        if let Some(fluid) = world.get_fluid_block(pos.as_ivec3()) {
            fluid != 0
        } else {
            false
        }
    }

    pub fn is_underwater(&self, world: &Chungus) -> bool {
        Self::is_underwater_point(world, self.pos() + Vec3::new(0.0, -0.8, 0.0))
    }

    pub fn may_swim(&self, world: &Chungus) -> bool {
        Self::is_underwater_point(world, self.pos() + Vec3::new(0.0, -1.2, 0.0))
    }

    pub fn mining_cooldown(&self) -> u64 {
        self.mining_cooldown
    }

    pub fn set_mining_cooldown(&mut self, mc: u64) {
        self.mining_cooldown = mc;
    }

    pub fn tick(&mut self, reactor: &Reactor<Message>, world: &Chungus, cur_tick: u64) {
        if self.no_clip {
            self.pos += self.vel;
            return;
        }

        if !world.is_loaded(self.pos) {
            return; // Just freeze the character until we have loaded the area, this shouldn't happen if at all possible
        }
        let underwater = self.is_underwater(world);

        let accel = if self.movement.xz().length() > 0.01 {
            CHARACTER_ACCELERATION
        } else {
            CHARACTER_STOP_RATE
        };
        let accel = if self.may_jump(world) {
            accel
        } else {
            accel * 0.4 // Slow down player movement changes during jumps
        };
        let accel = if underwater { accel * 0.7 } else { accel };

        self.vel.x = self.vel.x * (1.0 - accel) + (self.movement.x * 0.02) * accel;
        self.vel.z = self.vel.z * (1.0 - accel) + (self.movement.z * 0.02) * accel;

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
        if force > 0.01 {
            reactor.dispatch(Message::CharacterStomp { pos: self.pos });
        }
        if force > 0.05 {
            let amount = (force * 14.0) as i16;
            if amount > 0 {
                let damage = amount * amount;
                self.health.damage(damage);
                if self.health().is_dead() {
                    reactor.dispatch(Message::CharacterDeath { pos: self.pos });
                    self.rebirth();
                } else {
                    reactor.dispatch(Message::CharacterDamage {
                        pos: self.pos,
                        damage,
                    });
                }
            }
        }

        let len = self.vel.length();
        if len > 0.5 {
            self.vel *= 1.0 - (len - 0.2).clamp(0.0001, 1.0);
        }

        if self.may_jump(world) && (len > 0.01 && cur_tick & 0x7F == 0) {
            reactor.dispatch(Message::CharacterStep { pos: self.pos });
        }

        self.pos += self.vel;
    }

    pub fn raycast(&self, world: &Chungus, return_value: RaycastReturn) -> Option<IVec3> {
        let dir = self.direction() * 0.0625;
        let mut pos = self.pos();
        let mut i_pos = pos.floor().as_ivec3();
        if world.is_solid_i(i_pos) {
            return Some(i_pos);
        }

        for _ in 0..64 {
            let n_pos = pos + dir;
            let c_pos = pos.floor().as_ivec3();

            if (i_pos != c_pos) && world.is_solid_i(c_pos) {
                return Some(if return_value == RaycastReturn::Front {
                    i_pos
                } else {
                    c_pos
                });
            }
            i_pos = c_pos;
            pos = n_pos;
        }
        None
    }

    pub fn add_handler(reactor: &mut Reactor<Message>, game: &GameState) {
        {
            let player = game.player_rc();
            let clock = game.clock_rc();
            let f = move |reactor: &Reactor<Message>, _: Message| {
                let now = clock.borrow().elapsed().as_millis() as u64;
                let msg = {
                    let mut player = player.borrow_mut();
                    if player.may_act(now) {
                        let attack_pos = player.pos() + player.direction();
                        player.set_animation_hit();
                        player.set_cooldown(now + 400);
                        Message::CharacterAttack {
                            char_pos: player.pos(),
                            attack_pos,
                            damage: 1,
                        }
                    } else {
                        return;
                    }
                };
                let replies = reactor.dispatch_with_answer(msg);
                if !replies.is_empty() {
                    let mut player = player.borrow_mut();
                    player.set_mining_cooldown(now + 400)
                }
            };
            reactor.add_sink(Message::PlayerStrike, Box::new(f));
        }
        {
            let player = game.player_rc();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::PlayerSwitchSelection { delta, .. } = msg {
                    player.borrow_mut().switch_selection(delta);
                }
            };
            reactor.add_sink(Message::PlayerSwitchSelection { delta: 0 }, Box::new(f));
        }
        {
            let player = game.player_rc();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::PlayerSelect { i, .. } = msg {
                    player
                        .borrow_mut()
                        .set_inventory_active(i.try_into().unwrap());
                }
            };
            reactor.add_sink(Message::PlayerSelect { i: 0 }, Box::new(f));
        }
        {
            let player = game.player_rc();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::PlayerNoClip { no_clip, .. } = msg {
                    player.borrow_mut().set_no_clip(no_clip);
                }
            };
            reactor.add_sink(Message::PlayerNoClip { no_clip: false }, Box::new(f));
        }
        {
            let player = game.player_rc();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::PlayerTurn { direction, .. } = msg {
                    let mut player = player.borrow_mut();
                    player.rot += direction;
                    player.wrap_rot();
                }
            };
            reactor.add_sink(
                Message::PlayerTurn {
                    direction: Vec3::ZERO,
                },
                Box::new(f),
            );
        }
        {
            let player = game.player_rc();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::PlayerFly { direction, .. } = msg {
                    player.borrow_mut().vel = direction * 0.15;
                }
            };
            reactor.add_sink(
                Message::PlayerFly {
                    direction: Vec3::ZERO,
                },
                Box::new(f),
            );
        }
        {
            let player = game.player_rc();
            let f = move |_reactor: &Reactor<Message>, msg: Message| {
                if let Message::ItemDropPickup {
                    item: Item::Block(bi),
                    ..
                } = msg
                {
                    for _ in 0..bi.amount {
                        player.borrow_mut().add_block_to_inventory(bi.block);
                    }
                }
            };
            reactor.add_sink(
                Message::ItemDropPickup {
                    pos: Vec3::ZERO,
                    item: Item::None,
                },
                Box::new(f),
            );
        }
        {
            let player = game.player_rc();
            let f = move |_reactor: &Reactor<Message>, _msg: Message| {
                player.borrow_mut().check_animation();
            };
            reactor.add_sink(
                Message::DrawFrame {
                    player_pos: Vec3::ZERO,
                    ticks: 0,
                    render_distance: 0.0,
                },
                Box::new(f),
            );
        }
        {
            let player = game.player_rc();
            let world = game.world_rc();
            let clock = game.clock_rc();
            let f = move |reactor: &Reactor<Message>, msg: Message| {
                if let Message::PlayerMove { direction, .. } = msg {
                    let mut player = player.borrow_mut();
                    player.set_movement(direction);
                    let world = &world.borrow();
                    if direction.y > 0.0 && (player.may_jump(world) || player.may_swim(world)) {
                        let now = clock.borrow().elapsed().as_millis() as u64;
                        player.set_cooldown(now + 200);
                        player.jump();
                        reactor.dispatch(Message::CharacterJump { pos: player.pos });
                    }
                }
            };
            reactor.add_sink(
                Message::PlayerMove {
                    direction: Vec3::ZERO,
                },
                Box::new(f),
            );
        }
        {
            let player = game.player_rc();
            let world = game.world_rc();
            let clock = game.clock_rc();
            let f = move |reactor: &Reactor<Message>, msg: Message| {
                if let Message::PlayerBlockPlace { pos, .. } = msg {
                    let mut player = player.borrow_mut();
                    let now = clock.borrow().elapsed().as_millis() as u64;
                    if player.may_act(now) {
                        let mut world = world.borrow_mut();
                        if world.get_block(pos).unwrap_or(0) == 0 {
                            if let Item::Block(bi) = player.item() {
                                player.set_animation_hit();
                                player.set_cooldown(now + 300);
                                let b = bi.block;
                                world.set_block(pos, b);
                                player.remove_block_from_inventory(b);
                                reactor.dispatch(Message::BlockPlace { pos, block: b });
                            }
                        }
                    }
                }
            };
            reactor.add_sink(Message::PlayerBlockPlace { pos: IVec3::ZERO }, Box::new(f));
        }
        {
            let player = game.player_rc();
            let clock = game.clock_rc();
            let world = game.world_rc();
            let f = move |_reactor: &Reactor<Message>, msg: Message| {
                if let Message::PlayerBlockMine { pos, .. } = msg {
                    let now = clock.borrow().elapsed().as_millis() as u64;
                    if player.borrow().may_mine(now) {
                        if let Some(pos) = pos {
                            if let Some(b) = world.borrow_mut().get_block(pos) {
                                let mut player = player.borrow_mut();
                                player.set_mining(Some((pos, b)));
                                if player.may_act(now) {
                                    player.set_animation_hit();
                                    player.set_cooldown(now + 300);
                                }
                                return;
                            }
                        }
                    }
                    player.borrow_mut().set_mining(None);
                }
            };
            reactor.add_sink(Message::PlayerBlockMine { pos: None }, Box::new(f));
        }
        {
            let player = game.player_rc();
            let clock = game.clock_rc();
            let f = move |reactor: &Reactor<Message>, _msg: Message| {
                let mut player = player.borrow_mut();
                let now = clock.borrow().elapsed().as_millis() as u64;
                if player.may_act(now) {
                    let pos = player.inventory_active();
                    let item = player.drop_item(pos);
                    if item != Item::None {
                        player.set_animation_hit();
                        player.set_cooldown(now + 100);
                        let vel = player.direction();
                        let pos = player.pos() + vel * 2.0;
                        let vel = vel * 0.03;
                        reactor.defer(Message::CharacterDropItem { pos, vel, item });
                    }
                }
            };
            reactor.add_sink(Message::PlayerDropItem, Box::new(f));
        }
        {
            let player = game.player_rc();
            let world = game.world_rc();
            let f = move |reactor: &Reactor<Message>, msg: Message| {
                if let Message::GameTick { ticks } = msg {
                    player.borrow_mut().tick(reactor, &world.borrow(), ticks);
                    let player = player.borrow();
                    let msg = Message::CharacterPosRotVel {
                        pos: player.pos(),
                        rot: player.rot(),
                        vel: player.vel(),
                    };
                    reactor.dispatch(msg);
                }
            };
            reactor.add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
        }
        {
            let player = game.player_rc();
            let f = move |reactor: &Reactor<Message>, msg: Message| {
                if let Message::CharacterGainExperience { xp, .. } = msg {
                    let mut player = player.borrow_mut();
                    let experience = player.experience_mut();
                    experience.gain(xp);
                    if experience.level_up() {
                        let level = experience.level();
                        player.set_max_health(level as i16 * 4 + 8);
                        reactor.defer(Message::CharacterLevelUp {
                            pos: player.pos(),
                            level,
                        })
                    }
                }
            };
            reactor.add_sink(
                Message::CharacterGainExperience {
                    pos: Vec3::ZERO,
                    xp: 0,
                },
                Box::new(f),
            );
        }
    }
}
