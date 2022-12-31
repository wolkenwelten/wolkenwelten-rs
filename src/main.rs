// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
#![forbid(unsafe_code)]

// Right now we just start the slayer_mode immediatly, in the
// future we will instead start a menu_mode instead.
pub fn main() {
    wolkenwelten_slayer_mode::start();
}
