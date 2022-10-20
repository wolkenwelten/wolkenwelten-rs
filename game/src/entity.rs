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
use super::{Character, Chungus};
use glam::Vec3;

const ENTITY_SIZE: f32 = 0.6;
const ENTITY_BOUNCE_RATE: f32 = 0.6;

#[derive(Clone, Debug, Default)]
pub struct Entity {
    pub pos: Vec3,
    pub rot: Vec3,
    pub vel: Vec3,
}

impl Entity {
    pub fn new() -> Self {
        let pos = Vec3::ZERO;
        let rot = Vec3::ZERO;
        let vel = Vec3::ZERO;
        Self { pos, rot, vel }
    }
    pub fn pos(&self) -> Vec3 {
        self.pos
    }
    pub fn rot(&self) -> Vec3 {
        self.rot
    }
    pub fn vel(&self) -> Vec3 {
        self.vel
    }

    pub fn set_vel(&mut self, vel: Vec3) {
        self.vel = vel;
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn is_colliding(&self, world: &Chungus) -> bool {
        world.is_solid(self.pos() + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
            | world.is_solid(self.pos() + Vec3::new(ENTITY_SIZE, 0.0, 0.0))
            | world.is_solid(self.pos() + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
            | world.is_solid(self.pos() + Vec3::new(0.0, ENTITY_SIZE, 0.0))
            | world.is_solid(self.pos() + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
            | world.is_solid(self.pos() + Vec3::new(0.0, 0.0, ENTITY_SIZE))
    }

    pub fn tick(entities: &mut Vec<Self>, player: &Character, world: &Chungus) {
        for index in (0..entities.len()).rev() {
            if world.is_solid(entities[index].pos() + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
                | world.is_solid(entities[index].pos() + Vec3::new(-ENTITY_SIZE, 0.0, 0.0))
            {
                entities[index].vel.x *= -ENTITY_BOUNCE_RATE;
                entities[index].pos.x += entities[index].vel.x;
            }
            if world.is_solid(entities[index].pos() + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
                | world.is_solid(entities[index].pos() + Vec3::new(0.0, -ENTITY_SIZE, 0.0))
            {
                entities[index].vel.y *= -ENTITY_BOUNCE_RATE;
                entities[index].pos.y += entities[index].vel.y;
            }
            if world.is_solid(entities[index].pos() + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
                | world.is_solid(entities[index].pos() + Vec3::new(0.0, 0.0, -ENTITY_SIZE))
            {
                entities[index].vel.z *= -ENTITY_BOUNCE_RATE;
                entities[index].pos.z += entities[index].vel.z;
            }
            {
                let v = entities[index].vel;
                entities[index].pos += v;
                entities[index].vel.y -= 0.001;
                entities[index].rot.y += 0.05;
            }

            let dist = entities[index].pos - player.pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;

            if dd > (128.0 * 128.0) {
                entities.swap_remove(index); // Remove when far enough away
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity() {
        let mut e = Entity::new();
        assert_eq!(e.pos(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(e.rot(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(e.vel(), Vec3::new(0.0, 0.0, 0.0));
        e.set_pos(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.pos(), e.clone().pos());
        e.set_vel(Vec3::new(1.0, 1.0, 1.0));
    }
}
