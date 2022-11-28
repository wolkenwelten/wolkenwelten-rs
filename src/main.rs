// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
extern crate wolkenwelten_game;

use wolkenwelten_common::Reactor;
use wolkenwelten_game::GameState;
use wolkenwelten_scripting::start_runtime;

/// Here we just create a new GameState, optionally add the Sfx handler and
/// then start pass that along to the wolkenwelten-client-winit crate.
pub fn main() {
    let mut reactor = Reactor::new();
    #[cfg(feature = "sound")]
    {
        extern crate wolkenwelten_sound;
        wolkenwelten_sound::SfxList::add_handler(&mut reactor);
    }
    let game_state = GameState::new().expect("Couldn't initialize game backend");
    game_state.add_handler(&mut reactor);
    start_runtime(game_state, reactor);
}
