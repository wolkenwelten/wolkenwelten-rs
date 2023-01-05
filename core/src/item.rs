// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScriptedItem {
    pub id: u32,
    pub icon: u16,
    pub mesh: u16,
    pub amount: u16,
}

impl From<ScriptedItem> for Item {
    fn from(i: ScriptedItem) -> Self {
        Item::Scripted(i)
    }
}

impl ScriptedItem {
    pub fn new(id: u32, icon: u16, mesh: u16, amount: u16) -> Self {
        Self {
            id,
            icon,
            mesh,
            amount,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    #[default]
    None,
    Block(BlockItem),
    Scripted(ScriptedItem),
}
