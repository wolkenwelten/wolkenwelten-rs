// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Chungus, Health};
use glam::{IVec3, Vec3, Vec3Swizzles};
use std::{f32::consts::PI, time::Instant};
use wolkenwelten_common::{BlockItem, GameEvent, Item, Message};

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

    no_clip: bool,
    cooldown: u64,
    health: Health,

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
    pub fn may_jump(&self, world: &Chungus) -> bool {
        world.is_solid(self.pos + COL_POINT_BOTTOM)
    }
    #[inline]
    pub fn may_act(&self, now: u64) -> bool {
        self.cooldown < now
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
                if bi.amount <= 0 {
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
        match item {
            Item::Block(bi) => {
                if bi.amount > 1 {
                    let item = BlockItem::new(bi.block, 1).into();
                    self.inventory[pos] = BlockItem::new(bi.block, bi.amount - 1).into();
                    return item;
                }
            }
            _ => (),
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

    pub fn tick(&mut self, v: Vec3, events: &mut Vec<Message>, world: &Chungus, cur_tick: u64) {
        if self.no_clip {
            self.pos += self.vel;
            return;
        }

        if !world.is_loaded(self.pos) {
            return; // Just freeze the character until we have loaded the area, this shouldn't happen if at all possible
        }

        let accel = if v.xz().length() > 0.01 {
            CHARACTER_ACCELERATION
        } else {
            CHARACTER_STOP_RATE
        };
        let accel = if self.may_jump(world) {
            accel
        } else {
            accel * 0.4 // Slow down player movement changes during jumps
        };

        self.vel.x = self.vel.x * (1.0 - accel) + (v.x * 0.02) * accel;
        self.vel.z = self.vel.z * (1.0 - accel) + (v.z * 0.02) * accel;

        self.vel.y -= 0.0005;
        let old = self.vel;

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
            events.push(GameEvent::CharacterStomp(self.pos).into());
        }
        if force > 0.05 {
            let amount = (force * 14.0) as i16;
            if amount > 0 {
                self.health.damage(amount * amount);
                if self.health().is_dead() {
                    events.push(GameEvent::CharacterDeath(self.pos).into());
                } else {
                    events.push(GameEvent::CharacterDamage(self.pos, amount).into());
                }
            }
        }

        let len = self.vel.length();
        if len > 0.5 {
            self.vel *= 1.0 - (len - 0.2).clamp(0.0001, 1.0);
        }

        if self.may_jump(world) {
            if len > 0.025 && cur_tick & 0x3F == 0 {
                events.push(GameEvent::CharacterStep(self.pos).into());
            } else if len > 0.01 && cur_tick & 0x7F == 0 {
                events.push(GameEvent::CharacterStep(self.pos).into());
            }
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
}
