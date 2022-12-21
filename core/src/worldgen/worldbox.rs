// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct WorldBox {
    pub a: IVec3,
    pub b: IVec3,
}

impl WorldBox {
    pub fn new(a: IVec3, b: IVec3) -> Self {
        Self { a, b }
    }

    pub fn is_within(&self, other: &WorldBox) -> bool {
        other.contains(self)
    }

    pub fn contains(&self, other: &WorldBox) -> bool {
        self.a.x <= other.a.x
            && self.a.y <= other.a.y
            && self.a.z <= other.a.z
            && self.b.x >= other.b.x
            && self.b.y >= other.b.y
            && self.b.z >= other.b.z
    }

    pub fn contains_point(&self, p: IVec3) -> bool {
        self.a.x <= p.x
            && self.a.y <= p.y
            && self.a.z <= p.z
            && self.b.x >= p.x
            && self.b.y >= p.y
            && self.b.z >= p.z
    }

    pub fn intersects(&self, other: &WorldBox) -> bool {
        self.a.x <= other.b.x
            && self.b.x >= other.a.x
            && self.a.y <= other.b.y
            && self.b.y >= other.a.y
            && self.a.z <= other.b.z
            && self.b.z >= other.a.z
    }
}
