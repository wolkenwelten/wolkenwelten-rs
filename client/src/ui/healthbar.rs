// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use wolkenwelten_core::GameState;

pub fn prepare(fe: &mut ClientState, game: &GameState, x: i16, y: i16, heart_beat: bool) {
    let health = game.player().health();
    let hp = health.health();
    let max_hp = health.max_health();
    let rem = hp % 4;

    if heart_beat && hp > 0 {
        let tick_rate = if hp < max_hp / 2 {
            if hp < 4 {
                2
            } else {
                3
            }
        } else {
            4
        };
        let ticks = 127 - ((fe.ticks() >> tick_rate) & 0x7F);
        let rgba: [u8; 4] = [255, 255, 255, (ticks << 1) as u8];
        let hb_off = 16 - (ticks as i16 >> 3);

        if rem == 0 {
            fe.ui_mesh.push_heart(
                x + hp / 4 * 40 - hb_off - 40,
                y - hb_off,
                32 + hb_off * 2,
                rgba,
                4,
            );
        } else {
            fe.ui_mesh.push_heart(
                x + hp / 4 * 40 - hb_off,
                y - hb_off,
                32 + hb_off * 2,
                rgba,
                rem,
            );
        }
    }

    for heart in 0..hp / 4 {
        fe.ui_mesh.push_heart(x + heart * 40, y, 32, [255; 4], 4);
    }

    if hp < max_hp {
        fe.ui_mesh.push_heart(x + hp / 4 * 40, y, 32, [255; 4], rem);
    }

    let rest = hp / 4 + 1;
    for dot in rest..max_hp / 4 {
        fe.ui_mesh.push_heart(x + dot * 40, y, 32, [255; 4], 0);
    }
}
