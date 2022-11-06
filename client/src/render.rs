// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::meshes::BlockMesh;
use super::Frustum;
use crate::{ClientState, QueueEntry};
use glam::{
    f32::{Mat4, Vec3},
    IVec3,
};
use glium::{uniform, DrawError};
use std::time::Instant;
use wolkenwelten_common::{CHUNK_BITS, CHUNK_SIZE};
use wolkenwelten_game::{Character, Entity, GameState};

use glium::Surface;

pub const RENDER_DISTANCE: f32 = 192.0;
pub const FADE_DISTANCE: f32 = 32.0;
const FADEOUT_DISTANCE: f32 = FADE_DISTANCE * FADE_DISTANCE;
const FADEOUT_START_DISTANCE: f32 = (RENDER_DISTANCE - 32.0) * (RENDER_DISTANCE - 32.0);

fn calc_fov(fov: f32, player: &Character) -> f32 {
    let new = 90.0 + ((player.vel.length() - 0.025) * 40.0).clamp(0.0, 90.0);
    (fov + (new - fov) / 32.0).clamp(90.0, 170.0)
}

fn draw_entity(
    frame: &mut glium::Frame,
    fe: &ClientState,
    entity: &Entity,
    view: &Mat4,
    projection: &Mat4,
) -> Result<(), DrawError> {
    let rot = entity.rot();
    let pos = entity.pos();

    let model = Mat4::from_scale(Vec3::new(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0));
    let model = Mat4::from_rotation_x((rot.x - 90.0).to_radians()) * model;
    let model = Mat4::from_rotation_y(rot.y.to_radians()) * model;
    let model = Mat4::from_translation(pos) * model;
    let vp = projection.mul_mat4(view);
    let mvp = vp.mul_mat4(&model);
    fe.meshes
        .grenade
        .draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
}

fn render_held_item(
    frame: &mut glium::Frame,
    fe: &ClientState,
    projection: &Mat4,
) -> Result<(), DrawError> {
    let t = (fe.ticks() as f32 / 512.0).sin();
    let model = Mat4::from_scale(Vec3::new(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0));
    let model = Mat4::from_rotation_x((-90.0 + t * 6.0).to_radians()) * model;
    let model = Mat4::from_rotation_y((-10.0_ + t * 1.0).to_radians()) * model;
    let pos = Vec3::new(1.0, -0.5 + t * 0.05, -1.0);
    let model = Mat4::from_translation(pos) * model;
    let mvp = projection.mul_mat4(&model);

    fe.meshes
        .grenade
        .draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
}

pub fn prepare_chunk(
    fe: &mut ClientState,
    game: &GameState,
    pos: IVec3,
    now: Instant,
) -> Result<(), glium::vertex::BufferCreationError> {
    if let Some(chunk) = game.world.get_chunk(&pos) {
        if let Some(mesh) = fe.world_mesh.get_mut(&pos) {
            if chunk.get_light().get_last_updated() >= mesh.get_last_updated() {
                mesh.update(
                    &fe.display,
                    chunk.get_block(),
                    chunk.get_light(),
                    &game.world.blocks,
                    now,
                )?;
            }
        } else {
            let mut mesh = BlockMesh::new(&fe.display)?;
            mesh.update(
                &fe.display,
                chunk.get_block(),
                chunk.get_light(),
                &game.world.blocks,
                now,
            )?;
            fe.world_mesh.insert(pos, mesh);
        }
    }
    Ok(())
}

fn prepare_chunks(
    fe: &mut ClientState,
    game: &GameState,
) -> Result<(), glium::vertex::BufferCreationError> {
    let now = Instant::now();
    let player = game.player();
    let px = (player.pos.x as i32) >> CHUNK_BITS;
    let py = (player.pos.y as i32) >> CHUNK_BITS;
    let pz = (player.pos.z as i32) >> CHUNK_BITS;
    let view_steps = (RENDER_DISTANCE as i32 / CHUNK_SIZE as i32) + 1;
    for cx in -view_steps..=view_steps {
        for cy in -view_steps..=view_steps {
            for cz in -view_steps..=view_steps {
                let pos = IVec3::new(cx + px, cy + py, cz + pz);
                let d = (pos.as_vec3() * CHUNK_SIZE as f32) - player.pos;
                let d = d.dot(d);
                if d < FADEOUT_DISTANCE + FADEOUT_START_DISTANCE {
                    prepare_chunk(fe, game, pos, now)?;
                }
            }
        }
    }
    Ok(())
}

pub fn prepare_frame(
    fe: &mut ClientState,
    game: &GameState,
) -> Result<(), glium::vertex::BufferCreationError> {
    super::ui::prepare(fe, game);
    fe.calc_fps();
    fe.gc(game.player());
    fe.set_fov(calc_fov(fe.fov(), game.player()));
    prepare_chunks(fe, game)?;
    fe.particles.update(game.player().pos, RENDER_DISTANCE);
    Ok(())
}

fn render_chungus(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
    mvp: &Mat4,
) -> Result<(), DrawError> {
    let frustum = Frustum::extract(mvp);
    let render_queue = QueueEntry::build(game.player().pos, &frustum);
    let now = Instant::now();
    let mat_mvp = mvp.to_cols_array_2d();

    let cur_tex = fe
        .textures
        .blocks
        .texture()
        .sampled()
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
        .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat);

    for entry in render_queue.iter() {
        if let Some(mesh) = fe.world_mesh.get(&entry.pos) {
            let td = (now - mesh.get_first_created()).as_millis();
            let fade_in = (td as f32 / 500.0).clamp(0.0, 1.0);
            let alpha = entry.alpha * fade_in;
            let mask = entry.mask;
            let trans_pos = [entry.trans.x, entry.trans.y, entry.trans.z];
            let uniforms = uniform! {
                color_alpha: alpha,
                mat_mvp: mat_mvp,
                trans_pos: trans_pos,
                cur_tex: cur_tex,
            };
            let draw_parameters = glium::DrawParameters {
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                blend: glium::draw_parameters::Blend::alpha_blending(),
                depth: glium::draw_parameters::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            if mask == 0b111111 {
                frame.draw(
                    mesh.buffer(),
                    fe.block_indeces(),
                    &fe.shaders.block,
                    &uniforms,
                    &draw_parameters,
                )?;
            } else {
                for i in (0..6).filter(|i| (mask & (1 << i)) != 0) {
                    let start_offset = mesh.side_start[i] * 6;
                    let index_count = start_offset + (mesh.side_square_count[i] * 6);
                    if index_count == 0 {
                        continue;
                    }
                    if let Some(indeces) = fe.block_indeces().slice(start_offset..index_count) {
                        frame.draw(
                            mesh.buffer(),
                            indeces,
                            &fe.shaders.block,
                            &uniforms,
                            &draw_parameters,
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn render_sky(
    frame: &mut glium::Frame,
    fe: &ClientState,
    _game: &GameState,
    view: Mat4,
    projection: Mat4,
) -> Result<(), DrawError> {
    let s = RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0;
    let view = view * Mat4::from_scale(Vec3::new(s, s, s));
    let mat_mvp = (projection * view).to_cols_array_2d();
    let in_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    frame.draw(
        fe.meshes.dome.buffer(),
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &fe.shaders.mesh,
        &uniform! {
            mat_mvp: mat_mvp,
            in_color: in_color,
            cur_tex: fe.textures.sky.texture(),
        },
        &glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        },
    )
}

fn render_game(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
) -> Result<(), DrawError> {
    let (window_width, window_height) = fe.window_size();
    let projection = Mat4::perspective_rh_gl(
        fe.fov().to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
    );
    let view = Mat4::from_rotation_x(game.player().rot[1].to_radians());
    let view = view * Mat4::from_rotation_y(game.player().rot[0].to_radians());
    render_sky(frame, fe, game, view, projection)?;

    let view = view * Mat4::from_translation(-game.player().pos);
    let mvp = projection * view;
    fe.particles
        .draw(frame, &fe.display, &fe.shaders.particle, &mvp)?;

    for entity in game.entities.iter() {
        draw_entity(frame, fe, entity, &view, &projection)?;
    }
    render_chungus(frame, fe, game, &mvp)?;

    frame.clear(None, None, true, Some(4000.0), None);
    render_held_item(frame, fe, &projection)
}

pub fn render_frame(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
) -> Result<(), DrawError> {
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
    )
}
