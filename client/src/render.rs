// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{meshes::TextMesh, ClientState};
use anyhow::Result;
use glam::Mat4;
use glium::{texture::SrgbTexture2d, uniform, uniforms::Sampler, Surface};
use wolkenwelten_core::{Character, ChunkRequestQueue, GameState, CHUNK_SIZE};

pub mod chungus;
mod held_item;

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

fn prepare_overlay(fe: &mut ClientState, game: &GameState) {
    let overlay_goal_color = if game.player().is_underwater(&game.world()) {
        [0, 24, 242, 178].into()
    } else {
        [0; 4].into()
    };
    fe.set_overlay_color(overlay_goal_color);
}

/// Prepare the ClientState for rendering, since it is mutable we can create new, persistent,
/// buffers here. Like for example BlockMeshes.
pub fn prepare_frame(
    fe: &mut ClientState,
    game: &GameState,
    request: &mut ChunkRequestQueue,
) -> Result<()> {
    fe.set_fov(calc_fov(fe.fov(), &game.player()));
    fe.calc_fps();
    fe.gc(&game.player());
    super::ui::prepare(fe, game, request);
    chungus::handle_requests(fe, game, request)?;
    prepare_overlay(fe, game);
    Ok(())
}

fn render_overlay(
    frame: &mut glium::Frame,
    fe: &ClientState,
    mat_mvp: [[f32; 4]; 4],
    cur_tex: Sampler<SrgbTexture2d>,
) -> Result<()> {
    let color = fe.overlay_color();
    if color.a == 0 {
        return Ok(());
    }
    let (window_width, window_height) = fe.window_size();

    let mut mesh = TextMesh::new(&fe.display)?;
    let p = (0, 0, window_width as i16, window_height as i16);
    let tex = (76, 124, 4, 4);
    mesh.push_box(p, tex, color.into());
    mesh.prepare(&fe.display);

    frame.draw(
        mesh.buffer(),
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
pub fn render_frame(frame: &mut glium::Frame, fe: &ClientState, game: &GameState) -> Result<()> {
    let (window_width, window_height) = fe.window_size();
    let projection = Mat4::perspective_rh_gl(
        fe.fov().to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
    );

    frame.clear_depth(4095.0);
    held_item::draw(frame, fe, game, &projection)?;

    let projection = Mat4::orthographic_rh_gl(
        0.0,
        window_width as f32,
        window_height as f32,
        0.0,
        -10.0,
        10.0,
    );
    let mat_mvp = projection.to_cols_array_2d();
    let cur_tex = fe.textures.gui.texture_nn();
    render_overlay(frame, fe, mat_mvp, cur_tex)?;

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
