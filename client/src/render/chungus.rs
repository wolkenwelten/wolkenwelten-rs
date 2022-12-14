// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{meshes::BlockMesh, ClientState, Frustum, QueueEntry, RenderPassArgs};
use anyhow::Result;
use glam::{IVec3, Mat4};
use std::{collections::HashSet, time::Instant};
use wolkenwelten_core::{
    Chungus, ChunkBlockData, ChunkFluidData, ChunkRequestQueue, GameState, BLOCKS, FLUIDS,
};

pub fn should_update(mesh: &BlockMesh, chunks: &[&ChunkBlockData; 27]) -> bool {
    let mlu = mesh.last_updated();
    chunks[13].last_updated() > mlu
        || chunks[Chungus::neighbor_off(0, 1, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(2, 1, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 0, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 2, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 1, 0)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 1, 2)].last_updated() > mlu
}

pub fn should_update_fluid(mesh: &BlockMesh, chunks: &[&ChunkFluidData; 27]) -> bool {
    let mlu = mesh.last_updated();
    chunks[13].last_updated() > mlu
        || chunks[Chungus::neighbor_off(0, 1, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(2, 1, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 0, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 2, 1)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 1, 0)].last_updated() > mlu
        || chunks[Chungus::neighbor_off(1, 1, 2)].last_updated() > mlu
}

pub fn handle_requests(
    fe: &mut ClientState,
    game: &GameState,
    request: &mut ChunkRequestQueue,
) -> Result<()> {
    let now = Instant::now();
    let mut light_reqs: HashSet<IVec3> = HashSet::new();
    let mut block_reqs: HashSet<IVec3> = HashSet::new();
    let world = game.world_mut();

    BLOCKS.with(|blocks| {
        let block_types = blocks.borrow();
        request.get_mesh_mut().iter().for_each(|pos| {
            if let Some(lights) = world.get_tri_complex_light(pos, &mut light_reqs) {
                if let Some(chunks) = world.get_tri_chunk(pos, &mut block_reqs) {
                    if let Some(mesh) = fe.world_mesh.get_mut(pos) {
                        if lights[13].last_updated() >= mesh.last_updated()
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
    });

    FLUIDS.with(|fluids| {
        let fluid_types = fluids.borrow();
        request.get_fluid().iter().for_each(|pos| {
            if let Some(lights) = world.get_tri_complex_light(pos, &mut light_reqs) {
                if let Some(chunks) = world.get_tri_chunk(pos, &mut block_reqs) {
                    if let Some(fluids) = world.get_tri_fluids(pos, &mut block_reqs) {
                        if let Some(mesh) = fe.fluid_mesh.get_mut(pos) {
                            if lights[13].last_updated() >= mesh.last_updated()
                                || should_update_fluid(mesh, &fluids)
                                || should_update(mesh, &chunks)
                            {
                                let _ = mesh.update_fluid(
                                    &fe.display,
                                    &chunks,
                                    &lights,
                                    &fluids,
                                    &fluid_types,
                                    now,
                                );
                            }
                        } else {
                            let mesh = BlockMesh::new(&fe.display);
                            if let Ok(mut mesh) = mesh {
                                let r = mesh.update_fluid(
                                    &fe.display,
                                    &chunks,
                                    &lights,
                                    &fluids,
                                    &fluid_types,
                                    now,
                                );
                                if r.is_ok() {
                                    fe.fluid_mesh.insert(*pos, mesh);
                                }
                            }
                        }
                    }
                }
            }
        });
    });

    light_reqs
        .iter()
        .for_each(|pos| request.complex_light(*pos));
    block_reqs.iter().for_each(|pos| request.block(*pos));
    request.get_mesh_mut().clear();
    request.get_fluid_mut().clear();

    Ok(())
}

fn chungus_draw_blocks(
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

    let block_tex = fe
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
            mesh.draw(frame, fe, entry, mat_mvp, block_tex, alpha)?;
        }
    }
    Ok(())
}

fn chungus_draw_fluids(
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

    let fluid_tex = fe
        .textures
        .fluids
        .texture()
        .sampled()
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
        .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat);

    for entry in render_queue.iter() {
        request.fluid(entry.pos);
        if let Some(mesh) = fe.fluid_mesh.get(&entry.pos) {
            let td = (now - mesh.get_first_created()).as_millis();
            let fade_in = (td as f32 / 500.0).clamp(0.0, 1.0);
            let alpha = entry.alpha * fade_in * 0.8;
            mesh.draw(frame, fe, entry, mat_mvp, fluid_tex, alpha)?;
        }
    }
    Ok(())
}

pub fn chungus_block_pass(args: RenderPassArgs) -> RenderPassArgs {
    let mvp = args.projection * args.view;
    chungus_draw_blocks(args.frame, args.fe, args.game, &mvp, args.request)
        .expect("Error while drawing voxel chunks");
    args
}

pub fn chungus_fluid_pass(args: RenderPassArgs) -> RenderPassArgs {
    let mvp = args.projection * args.view;
    chungus_draw_fluids(args.frame, args.fe, args.game, &mvp, args.request)
        .expect("Error while drawing voxel chunks");
    args
}
