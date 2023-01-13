use std::{cell::RefCell, collections::HashMap};

// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use serde::{Deserialize, Serialize};

thread_local! {
    pub static SCRIPTED_ITEMS: RefCell<ScriptedItemList> = RefCell::new(ScriptedItemList::new());
}

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

#[derive(Default, Debug)]
pub struct ScriptedItemList {
    pub items: HashMap<u32, ScriptedItem>,
}

impl ScriptedItemList {
    pub fn new() -> ScriptedItemList {
        Default::default()
    }

    pub fn get(id:u32) -> Option<ScriptedItem> {
        SCRIPTED_ITEMS.with(|l| {
            l.borrow().items.get(&id).map(|v| v.clone())
        })
    }

    pub fn get_icon(id:u32) -> Option<u16> {
        SCRIPTED_ITEMS.with(|l| {
            let l = l.borrow();
            l.items.get(&id).map(|si| si.icon)
        })
    }

    pub fn get_mesh(id:u32) -> Option<u16> {
        SCRIPTED_ITEMS.with(|l| {
            let l = l.borrow();
            l.items.get(&id).map(|si| si.mesh)
        })
    }

    pub fn get_amount(id:u32) -> Option<u16> {
        SCRIPTED_ITEMS.with(|l| {
            let l = l.borrow();
            l.items.get(&id).map(|si| si.amount)
        })
    }

    pub fn set_icon(id:u32, icon:u16) {
        SCRIPTED_ITEMS.with(|l| {
            let mut l = l.borrow_mut();
            if let Some(si) = l.items.get_mut(&id) {
                si.icon = icon;
            } else {
                let si = ScriptedItem::new(id, icon, 0, 0);
                l.items.insert(id, si);
            }
        });
    }

    pub fn set_mesh(id:u32, mesh:u16) {
        SCRIPTED_ITEMS.with(|l| {
            let mut l = l.borrow_mut();
            if let Some(si) = l.items.get_mut(&id) {
                si.mesh = mesh;
            } else {
                let si = ScriptedItem::new(id, 0, mesh, 0);
                l.items.insert(id, si);
            }
        });
    }

    pub fn set_amount(id:u32, amount:u16) {
        SCRIPTED_ITEMS.with(|l| {
            let mut l = l.borrow_mut();
            if let Some(si) = l.items.get_mut(&id) {
                si.amount = amount;
            } else {
                let si = ScriptedItem::new(id, 0, 0, amount);
                l.items.insert(id, si);
            }
        });
    }

    pub fn delete(id:u32) {
        SCRIPTED_ITEMS.with(|l| {
            let mut l = l.borrow_mut();
            if let Some(si) = l.items.get_mut(&id) {
                si.delete = true;
            }
        });
    }
}

#[derive(Clone, Default, Debug)]
pub struct ScriptedItem {
    pub id: u32,
    pub icon: u16,
    pub mesh: u16,
    pub amount: u16,
    pub delete: bool,
}

impl From<&ScriptedItem> for Item {
    fn from(i: &ScriptedItem) -> Self {
        Item::Scripted(i.id)
    }
}

impl ScriptedItem {
    pub fn new(id: u32, icon: u16, mesh: u16, amount: u16) -> Self {
        Self {
            id,
            icon,
            mesh,
            amount,
            delete: false,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    #[default]
    None,
    Block(BlockItem),
    Scripted(u32),
}
