// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use glam::IVec3;
use wolkenwelten_core::{ChunkRequestQueue, GameState, WorldBox};

pub fn prepare(fe: &mut ClientState, game: &GameState, request: &ChunkRequestQueue) {
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

    let y = y + 20;

    let position = {
        let player = game.player();
        WorldBox {
            a: player.pos().as_ivec3() - IVec3::new(1, 1, 1),
            b: player.pos().as_ivec3() + IVec3::new(1, 1, 1),
        }
    };
    let outlines = game.world().get_outlines(position);
    let y = y + 20;
    for (i, outline) in outlines.iter().enumerate() {
        let y = y + (10 * i as i16);
        let text = format!("{}", outline);
        fe.ui_mesh
            .push_string(8, y, 1, [0xFF, 0xFF, 0xFF, 0xFF], text.as_str());
    }
}
