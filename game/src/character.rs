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
use glam::Vec3;

#[derive(Clone, Debug, Default)]
pub struct Character {
    pub pos: Vec3,
    pub rot: Vec3,
    pub vel: Vec3,

    pub no_clip: bool,
}

const COL_POINT_TOP: Vec3 = Vec3::new(0.5, 1.0, 0.0);
const COL_POINT_BOTTOM: Vec3 = Vec3::new(0.0, -2.5, 0.0);
const COL_POINT_LEFT: Vec3 = Vec3::new(-0.5, -2.0, 0.0);
const COL_POINT_RIGHT: Vec3 = Vec3::new(0.5, -2.0, 0.0);
const COL_POINT_FRONT: Vec3 = Vec3::new(-0.5, -2.0, 1.0);
const COL_POINT_BACK: Vec3 = Vec3::new(0.5, -2.0, -1.0);

impl Character {
    pub fn new() -> Self {
        let pos = Vec3::ZERO;
        let rot = Vec3::ZERO;
        let vel = Vec3::ZERO;
        Self {
            pos,
            rot,
            vel,
            no_clip: false,
        }
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

    pub fn tick(&mut self, world: &Chungus) {
        if self.no_clip {
            self.pos += self.vel;
            return;
        }

        self.vel.y -= 0.003;

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

        self.pos += self.vel;
    }
}
