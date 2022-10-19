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
