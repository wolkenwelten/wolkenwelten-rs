// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use wolkenwelten_core::{ChunkRequestQueue, GameState};

mod inventory;
mod log;

fn prepare_healthbar(fe: &mut ClientState, game: &GameState, x: i16, y: i16, heart_beat: bool) {
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

fn prepare_experience(fe: &mut ClientState, game: &GameState, x: i16, y: i16, size: i16) {
    {
        let percent = game.player().experience().percent_till_level_up();
        let off = (size as f32 * percent) as i16;
        let p = (x, y + size - off, size, off);
        let tex = (76, 124, 4, 4);
        let rgba = [0xFF, 0x9F, 0x0A, 0xCF];
        fe.ui_mesh.push_box(p, tex, rgba);
    }
    let p = (x, y, size, size);
    let tex = (80, 124, 4, 4);
    let rgba = [0xFF, 0xFF, 0xFF, 0xFF];
    fe.ui_mesh.push_box(p, tex, rgba);

    let xp_text = format!("{}", game.player().experience().level());
    fe.ui_mesh.push_string(
        x + size / 2 - 12,
        y + size / 2 - 16,
        4,
        [0xFF, 0xFF, 0xFF, 0xFF],
        xp_text.as_str(),
    );

    let xp_text = format!(
        "{}/{}",
        game.player().experience().xp(),
        game.player().experience().next_level()
    );
    if fe.show_debug_info() {
        fe.ui_mesh.push_string(
            x + 4,
            y + size + 4,
            1,
            [0xFF, 0xFF, 0xFF, 0xFF],
            xp_text.as_str(),
        );
    }
}

fn prepare_fps(fe: &mut ClientState) {
    let (window_width, _window_height) = fe.window_size();
    let fps_text = format!("{}", fe.fps());
    fe.ui_mesh.push_string(
        window_width as i16 - 48,
        8,
        2,
        [0xFF, 0xFF, 0xFF, 0xFF],
        fps_text.as_str(),
    );
}

fn prepare_debug_text(fe: &mut ClientState, game: &GameState, request: &ChunkRequestQueue) {
    if !fe.show_debug_info() {
        return;
    }
    let pos = game.player().pos();
    let pos_text = format!(
        "X:{:8.2} Y:{:8.2} Z:{:8.2}   Ticks:{}",
        pos[0], pos[1], pos[2], game.ticks_elapsed
    );
    let y = 96;
    fe.ui_mesh
        .push_string(8, y, 2, [0xFF, 0xFF, 0xFF, 0xFF], pos_text.as_str());

    let col_text = format!(
        "Count: (Chunks:{}, BlockMeshes:{})",
        game.world().chunk_count(),
        fe.world_mesh.len(),
    );
    let y = y + 20;
    fe.ui_mesh
        .push_string(8, y, 2, [0xFF, 0xFF, 0xFF, 0xFF], col_text.as_str());
    let text = format!(
        "Requests: (Block:{}, Light:(Simple:{} / Complex:{}), Mesh:{}, Fluid:{})",
        request.block_len(),
        request.simple_light_len(),
        request.complex_light_len(),
        request.mesh_len(),
        request.fluid_len(),
    );
    let y = y + 20;
    fe.ui_mesh
        .push_string(8, y, 2, [0xFF, 0xFF, 0xFF, 0xFF], text.as_str());
}

fn prepare_crosshair(fe: &mut ClientState) {
    let (window_width, window_height) = fe.window_size();

    let pos = (
        window_width as i16 / 2 - 32,
        window_height as i16 / 2 - 32,
        32,
        32,
    );
    let tex = (72, 124, 4, 4);
    fe.ui_mesh.push_box(pos, tex, [0xFF, 0xFF, 0xFF, 0x7F]);
}

pub fn prepare(fe: &mut ClientState, game: &GameState, request: &ChunkRequestQueue) {
    prepare_fps(fe);
    prepare_crosshair(fe);
    prepare_healthbar(fe, game, 96, 16, true);
    prepare_experience(fe, game, 16, 16, 64);

    prepare_debug_text(fe, game, request);
    inventory::prepare(fe, game);
    log::prepare(fe);

    fe.ui_mesh.prepare(&fe.display);
}
