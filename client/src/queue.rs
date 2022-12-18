// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Frustum;
use crate::{BlockMesh, RENDER_DISTANCE};
use glam::{IVec3, Vec3};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use wolkenwelten_core::{CHUNK_BITS, CHUNK_SIZE};

/// An entry in our chunk draw queue. This is mainly used for
/// sorting all chunks back-to-front so that the alpha blending
/// works correctly. We also store the overall transparency
/// of the chunk, as well as which sides are to be drawn here.
/// That way we can keep the actual render function quite simple,
/// and just do the draw calls there.
#[derive(Debug, Default)]
pub struct QueueEntry {
    pub dist: i64,
    pub pos: IVec3,
    pub trans: Vec3,
    pub mask: u8,
}

impl QueueEntry {
    fn new(pos: IVec3, trans: Vec3, dist: i64, mask: u8) -> Self {
        Self {
            dist,
            pos,
            trans,
            mask,
        }
    }

    /// Build a new draw queue, based on the players position and the pre-extracted
    /// frustum. We also cull all the chunks that can't be visible from the players PoV here.
    pub fn build(player_pos: Vec3, frustum: &Frustum) -> BinaryHeap<Self> {
        let mut render_queue: BinaryHeap<Self> = BinaryHeap::with_capacity(512);
        let px = (player_pos.x.floor() as i32) >> CHUNK_BITS;
        let py = (player_pos.y.floor() as i32) >> CHUNK_BITS;
        let pz = (player_pos.z.floor() as i32) >> CHUNK_BITS;
        let view_steps = (RENDER_DISTANCE / CHUNK_SIZE as f32).ceil() as i32;

        for cx in -view_steps..=view_steps {
            for cy in -view_steps..=view_steps {
                for cz in -view_steps..=view_steps {
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
                        let dist = trans
                            + Vec3::new(
                                CHUNK_SIZE as f32 / 2.0,
                                CHUNK_SIZE as f32 / 2.0,
                                CHUNK_SIZE as f32 / 2.0,
                            )
                            - player_pos;
                        let dist = dist.length();
                        if dist < RENDER_DISTANCE + CHUNK_SIZE as f32 {
                            let dist = (dist * 8192.0) as i64;
                            let pos = IVec3::new(x, y, z);
                            let mask = BlockMesh::calc_mask(cx, cy, cz);
                            render_queue.push(Self::new(pos, trans, dist, mask));
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
