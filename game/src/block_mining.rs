// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default)]
pub struct BlockMining {
    pub block: u8,
    pub damage: u16,
    pub block_health: u16,
}

#[derive(Debug, Default)]
pub struct BlockMiningMap {
    map: HashMap<IVec3, BlockMining>,
}

impl BlockMiningMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.map.retain(|_, p| {
            p.damage -= 1;
            p.damage > 0
        });
    }

    pub fn mine(&mut self, pos: IVec3, block: u8, dmg: u16, block_health: u16) -> bool {
        if let Some(m) = self.map.get_mut(&pos) {
            m.damage += dmg;
            if m.damage > m.block_health {
                m.damage = 1;
                return true;
            }
        } else {
            self.map.insert(
                pos,
                BlockMining {
                    damage: 2,
                    block,
                    block_health,
                },
            );
        }
        false
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<IVec3, BlockMining> {
        self.map.iter()
    }
}
