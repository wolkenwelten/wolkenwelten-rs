// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
extern crate wolkenwelten_game;

use std::boxed::Box;
use wolkenwelten_client_winit::{start_app, MessageSink};
use wolkenwelten_common::Message;
use wolkenwelten_game::GameState;

pub fn main() {
    let game_state = GameState::new().expect("Couldn't initialize game backend");
    let message_sinks: Vec<MessageSink> = vec![
        #[cfg(feature = "sound")]
        {
            extern crate wolkenwelten_sound;

            let sfx = wolkenwelten_sound::SfxList::new();
            let λ = move |msgs: &Vec<Message>| {
                sfx.msg_sink(msgs);
            };
            Box::new(λ)
        },
    ];
    start_app(game_state, message_sinks)
}
