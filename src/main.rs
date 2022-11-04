// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
extern crate wolkenwelten_client;
extern crate wolkenwelten_game;
extern crate wolkenwelten_scripting;
extern crate wolkenwelten_sound;

use wolkenwelten_client::ClientState;
use wolkenwelten_game::GameState;
use wolkenwelten_scripting::Runtime;
use wolkenwelten_sound::SfxList;

mod lib;

pub fn main() {
    let (event_loop, display) = lib::init();
    let render_state = ClientState::new(display).expect("Can't create ClientState");

    // And after having set up everything we can start up the event loop
    lib::run_event_loop(lib::AppState {
        game_state: GameState::new(),
        render_state,
        event_loop,
        runtime: Runtime::new(),
        sfx: SfxList::new(),
    })
}
