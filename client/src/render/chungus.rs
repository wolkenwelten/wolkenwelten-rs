// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{meshes::BlockMesh, ClientState, Frustum, QueueEntry, RENDER_DISTANCE};
use anyhow::Result;
use glam::{f32::Mat4, IVec3};
use glium::{uniform, DrawError, Surface};
use std::time::Instant;
use wolkenwelten_common::{CHUNK_BITS, CHUNK_SIZE};
use wolkenwelten_game::GameState;

use super::FADE_DISTANCE;

const FADEOUT_DISTANCE: f32 = FADE_DISTANCE * FADE_DISTANCE;
const FADEOUT_START_DISTANCE: f32 = (RENDER_DISTANCE - 32.0) * (RENDER_DISTANCE - 32.0);

fn prepare_chunk(fe: &mut ClientState, game: &GameState, pos: IVec3, now: Instant) -> Result<()> {
    if let Some(chunk) = game.world.get_chunk(&pos) {
        if let Some(mesh) = fe.world_mesh.get_mut(&pos) {
            if chunk.get_light().get_last_updated() >= mesh.get_last_updated() {
                let block_types = game.world.blocks.borrow();
                mesh.update(
                    &fe.display,
                    chunk.get_block(),
                    chunk.get_light(),
                    &block_types,
                    now,
                )?;
            }
        } else {
            let mut mesh = BlockMesh::new(&fe.display)?;
            let block_types = game.world.blocks.borrow();
            mesh.update(
                &fe.display,
                chunk.get_block(),
                chunk.get_light(),
                &block_types,
                now,
            )?;
            fe.world_mesh.insert(pos, mesh);
        }
    }
    Ok(())
}

pub fn prepare(fe: &mut ClientState, game: &GameState) -> Result<()> {
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

pub fn draw(
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
        .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
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
                let index_count = (mesh.side_start[5] + mesh.side_square_count[5]) * 6;
                if let Some(indeces) = fe.block_indeces().slice(..index_count) {
                    frame.draw(
                        mesh.buffer(),
                        indeces,
                        &fe.shaders.block,
                        &uniforms,
                        &draw_parameters,
                    )?;
                }
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
