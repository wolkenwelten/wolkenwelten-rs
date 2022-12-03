// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use wolkenwelten_client::RenderInit;
use wolkenwelten_core::{GameState, Reactor};
use wolkenwelten_scripting::start_runtime;

/// Here we just create a new GameState, optionally add the Sfx handler and
/// then start pass that along to the wolkenwelten-client-winit crate.
pub fn main() {
    let mut reactor = Reactor::new();
    let mut render_init_fun: Vec<RenderInit> = vec![];

    #[cfg(feature = "sound")]
    {
        extern crate wolkenwelten_sound;
        render_init_fun.push(Box::new(wolkenwelten_sound::init));
    }

    #[cfg(feature = "block-mining")]
    {
        extern crate wolkenwelten_block_mining;
        render_init_fun.push(Box::new(wolkenwelten_block_mining::init));
    }

    #[cfg(feature = "sky")]
    {
        extern crate wolkenwelten_sky;
        render_init_fun.push(Box::new(wolkenwelten_sky::init));
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

    #[cfg(feature = "shadow")]
    {
        extern crate wolkenwelten_shadow;
        render_init_fun.push(Box::new(wolkenwelten_shadow::init));
    }

    #[cfg(feature = "particles")]
    {
        extern crate wolkenwelten_particles;
        render_init_fun.push(Box::new(wolkenwelten_particles::init));
    }

    let game_state = GameState::new().expect("Couldn't initialize game backend");
    game_state.add_handler(&mut reactor);
    start_runtime(game_state, reactor, render_init_fun);
}
