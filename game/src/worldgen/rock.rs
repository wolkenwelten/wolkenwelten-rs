// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;
use wolkenwelten_common::{ChunkBlockData, CHUNK_SIZE};

pub trait WorldgenRocks {
    fn wg_rock(&mut self, pos: IVec3);
}

impl WorldgenRocks for ChunkBlockData {
    fn wg_rock(&mut self, pos: IVec3) {
        if pos.x < 5 {
            return;
        }
        if pos.z < 5 {
            return;
        }
        if pos.x > CHUNK_SIZE as i32 - 5 {
            return;
        }
        if pos.z > CHUNK_SIZE as i32 - 5 {
            return;
        }
        if pos.y < 2 {
            return;
        }
        if pos.y > CHUNK_SIZE as i32 - 12 {
            return;
        }

        self.set_sphere(3, IVec3::new(pos.x, pos.y + 2, pos.z), 3);
    }
}
