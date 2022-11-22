// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct BlockItem {
    pub block: u8,
    pub amount: u16,
}

impl From<BlockItem> for Item {
    fn from(i: BlockItem) -> Self {
        Item::Block(i)
    }
}

impl BlockItem {
    pub fn new(block: u8, amount: u16) -> Self {
        Self { block, amount }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub enum Item {
    #[default]
    None,
    Block(BlockItem),
}
