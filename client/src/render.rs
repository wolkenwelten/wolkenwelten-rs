// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::meshes::BlockMesh;
use super::Frustum;
use crate::ClientState;
use glam::{IVec3, f32::{Mat4, Vec3}};
use glium::uniform;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;
use wolkenwelten_common::{CHUNK_BITS, CHUNK_SIZE};
use wolkenwelten_game::{Character, Entity, GameState};

use glium::Surface;

pub const RENDER_DISTANCE: f32 = 192.0;
const FADE_DISTANCE: f32 = 32.0;
const FADEOUT_DISTANCE: f32 = FADE_DISTANCE * FADE_DISTANCE;
const FADEOUT_START_DISTANCE: f32 = (RENDER_DISTANCE - 32.0) * (RENDER_DISTANCE - 32.0);
pub const VIEW_STEPS: i32 = (RENDER_DISTANCE as i32 / CHUNK_SIZE as i32) + 1;

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
) {
    let rot = entity.rot();
    let pos = entity.pos();

    let model = Mat4::from_scale(Vec3::new(1.0/16.0,1.0/16.0,1.0/16.0));
    let model = Mat4::from_rotation_x(rot.x - 90.0) * model;
    let model = Mat4::from_rotation_y(rot.y) * model;
    let model = Mat4::from_translation(pos) * model;
    let vp = projection.mul_mat4(view);
    let mvp = vp.mul_mat4(&model);
    let mat_mvp = mvp.to_cols_array_2d();

    let trans_pos:[f32; 3] = fe.meshes.grenade.trans_pos();
    let color_alpha:f32 = 1.0;

    frame
        .draw(
            fe.meshes.grenade.buffer(),
            fe.block_indeces(),
            &fe.shaders.block,
            &uniform! {
                mat_mvp: mat_mvp,
                trans_pos: trans_pos,
                color_alpha: color_alpha,
                cur_tex: fe.meshes.grenade.texture(),
            },
            &glium::DrawParameters {
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                blend: glium::draw_parameters::Blend::alpha_blending(),
                depth: glium::draw_parameters::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap();
}

pub fn prepare_chunk(fe: &mut ClientState, game: &GameState, pos: IVec3, now: Instant) {
    if let Some(chunk) = game.world.get_chunk(&pos) {
        if let Some(mesh) = fe.world_mesh.get_mut(&pos) {
            if chunk.get_light().get_last_updated() < mesh.get_last_updated() {
                return;
            }
            mesh.update(&fe.display, chunk.get_block(), chunk.get_light(), &game.world.blocks, now);
            return;
        }
        let mut mesh = BlockMesh::new(&fe.display);
        mesh.update(&fe.display, chunk.get_block(), chunk.get_light(), &game.world.blocks, now);
        fe.world_mesh.insert(pos, mesh);
    }
}

fn prepare_chunks(fe: &mut ClientState, game: &GameState) {
    let now = Instant::now();
    let px = (game.player.pos.x as i32) >> CHUNK_BITS;
    let py = (game.player.pos.y as i32) >> CHUNK_BITS;
    let pz = (game.player.pos.z as i32) >> CHUNK_BITS;
    for cx in -VIEW_STEPS..=VIEW_STEPS {
        for cy in -VIEW_STEPS..=VIEW_STEPS {
            for cz in -VIEW_STEPS..=VIEW_STEPS {
                let pos = IVec3::new(cx + px, cy + py, cz + pz);
                let d = (pos.as_vec3() * CHUNK_SIZE as f32) - game.player.pos;
                let d = d.dot(d);
                if d < FADEOUT_DISTANCE + FADEOUT_START_DISTANCE {
                    prepare_chunk(fe, game, pos, now);
                }
            }
        }
    }
}

fn prepare_ui(fe: &mut ClientState, game: &GameState) {
    let fps_text = format!("FPS: {}", fe.fps());
    fe.ui_mesh
        .push_string(8, 8, 2, [0xFF, 0xFF, 0xFF, 0xFF], fps_text.as_str());

    let pos_text = format!(
        "X:{:8.2} Y:{:8.2} Z:{:8.2}   Ticks:{}",
        game.player.pos[0], game.player.pos[1], game.player.pos[2], game.ticks_elapsed
    );
    fe.ui_mesh
        .push_string(8, 40, 2, [0xFF, 0xFF, 0xFF, 0xFF], pos_text.as_str());

    let block_name = game.world.blocks[game.player.block_selection() as usize].name();
    let block_sel_text = format!("Block selection: {}", block_name);
    fe.ui_mesh.push_string(
        8,
        fe.window_size().1 as i16 - 20,
        2,
        [0xFF, 0xFF, 0xFF, 0xFF],
        block_sel_text.as_str(),
    );

    #[cfg(debug_assertions)]
    {
        let rot_text = format!(
            "Y:{:8.2} P:{:8.2} R:{:8.2}",
            game.player.rot[0], game.player.rot[1], game.player.rot[2]
        );
        let col_text = format!(
            "Entities: {}   Chunks: {}   BlockMeshes: {}  Particles: {}",
            game.get_entity_count(),
            game.world.chunks.len(),
            fe.world_mesh.len(),
            fe.particles.len(),
        );
        fe.ui_mesh
            .push_string(8, 60, 2, [0xFF, 0xFF, 0xFF, 0xFF], rot_text.as_str())
            .push_string(8, 100, 2, [0xFF, 0xFF, 0xFF, 0xFF], col_text.as_str());
    }
    let (window_width, window_height) = fe.window_size();

    let pos = (
        window_width as i16 / 2 - 32,
        window_height as i16 / 2 - 32,
        32,
        32,
    );
    let tex = (200, 252, 4, 4);
    fe.ui_mesh.push_box(pos, tex, [0xFF, 0xFF, 0xFF, 0x7F]);
    fe.ui_mesh.prepare(&fe.display);
}

pub fn prepare_frame(fe: &mut ClientState, game: &GameState) {
    prepare_ui(fe, game);
    fe.calc_fps();
    fe.gc(&game.player);
    fe.set_fov(calc_fov(fe.fov(), &game.player));
    prepare_chunks(fe, game);
    fe.particles.update(game.player.pos, RENDER_DISTANCE);
}

#[derive(Debug, Default)]
struct QueueEntry {
    dist: i64,
    pos: IVec3,
    trans: Vec3,
    mask: u8,
    alpha: f32,
}

impl QueueEntry {
    pub fn new(pos: IVec3, trans: Vec3, dist: i64, mask: u8, alpha: f32) -> Self {
        Self {
            dist,
            pos,
            trans,
            mask,
            alpha,
        }
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        //self.dist.cmp(&other.dist)
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueueEntry {}
impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

fn build_render_queue(player_pos: Vec3, frustum: &Frustum) -> BinaryHeap<QueueEntry> {
    let mut render_queue: BinaryHeap<QueueEntry> = BinaryHeap::with_capacity(512);
    let px = (player_pos.x.floor() as i32) >> CHUNK_BITS;
    let py = (player_pos.y.floor() as i32) >> CHUNK_BITS;
    let pz = (player_pos.z.floor() as i32) >> CHUNK_BITS;

    for cx in -VIEW_STEPS..=VIEW_STEPS {
        for cy in -VIEW_STEPS..=VIEW_STEPS {
            for cz in -VIEW_STEPS..=VIEW_STEPS {
                let x = px + cx;
                let y = py + cy;
                let z = pz + cz;
                let trans_x = x as f32 * CHUNK_SIZE as f32;
                let trans_y = y as f32 * CHUNK_SIZE as f32;
                let trans_z = z as f32 * CHUNK_SIZE as f32;
                if !frustum.contains_cube(Vec3::new(trans_x, trans_y, trans_z), CHUNK_SIZE as f32) {
                    continue;
                } else {
                    let trans = Vec3::new(trans_x, trans_y, trans_z);
                    let dist = trans - player_pos;
                    let dist = dist.length();
                    if dist < RENDER_DISTANCE {
                        let alpha = if dist < (RENDER_DISTANCE - FADE_DISTANCE) {
                            1.0
                        } else {
                            1.0 - ((dist - (RENDER_DISTANCE - FADE_DISTANCE)) / FADE_DISTANCE)
                        };
                        let dist = (dist * 8192.0) as i64;
                        let pos = IVec3::new(x, y, z);
                        let mask = BlockMesh::calc_mask(cx, cy, cz);
                        render_queue.push(QueueEntry::new(pos, trans, dist, mask, alpha));
                    }
                }
            }
        }
    }
    render_queue
}

fn render_chungus(frame: &mut glium::Frame, fe: &ClientState, game: &GameState, mvp: &Mat4) {
    let frustum = Frustum::extract(mvp);
    let render_queue = build_render_queue(game.player.pos, &frustum);
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
                frame
                    .draw(
                        mesh.buffer(),
                        fe.block_indeces(),
                        &fe.shaders.block,
                        &uniforms,
                        &draw_parameters,
                    )
                    .unwrap();
            } else {
                (0..6).filter(|i| (mask & (1 << i)) != 0).for_each(|i| {
                    let start_offset = mesh.side_start[i] * 6;
                    let index_count = start_offset + (mesh.side_square_count[i] * 6);
                    if index_count == 0 {
                        return;
                    }
                    if let Some(indeces) = fe.block_indeces().slice(start_offset..index_count) {
                        frame
                            .draw(
                                mesh.buffer(),
                                indeces,
                                &fe.shaders.block,
                                &uniforms,
                                &draw_parameters,
                            )
                            .unwrap();
                    }
                });
            }
        }
    }
}

fn render_sky(
    frame: &mut glium::Frame,
    fe: &ClientState,
    _game: &GameState,
    view: Mat4,
    projection: Mat4,
) {
    let s = RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0;
    let view = view * Mat4::from_scale(Vec3::new(s, s, s));
    let mat_mvp = (projection * view).to_cols_array_2d();
    let in_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    frame
        .draw(
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
        .unwrap();
}

fn render_game(frame: &mut glium::Frame, fe: &ClientState, game: &GameState) {
    let (window_width, window_height) = fe.window_size();
    let projection = Mat4::perspective_rh_gl(
        fe.fov().to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
    );
    let view = Mat4::from_rotation_x(game.player.rot[1].to_radians());
    let view = view * Mat4::from_rotation_y(game.player.rot[0].to_radians());
    render_sky(frame, fe, game, view, projection);

    let view = view * Mat4::from_translation(-game.player.pos);
    let mvp = projection * view;
    fe.particles
        .draw(frame, &fe.display, &fe.shaders.particle, &mvp)
        .unwrap();

    game.entities
        .iter()
        .for_each(|entity| draw_entity(frame, fe, entity, &view, &projection));
    render_chungus(frame, fe, game, &mvp);
}

pub fn render_frame(frame: &mut glium::Frame, fe: &ClientState, game: &GameState) {
    frame.clear(
        None,
        Some((0.32, 0.63, 0.96, 1.0)),
        true,
        Some(4000.0),
        None,
    );

    render_game(frame, fe, game);

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
    frame
        .draw(
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
        .unwrap();
}
