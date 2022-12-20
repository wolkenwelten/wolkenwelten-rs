// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use wolkenwelten::default_init_functions;
use wolkenwelten_core::{GameState, Reactor};
use wolkenwelten_scripting::start_runtime;

/// Here we just create a new GameState, optionally add the Sfx handler and
/// then start pass that along to the wolkenwelten-client-winit crate.
pub fn main() {
    // First we create a new reactor, this part is responsible for dispatching messages and events
    // from one part of the game to another without them having to know of each other
    let mut reactor = Reactor::new();

    // Now we need to initialize a new game state, as the name implies everything gameplayer related will be stored here.
    let game_state = GameState::new().with_handler(&mut reactor);

    // Here we generate a Vec of all the plugins that should be enabled during initialization
    let init_functions = default_init_functions();
    // You could add your own functions here

    // Finally we pass everything along to the scripting system, which initializes itself and then passes everything along
    // to the client crate which finally opens a window
    start_runtime(game_state, reactor, init_functions);
}
