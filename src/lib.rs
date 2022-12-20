// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use wolkenwelten_client::RenderInit;

/// This function returns a list of the default plugins whose cargo feature is active
pub fn default_init_functions() -> Vec<RenderInit> {
    let mut render_init_fun: Vec<RenderInit> = vec![];
    #[cfg(feature = "sound")]
    {
        extern crate wolkenwelten_sound;
        render_init_fun.push(Box::new(wolkenwelten_sound::init));
    }

    #[cfg(feature = "sky")]
    {
        extern crate wolkenwelten_sky;
        render_init_fun.push(Box::new(wolkenwelten_sky::init));
    }

    #[cfg(feature = "shadow")]
    {
        extern crate wolkenwelten_shadow;
        render_init_fun.push(Box::new(wolkenwelten_shadow::init));
    }

    #[cfg(feature = "block-mining")]
    {
        extern crate wolkenwelten_block_mining;
        render_init_fun.push(Box::new(wolkenwelten_block_mining::init));
    }

    #[cfg(feature = "mob")]
    {
        extern crate wolkenwelten_mob;
        render_init_fun.push(Box::new(wolkenwelten_mob::init));
    }

    #[cfg(feature = "grenade")]
    {
        extern crate wolkenwelten_grenade;
        render_init_fun.push(Box::new(wolkenwelten_grenade::init));
    }

    #[cfg(feature = "item-drop")]
    {
        extern crate wolkenwelten_item_drop;
        render_init_fun.push(Box::new(wolkenwelten_item_drop::init));
    }

    #[cfg(feature = "particles")]
    {
        extern crate wolkenwelten_particles;
        render_init_fun.push(Box::new(wolkenwelten_particles::init));
    }

    #[cfg(feature = "worldgen")]
    {
        extern crate wolkenwelten_worldgen;
        render_init_fun.push(Box::new(wolkenwelten_worldgen::init));
    }
    render_init_fun
}
