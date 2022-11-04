// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Frustum;
use crate::{BlockMesh, FADE_DISTANCE, RENDER_DISTANCE, VIEW_STEPS};
use glam::{IVec3, Vec3};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use wolkenwelten_common::{CHUNK_BITS, CHUNK_SIZE};

#[derive(Debug, Default)]
pub struct QueueEntry {
    pub dist: i64,
    pub pos: IVec3,
    pub trans: Vec3,
    pub mask: u8,
    pub alpha: f32,
}

impl QueueEntry {
    fn new(pos: IVec3, trans: Vec3, dist: i64, mask: u8, alpha: f32) -> Self {
        Self {
            dist,
            pos,
            trans,
            mask,
            alpha,
        }
    }

    pub fn build(player_pos: Vec3, frustum: &Frustum) -> BinaryHeap<Self> {
        let mut render_queue: BinaryHeap<Self> = BinaryHeap::with_capacity(512);
        let px = (player_pos.x.floor() as i32) >> CHUNK_BITS;
        let py = (player_pos.y.floor() as i32) >> CHUNK_BITS;
        let pz = (player_pos.z.floor() as i32) >> CHUNK_BITS;

        for cx in -VIEW_STEPS..=VIEW_STEPS {
            for cy in -VIEW_STEPS..=VIEW_STEPS {
                for cz in -VIEW_STEPS..=VIEW_STEPS {
                    let x = px + cx;
                    let y = py + cy;
                    let z = pz + cz;
                    let trans_x = x as f32 * CHUNK_SIZE as f32;
                    let trans_y = y as f32 * CHUNK_SIZE as f32;
                    let trans_z = z as f32 * CHUNK_SIZE as f32;
                    if !frustum
                        .contains_cube(Vec3::new(trans_x, trans_y, trans_z), CHUNK_SIZE as f32)
                    {
                        continue;
                    } else {
                        let trans = Vec3::new(trans_x, trans_y, trans_z);
                        let dist = trans - player_pos;
                        let dist = dist.length();
                        if dist < RENDER_DISTANCE {
                            let alpha = if dist < (RENDER_DISTANCE - FADE_DISTANCE) {
                                1.0
                            } else {
                                1.0 - ((dist - (RENDER_DISTANCE - FADE_DISTANCE)) / FADE_DISTANCE)
                            };
                            let dist = (dist * 8192.0) as i64;
                            let pos = IVec3::new(x, y, z);
                            let mask = BlockMesh::calc_mask(cx, cy, cz);
                            render_queue.push(Self::new(pos, trans, dist, mask, alpha));
                        }
                    }
                }
            }
        }
        render_queue
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        //self.dist.cmp(&other.dist)
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueueEntry {}
impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
