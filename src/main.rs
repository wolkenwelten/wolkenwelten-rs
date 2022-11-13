// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
extern crate wolkenwelten_game;

use std::boxed::Box;
use wolkenwelten_client_winit::{start_app, MessageSink};
use wolkenwelten_common::Message;
use wolkenwelten_game::GameState;

/// Here we just create a new GameState, optionally add the Sfx handler and
/// then start pass that along to the wolkenwelten-client-winit crate.
pub fn main() {
    let message_sinks: Vec<MessageSink> = vec![
        #[cfg(feature = "sound")]
        {
            extern crate wolkenwelten_sound;
            let sfx = wolkenwelten_sound::SfxList::new();
            Box::new(move |msgs: &Vec<Message>| {
                sfx.msg_sink(msgs);
            })
        },
    ];
    let game_state = GameState::new().expect("Couldn't initialize game backend");
    start_app(game_state, message_sinks)
}
