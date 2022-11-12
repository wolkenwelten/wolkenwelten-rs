// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;
use wolkenwelten_common::{ChunkBlockData, CHUNK_SIZE};

pub trait WorldgenVegetation {
    fn wg_shrub(&mut self, pos: IVec3);
}

impl WorldgenVegetation for ChunkBlockData {
    fn wg_shrub(&mut self, pos: IVec3) {
        if pos.x < 1 {
            return;
        }
        if pos.z < 1 {
            return;
        }
        if pos.x > CHUNK_SIZE as i32 - 2 {
            return;
        }
        if pos.z > CHUNK_SIZE as i32 - 2 {
            return;
        }
        if pos.y < 0 {
            return;
        }
        if pos.y > CHUNK_SIZE as i32 - 5 {
            return;
        }
        self.set_block(5, pos.into());
        self.set_block(5, IVec3::new(pos.x, pos.y + 1, pos.z));
        self.set_block(5, IVec3::new(pos.x, pos.y + 2, pos.z));
        self.set_block(6, IVec3::new(pos.x, pos.y + 3, pos.z));
        self.set_block(6, IVec3::new(pos.x, pos.y + 4, pos.z));
        self.set_block(6, IVec3::new(pos.x + 1, pos.y + 2, pos.z));
        self.set_block(6, IVec3::new(pos.x - 1, pos.y + 2, pos.z));
        self.set_block(6, IVec3::new(pos.x + 1, pos.y + 3, pos.z));
        self.set_block(6, IVec3::new(pos.x - 1, pos.y + 3, pos.z));
        self.set_block(6, IVec3::new(pos.x, pos.y + 2, pos.z + 1));
        self.set_block(6, IVec3::new(pos.x, pos.y + 2, pos.z - 1));
        self.set_block(6, IVec3::new(pos.x, pos.y + 3, pos.z + 1));
        self.set_block(6, IVec3::new(pos.x, pos.y + 3, pos.z - 1));
    }
}
