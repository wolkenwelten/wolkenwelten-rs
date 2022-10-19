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

/*
 * Most of this code is translated from Mark Morley's Tutorials which can be found here:
 * http://www.crownandcutlass.com/features/technicaldetails/frustum.html
 */
use glam::f32::{Mat4, Vec3, Vec4};

#[derive(Debug)]
pub struct Frustum {
    planes: [Vec4; 6],
}

impl Frustum {
    pub fn extract(clip: &Mat4) -> Frustum {
        Frustum {
            planes: [
                clip.row(3) - clip.row(0), // Right
                clip.row(3) + clip.row(0), // Left
                clip.row(3) - clip.row(1), // Top
                clip.row(3) + clip.row(1), // Bottom
                clip.row(3) - clip.row(2), // Far
                clip.row(3) + clip.row(2), // Near
            ],
        }
    }

    pub fn contains_point(&self, pos: Vec3) -> bool {
        let pos = pos.extend(1.0);
        for p in 0..6 {
            if self.planes[p].dot(pos) <= 0.0 {
                return false;
            }
        }
        true
    }

    pub fn contains_cube(&self, pos: Vec3, size: f32) -> bool {
        let pos = pos.extend(1.0);
        for i in 0..6 {
            if self.planes[i].dot(pos + Vec4::new(0.0, 0.0, 0.0, 0.0)) > 0.0 {
                continue;
            }
            if self.planes[i].dot(pos + Vec4::new(size, 0.0, 0.0, 0.0)) > 0.0 {
                continue;
            }
            if self.planes[i].dot(pos + Vec4::new(0.0, size, 0.0, 0.0)) > 0.0 {
                continue;
            }
            if self.planes[i].dot(pos + Vec4::new(size, size, 0.0, 0.0)) > 0.0 {
                continue;
            }
            if self.planes[i].dot(pos + Vec4::new(0.0, 0.0, size, 0.0)) > 0.0 {
                continue;
            }
            if self.planes[i].dot(pos + Vec4::new(size, 0.0, size, 0.0)) > 0.0 {
                continue;
            }
            if self.planes[i].dot(pos + Vec4::new(0.0, size, size, 0.0)) > 0.0 {
                continue;
            }
            if self.planes[i].dot(pos + Vec4::new(size, size, size, 0.0)) > 0.0 {
                continue;
            }
            return false;
        }
        true
    }
}
