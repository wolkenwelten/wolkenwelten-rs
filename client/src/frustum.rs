// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.

// Most of this code is translated from Mark Morley's excellent Tutorials which can be found here:
// http://www.crownandcutlass.com/features/technicaldetails/frustum.html
use glam::f32::{Mat4, Vec3, Vec4};

/// Stores a frustum, to be used when frustum culling chunks
#[derive(Debug)]
pub struct Frustum {
    planes: [Vec4; 6],
}

impl Frustum {
    /// Extract a frustum from a view-projection matrix
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

    /// Determine whether `pos` is inside the frustum
    pub fn contains_point(&self, pos: Vec3) -> bool {
        let pos = pos.extend(1.0);
        self.planes.iter().all(|plane| plane.dot(pos) <= 0.0)
    }

    /// Determine whether a cube a `pos` of `size` is within the frustum
    pub fn contains_cube(&self, pos: Vec3, size: f32) -> bool {
        let pos = pos.extend(1.0);
        'planes: for plane in self.planes.iter() {
            for x in 0..=1 {
                for y in 0..=1 {
                    for z in 0..=1 {
                        let pos =
                            pos + Vec4::new(x as f32 * size, y as f32 * size, z as f32 * size, 0.0);
                        if plane.dot(pos + Vec4::new(0.0, 0.0, 0.0, 0.0)) > 0.0 {
                            continue 'planes;
                        }
                    }
                }
            }
            return false;
        }
        true
    }
}
