// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use wolkenwelten_core::{ChunkRequestQueue, GameState};

mod debug;
mod healthbar;
mod inventory;
mod log;

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

fn prepare_death_overlay(fe: &mut ClientState, game: &GameState) {
    if !game.player().is_dead() {
        return;
    }
    let (window_width, window_height) = fe.window_size();

    let x = window_width as i16 / 2 - 128;
    let y = window_height as i16 / 2 - 16;
    let rgba = [0xFF, 0xFF, 0xFF, 0xFF];
    let text = "You died";
    fe.ui_mesh.push_string(x, y, 4, rgba, text);
    let x = x - 128;
    let y = y + 48;
    let text = format!(
        "You reached level {} with a score of {}",
        game.player().experience().level(),
        game.score()
    );
    fe.ui_mesh.push_string(x, y, 2, rgba, &text);

    let x = x + 48;
    let y = y + 56;
    fe.ui_mesh
        .push_string(x, y, 2, rgba, "Press R to try once more");
}

pub fn prepare(fe: &mut ClientState, game: &GameState, request: &ChunkRequestQueue) {
    prepare_fps(fe);
    prepare_crosshair(fe);
    healthbar::prepare(fe, game, 96, 16, true);
    prepare_experience(fe, game, 16, 16, 64);
    prepare_death_overlay(fe, game);
    debug::prepare(fe, game, request);
    log::prepare(fe);
    inventory::prepare(fe, game);
    fe.ui_mesh.prepare(&fe.display);
}
