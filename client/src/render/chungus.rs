// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{meshes::BlockMesh, ClientState, Frustum, QueueEntry};
use anyhow::Result;
use glam::{IVec3, Mat4};
use glium::{uniform, Surface};
use std::{collections::HashSet, time::Instant};
use wolkenwelten_common::{ChunkBlockData, ChunkRequestQueue};
use wolkenwelten_game::GameState;

pub fn should_update(mesh: &BlockMesh, chunks: &[&ChunkBlockData; 27]) -> bool {
    for chunk in chunks.iter() {
        if chunk.get_last_updated() >= mesh.get_last_updated() {
            return true;
        }
    }
    false
}

pub fn handle_requests(
    fe: &mut ClientState,
    game: &GameState,
    request: &mut ChunkRequestQueue,
) -> Result<()> {
    let now = Instant::now();

    let mut light_reqs: HashSet<IVec3> = HashSet::new();
    let mut block_reqs: HashSet<IVec3> = HashSet::new();
    request.get_mesh_mut().iter().for_each(|pos| {
        if let Some(lights) = game.world.get_tri_complex_light(&pos, &mut light_reqs) {
            if let Some(chunks) = game.world.get_tri_chunk(&pos, &mut block_reqs) {
                let block_types = game.world.blocks.borrow();
                if let Some(mesh) = fe.world_mesh.get_mut(&pos) {
                    if lights[1 * 3 * 3 + 1 * 3 + 1].get_last_updated() >= mesh.get_last_updated()
                        || should_update(mesh, &chunks)
                    {
                        let _ = mesh.update(&fe.display, &chunks, &lights, &block_types, now);
                    }
                } else {
                    let mesh = BlockMesh::new(&fe.display);
                    if let Ok(mut mesh) = mesh {
                        let r = mesh.update(&fe.display, &chunks, &lights, &block_types, now);
                        if r.is_ok() {
                            fe.world_mesh.insert(*pos, mesh);
                        }
                    }
                }
            }
        }
    });
    light_reqs
        .iter()
        .for_each(|pos| request.complex_light(*pos));
    block_reqs.iter().for_each(|pos| request.block(*pos));
    request.get_mesh_mut().clear();

    Ok(())
}

pub fn draw(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
    mvp: &Mat4,
    request: &mut ChunkRequestQueue,
) -> Result<()> {
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
        request.mesh(entry.pos);
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
                blend: glium::draw_parameters::Blend {
                    color: glium::draw_parameters::BlendingFunction::Addition {
                        source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                        destination:
                            glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    alpha: glium::draw_parameters::BlendingFunction::Addition {
                        source: glium::draw_parameters::LinearBlendingFactor::One,
                        destination:
                            glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
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
