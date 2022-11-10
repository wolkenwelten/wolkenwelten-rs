// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use anyhow::Result;
use glam::Mat4;
use glium::{uniform, Surface};
use wolkenwelten_common::CHUNK_SIZE;
use wolkenwelten_game::{Character, GameState};

mod chungus;
mod entity;
mod held_item;
mod sky;

pub const RENDER_DISTANCE: f32 = if cfg!(debug_assertions) { 192.0 } else { 256.0 };
pub const FADE_DISTANCE: f32 = 32.0;

fn calc_fov(fov: f32, player: &Character) -> f32 {
    let new = 90.0 + ((player.vel.length() - 0.025) * 40.0).clamp(0.0, 90.0);
    (fov + (new - fov) / 32.0).clamp(90.0, 170.0)
}

pub fn prepare_frame(fe: &mut ClientState, game: &GameState) -> Result<()> {
    super::ui::prepare(fe, game);
    fe.calc_fps();
    fe.gc(game.player());
    fe.set_fov(calc_fov(fe.fov(), game.player()));
    chungus::prepare(fe, game)?;
    Ok(())
}

fn render_game(frame: &mut glium::Frame, fe: &ClientState, game: &GameState) -> Result<()> {
    let (window_width, window_height) = fe.window_size();
    let projection = Mat4::perspective_rh_gl(
        fe.fov().to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
    );
    let view = Mat4::from_rotation_x(game.player().rot[1].to_radians());
    let view = view * Mat4::from_rotation_y(game.player().rot[0].to_radians());
    sky::draw(frame, fe, view, projection)?;

    let view = view * Mat4::from_translation(-game.player().pos);
    let mvp = projection * view;
    {
        let particles = fe.particles();
        let particles = particles.borrow();
        particles.draw(frame, &fe.display, &fe.shaders.particle, &mvp)?;
    }

    for entity in game.entities.iter() {
        entity::draw(frame, fe, entity, &view, &projection)?;
    }
    chungus::draw(frame, fe, game, &mvp)?;

    frame.clear(None, None, true, Some(4000.0), None);
    held_item::draw(frame, fe, &projection)
}

pub fn render_frame(frame: &mut glium::Frame, fe: &ClientState, game: &GameState) -> Result<()> {
    frame.clear(
        None,
        Some((0.32, 0.63, 0.96, 1.0)),
        true,
        Some(4000.0),
        None,
    );

    render_game(frame, fe, game)?;

    let (window_width, window_height) = fe.window_size();
    let perspective = Mat4::orthographic_rh_gl(
        0.0,
        window_width as f32,
        window_height as f32,
        0.0,
        -10.0,
        10.0,
    );
    let mat_mvp = perspective.to_cols_array_2d();
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
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..Default::default()
        },
    )?;
    Ok(())
}
