/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use super::meshes::BlockMesh;
use super::Frustum;
use crate::ClientState;
use glam::f32::{Mat4, Vec3};
use glam::IVec3;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;
use wolkenwelten_common::{CHUNK_BITS, CHUNK_SIZE};
use wolkenwelten_game::{Character, Entity, GameState};

pub const RENDER_DISTANCE: f32 = 192.0;
const FADE_DISTANCE: f32 = 32.0;
const FADEOUT_DISTANCE: f32 = FADE_DISTANCE * FADE_DISTANCE;
const FADEOUT_START_DISTANCE: f32 = (RENDER_DISTANCE - 32.0) * (RENDER_DISTANCE - 32.0);
pub const VIEW_STEPS: i32 = (RENDER_DISTANCE as i32 / CHUNK_SIZE as i32) + 1;

fn calc_fov(fov: f32, player: &Character) -> f32 {
    let new = 90.0 + ((player.vel.length() - 0.025) * 40.0).clamp(0.0, 90.0);
    (fov + (new - fov) / 32.0).clamp(90.0, 170.0)
}

pub fn set_viewport(fe: &ClientState) {
    let (w, h) = fe.window_size();
    // unsafe { gl::Viewport(0, 0, w.try_into().unwrap(), h.try_into().unwrap()) }
}

fn draw_entity(fe: &ClientState, entity: &Entity, view: &Mat4, projection: &Mat4) {
    let rot = entity.rot();
    let pos = entity.pos();

    let model = Mat4::from_rotation_x(rot.x);
    let model = Mat4::from_rotation_y(rot.y) * model;
    let model = Mat4::from_translation(pos) * model;
    let vp = projection.mul_mat4(view);
    let mvp = vp.mul_mat4(&model);

    //fe.shaders.mesh.set_mvp(&mvp);
    //fe.textures.pear.bind();
    fe.meshes.pear.draw();
}

/*
pub fn render_init() {
    unsafe {
        gl::ClearColor(0.32, 0.63, 0.96, 1.0);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::Enable(gl::BLEND);
        gl::Enable(gl::TEXTURE0);
        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);

        gl::Enable(gl::PROGRAM_POINT_SIZE);
    }
    let major_version: i32 = unsafe {
        let mut tmp: GLint = 0;
        gl::GetIntegerv(gl::MAJOR_VERSION, std::ptr::addr_of_mut!(tmp));
        tmp
    };
    let minor_version: i32 = unsafe {
        let mut tmp: GLint = 0;
        gl::GetIntegerv(gl::MINOR_VERSION, std::ptr::addr_of_mut!(tmp));
        tmp
    };
    set_gl_version(major_version, minor_version);
}
*/

pub fn prepare_chunk(fe: &mut ClientState, game: &GameState, pos: IVec3, now: Instant) {
    if let Some(chunk) = game.world.get_chunk(&pos) {
        if let Some(mesh) = fe.world_mesh.get_mut(&pos) {
            if chunk.get_light().get_last_updated() < mesh.get_last_updated() {
                return;
            }
            mesh.update(chunk.get_block(), chunk.get_light(), game, now);
            return;
        }
        let mut mesh = BlockMesh::new(fe.block_indeces());
        mesh.update(chunk.get_block(), chunk.get_light(), game, now);
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

pub fn prepare_frame(fe: &mut ClientState, game: &GameState) {
    fe.calc_fps();
    fe.gc(&game.player);
    fe.set_fov(calc_fov(fe.fov(), &game.player));
    prepare_chunks(fe, game);
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

fn render_chungus(fe: &ClientState, game: &GameState, mvp: &Mat4) {
    let frustum = Frustum::extract(mvp);
    let render_queue = build_render_queue(game.player.pos, &frustum);
    let now = Instant::now();

    //fe.shaders.block.set_used();
    //fe.shaders.block.set_mvp(mvp);
    //fe.textures.blocks.bind();

    for entry in render_queue.iter() {
        if let Some(mesh) = fe.world_mesh.get(&entry.pos) {
            let td = (now - mesh.get_first_created()).as_millis();
            let fade_in = (td as f32 / 500.0).clamp(0.0, 1.0);
            let alpha = entry.alpha * fade_in;
            /*
            fe.shaders.block.set_alpha(alpha);
            fe.shaders
                .block
                .set_trans(entry.trans.x, entry.trans.y, entry.trans.z);
             */
            mesh.draw(entry.mask)
        }
    }
}
