// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
#![forbid(unsafe_code)]
use wolkenwelten_client::RenderInitArgs;

mod item_drop;
pub use item_drop::{ItemDrop, ItemDropList};

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    let args = item_drop::init(args);
    args
}