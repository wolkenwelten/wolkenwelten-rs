/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use super::Chungus;
use crate::GameEvent;
use crate::GameEvent::CharacterStomp;
use glam::{IVec3, Vec3};
use std::f32::consts::PI;

#[derive(Clone, Debug, Default)]
pub struct Character {
    pub pos: Vec3,
    pub rot: Vec3,
    pub vel: Vec3,

    pub no_clip: bool,
    pub cooldown: u64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum RaycastReturn {
    #[default]
    Within,
    Front,
}

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
    pub fn pos(&self) -> Vec3 {
        self.pos
    }
    pub fn rot(&self) -> Vec3 {
        self.rot
    }

    pub fn no_clip(&self) -> bool {
        self.no_clip
    }
    pub fn set_no_clip(&mut self, no_clip: bool) {
        self.no_clip = no_clip;
    }

    pub fn set_vel(&mut self, vel: &Vec3) {
        self.vel = *vel;
    }
    pub fn set_pos(&mut self, pos: &Vec3) {
        self.pos = *pos;
    }

    pub fn may_jump(&self, world: &Chungus) -> bool {
        world.is_solid(self.pos + COL_POINT_BOTTOM)
    }
    pub fn may_act(&self, now: u64) -> bool {
        self.cooldown < now
    }
    pub fn jump(&mut self) {
        self.vel.y = 0.055;
    }
    pub fn set_cooldown(&mut self, until: u64) {
        self.cooldown = until;
    }

    pub fn direction(&self) -> Vec3 {
        let a = self.rot;
        Vec3::new(
            ((a.x - 90.0) * PI / 180.0).cos() * (-a.y * PI / 180.0).cos(),
            (-a.y * PI / 180.0).sin(),
            ((a.x - 90.0) * PI / 180.0).sin() * (-a.y * PI / 180.0).cos(),
        )
    }

    pub fn tick(&mut self, events: &mut Vec<GameEvent>, world: &Chungus) {
        if self.no_clip {
            self.pos += self.vel;
            return;
        }

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

        if (old - self.vel).length() > 0.01 {
            events.push(CharacterStomp(self.pos));
        }

        let len = self.vel.length();
        if len > 0.2 {
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

        for _ in 0..512 {
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
