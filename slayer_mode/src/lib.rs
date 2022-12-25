// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use wolkenwelten_client::RenderInit;
use wolkenwelten_core::{GameState, Reactor};
use wolkenwelten_scripting::{push_init_code, start_runtime};

/// This function returns a list of the default plugins whose cargo feature is active
pub fn add_init_functions() -> Vec<RenderInit> {
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
    push_init_code(include_str!("./main.js"));

    render_init_fun
}

/// Here we just create a new GameState, optionally add the Sfx handler and
/// then start pass that along to the wolkenwelten-client-winit crate.
pub fn start() {
    // First we create a new reactor, this part is responsible for dispatching messages and events
    // from one part of the game to another without them having to know of each other
    let mut reactor = Reactor::new();

    // Now we need to initialize a new game state, as the name implies everything gameplayer related will be stored here.
    let game_state = GameState::new().with_handler(&mut reactor);

    // Here we generate a Vec of all the plugins that should be enabled during initialization
    let init_functions = add_init_functions();
    // You could add your own functions here

    // Finally we pass everything along to the scripting system, which initializes itself and then passes everything along
    // to the client crate which finally opens a window
    start_runtime(game_state, reactor, init_functions);
}
