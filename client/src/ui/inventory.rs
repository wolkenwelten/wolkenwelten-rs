// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use glam::IVec2;
use wolkenwelten_common::Item;
use wolkenwelten_game::GameState;

fn prepare_slot(
    fe: &mut ClientState,
    _game: &GameState,
    pos: IVec2,
    size: IVec2,
    active: bool,
    item: Item,
) {
    let p = (pos.x as i16, pos.y as i16, size.x as i16, size.y as i16);
    let tex = (220, 252, 4, 4);
    let rgba = [0xFF, 0xFF, 0xFF, 0x7F];
    fe.ui_mesh.push_box(p, tex, rgba);

    let tex = (if active { 212 } else { 208 }, 252, 4, 4);
    let rgba = [0xFF, 0xFF, 0xFF, 0xFF];
    fe.ui_mesh.push_box(p, tex, rgba);

    match item {
        Item::Block(bi) => {
            let p = (
                (pos.x + size.x / 16) as i16,
                (pos.y + size.x / 16) as i16,
                (size.x - size.x / 8) as i16,
                (size.y - size.y / 8) as i16,
            );
            let tex = (
                ((bi.block % 32) * 4) as i16,
                ((bi.block / 32) * 4) as i16,
                4,
                4,
            );
            let rgba = [0xFF, 0xFF, 0xFF, 0xFF];
            fe.ui_mesh.push_box(p, tex, rgba);

            let text = format!("{}x", bi.amount);
            fe.ui_mesh.push_string(
                (pos.x + 4) as i16,
                (pos.y + size.y - 12) as i16,
                1,
                rgba,
                text.as_str(),
            );
        }
        _ => (),
    }
}

pub fn prepare(fe: &mut ClientState, game: &GameState) {
    let (window_width, window_height) = fe.window_size();
    let player = game.player();

    let active_i = player.inventory_active();
    let inv = player.inventory();
    let x = window_width as i32 - inv.len() as i32 * 64;
    let y = (window_height - 64) as i32;
    let size = IVec2::new(64, 64);
    for (i, item) in inv.iter().enumerate() {
        let pos = IVec2::new(x + i as i32 * 64, y);
        prepare_slot(fe, game, pos, size, i == active_i, *item);
    }
}
