// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use anyhow::Result;
use glam::Mat4;
use glium::{uniform, Surface};
use wolkenwelten_common::{ChunkRequestQueue, CHUNK_SIZE};
use wolkenwelten_game::{Character, GameState};

mod block_mining;
mod chungus;
mod grenade;
mod held_item;
mod item_drop;
mod sky;

pub const RENDER_DISTANCE: f32 = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
    156.0
} else if cfg!(debug_assertions) {
    192.0
} else {
    256.0
};
pub const FADE_DISTANCE: f32 = 32.0;

/// Calculate the FoV based on player veolicty
fn calc_fov(fov: f32, player: &Character) -> f32 {
    let new = 90.0 + ((player.vel.length() - 0.025) * 40.0).clamp(0.0, 90.0);
    (fov + (new - fov) / 32.0).clamp(90.0, 170.0)
}

/// Prepare the ClientState for rendering, since it is mutable we can create new, persistent,
/// buffers here. Like for example BlockMeshes.
pub fn prepare_frame(
    fe: &mut ClientState,
    game: &GameState,
    request: &mut ChunkRequestQueue,
) -> Result<()> {
    fe.set_fov(calc_fov(fe.fov(), game.player()));
    fe.calc_fps();
    fe.gc(game.player());
    super::ui::prepare(fe, game, &request);
    chungus::handle_requests(fe, game, request)?;
    Ok(())
}

/// Render the game view to `frame`
fn render_game(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
    projection: &Mat4,
    request: &mut ChunkRequestQueue,
) -> Result<()> {
    let view = Mat4::from_rotation_x(game.player().rot[1].to_radians());
    let view = view * Mat4::from_rotation_y(game.player().rot[0].to_radians());
    sky::draw(frame, fe, &view, projection)?;

    let view = view * Mat4::from_translation(-game.player().pos);
    let mvp = *projection * view;
    {
        let particles = fe.particles();
        let particles = particles.borrow();
        particles.draw(frame, &fe.display, &fe.shaders.particle, &mvp)?;
    }

    for entity in game.grenades.iter() {
        grenade::draw(frame, fe, entity, &view, &projection)?;
    }
    for entity in game.drops().iter() {
        item_drop::draw(frame, fe, entity, &view, &projection)?;
    }
    chungus::draw(frame, fe, game, &mvp, request)?;

    block_mining::draw(frame, fe, game, &mvp)?;

    Ok(())
}

/// Render the UI layered on top of the game view
fn render_ui(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
    projection: &Mat4,
) -> Result<()> {
    frame.clear_depth(4095.0);
    held_item::draw(frame, fe, game, projection)?;

    let (window_width, window_height) = fe.window_size();
    let projection = Mat4::orthographic_rh_gl(
        0.0,
        window_width as f32,
        window_height as f32,
        0.0,
        -10.0,
        10.0,
    );
    let mat_mvp = projection.to_cols_array_2d();
    let cur_tex = fe.textures.gui.texture();
    frame.draw(
        fe.ui_mesh.buffer(),
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &fe.shaders.text,
        &uniform! {
            mat_mvp: mat_mvp,
            cur_tex: cur_tex,
        },
        &glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::draw_parameters::Blend {
                color: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::One,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            ..Default::default()
        },
    )?;
    Ok(())
}

/// Render an entire frame, including the game view, as well as the UI.
/// Everything needs to be prepared beforehand, since only the frame itself is mutable.
/// If you need to build a persistent buffer somewhere, you need to do so during the `prepare_frame` call.
pub fn render_frame(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
    request: &mut ChunkRequestQueue,
) -> Result<()> {
    let (window_width, window_height) = fe.window_size();
    let projection = Mat4::perspective_rh_gl(
        fe.fov().to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
    );

    frame.clear(
        None,
        Some((0.32, 0.63, 0.96, 1.0)),
        true,
        Some(65535.0),
        None,
    );
    render_game(frame, fe, game, &projection, request)?;
    render_ui(frame, fe, game, &projection)?;
    Ok(())
}
