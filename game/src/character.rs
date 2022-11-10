// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Chungus, Health};
use glam::{IVec3, Vec3, Vec3Swizzles};
use std::f32::consts::PI;
use wolkenwelten_common::{GameEvent, Message};

#[derive(Clone, Debug, Default)]
pub struct Character {
    pub pos: Vec3,
    pub rot: Vec3,
    pub vel: Vec3,

    no_clip: bool,
    cooldown: u64,
    block_selection: u8,
    health: Health,
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
    pub fn is_dead(&self) -> bool {
        self.health.is_dead()
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

    pub fn switch_block_selection(&mut self, delta: i32) {
        let sel = (self.block_selection as i32 + if delta > 0 { 1 } else { -1 }) as u8 % 16;
        self.block_selection = sel;
    }

    #[inline]
    pub fn block_selection(&self) -> u8 {
        self.block_selection + 1
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

    pub fn tick(&mut self, v: Vec3, events: &mut Vec<Message>, world: &Chungus) {
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

        if world.is_solid(self.pos + COL_POINT_LEFT) {
            self.vel.x = self.vel.x.max(0.0);
        }
        if world.is_solid(self.pos + COL_POINT_RIGHT) {
            self.vel.x = self.vel.x.min(0.0);
        }

        if world.is_solid(self.pos + COL_POINT_BOTTOM) {
            self.vel.y = self.vel.y.max(0.0);
        }
        if world.is_solid(self.pos + COL_POINT_TOP) {
            self.vel.y = self.vel.y.min(0.0);
        }

        if world.is_solid(self.pos + COL_POINT_FRONT) {
            self.vel.z = self.vel.z.min(0.0);
        }
        if world.is_solid(self.pos + COL_POINT_BACK) {
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
