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
use gl::types::GLint;
use glam::f32::{Mat4, Vec3};
use glam::IVec3;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::RwLock;
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

static GL_VERSION: RwLock<(i32, i32)> = RwLock::new((0, 0));

pub fn can_use_object_labels() -> bool {
    let ver = GL_VERSION.read().unwrap();
    let (major_version, minor_version) = *ver;
    major_version > 4 || (major_version == 4) && (minor_version >= 3)
}

pub fn set_gl_version(major_version: i32, minor_version: i32) {
    let mut ver = GL_VERSION.write().unwrap();
    *ver = (major_version, minor_version);
}

pub fn set_viewport(fe: &ClientState) {
    let (w, h) = fe.window_size();
    unsafe { gl::Viewport(0, 0, w.try_into().unwrap(), h.try_into().unwrap()) }
}

fn draw_entity(fe: &ClientState, entity: &Entity, view: &Mat4, projection: &Mat4) {
    let rot = entity.rot();
    let pos = entity.pos();

    let model = Mat4::from_rotation_x(rot.x);
    let model = Mat4::from_rotation_y(rot.y) * model;
    let model = Mat4::from_translation(pos) * model;
    let vp = projection.mul_mat4(view);
    let mvp = vp.mul_mat4(&model);

    fe.shaders.mesh.set_mvp(&mvp);
    fe.textures.pear.bind();
    fe.meshes.pear.draw();
}

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

pub fn prepare_chunk(fe: &mut ClientState, game: &GameState, pos: IVec3, now: u64) {
    match fe.world_mesh.get(&pos) {
        Some(_) => {}
        _ => {
            if let Some(chunk) = game.get_chunk_block(pos) {
                if let Some(light) = game.get_chunk_light(pos) {
                    let mut mesh = BlockMesh::new(fe.block_indeces());
                    mesh.update(chunk, light, game, now);
                    fe.world_mesh.insert(pos, mesh);
                }
            }
        }
    }
}

fn prepare_chunks(fe: &mut ClientState, game: &GameState) {
    let now = game.get_millis();
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
        .push_string(8, 8, 2, 0xFFFFFFFF, fps_text.as_str());

    let pos_text = format!(
        "X:{:8.2} Y:{:8.2} Z:{:8.2}   Ticks:{}",
        game.player.pos[0], game.player.pos[1], game.player.pos[2], game.ticks_elapsed
    );
    fe.ui_mesh
        .push_string(8, 40, 2, 0xFFFFFFFF, pos_text.as_str());

    #[cfg(debug_assertions)]
    {
        let rot_text = format!(
            "Y:{:8.2} P:{:8.2} R:{:8.2}",
            game.player.rot[0], game.player.rot[1], game.player.rot[2]
        );
        let col_text = format!(
            "Entities: {}   Chunks: {}   BlockMeshes: {}",
            game.get_entity_count(),
            game.world.block_data.len(),
            fe.world_mesh.len(),
        );
        fe.ui_mesh
            .push_string(8, 60, 2, 0xFFFFFFFF, rot_text.as_str())
            .push_string(8, 100, 2, 0xFFFFFFFF, col_text.as_str());
    }
    let (window_width, window_height) = fe.window_size();

    let pos = (
        window_width as i16 / 2 - 32,
        window_height as i16 / 2 - 32,
        32,
        32,
    );
    let tex = (200, 252, 4, 4);
    fe.ui_mesh.push_box(pos, tex, 0x7FFFFFFF);
    fe.ui_mesh.prepare();
}

pub fn prepare_frame(fe: &mut ClientState, game: &GameState) {
    prepare_ui(fe, game);
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
    let mut render_queue: BinaryHeap<QueueEntry> = BinaryHeap::with_capacity(4096);
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
    let now = game.get_millis();

    fe.shaders.block.set_used();
    fe.shaders.block.set_mvp(mvp);
    fe.textures.blocks.bind();

    for entry in render_queue.iter() {
        if let Some(mesh) = fe.world_mesh.get(&entry.pos) {
            let td = now - mesh.last_updated_at();
            let fade_in = (td as f32 / 500.0).clamp(0.0, 1.0);
            let alpha = entry.alpha * fade_in;
            fe.shaders.block.set_alpha(alpha);
            fe.shaders
                .block
                .set_trans(entry.trans.x, entry.trans.y, entry.trans.z);
            mesh.draw(entry.mask)
        }
    }
}

fn render_sky(fe: &ClientState, _game: &GameState, view: Mat4, projection: Mat4) {
    let s = RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0;
    let view = view * Mat4::from_scale(Vec3::new(s, s, s));
    let mvp = projection * view;
    fe.shaders.mesh.set_used();
    fe.shaders.mesh.set_color(1.0, 1.0, 1.0, 1.0);
    fe.textures.sky.bind();
    fe.shaders.mesh.set_mvp(&mvp);
    fe.meshes.dome.draw();
}

fn render_game(fe: &ClientState, game: &GameState) {
    let (window_width, window_height) = fe.window_size();
    let projection = Mat4::perspective_rh_gl(
        fe.fov().to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
    );
    let view = Mat4::from_rotation_x(game.player.rot[1].to_radians());
    let view = view * Mat4::from_rotation_y(game.player.rot[0].to_radians());
    render_sky(fe, game, view, projection);

    let view = view * Mat4::from_translation(-game.player.pos);
    let mvp = projection * view;

    fe.textures.pear.bind();
    game.entities
        .iter()
        .for_each(|entity| draw_entity(fe, entity, &view, &projection));
    render_chungus(fe, game, &mvp);
}

pub fn render_frame(fe: &ClientState, game: &GameState) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        if fe.wireframe() {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    };
    render_game(fe, game);
    unsafe {
        if fe.wireframe() {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
    };

    let (window_width, window_height) = fe.window_size();
    let perspective = Mat4::orthographic_rh_gl(
        0.0,
        window_width as f32,
        window_height as f32,
        0.0,
        -10.0,
        10.0,
    );
    unsafe { gl::Disable(gl::DEPTH_TEST) }
    fe.shaders.text.set_used();
    fe.shaders.text.set_mvp(&perspective);
    fe.textures.gui.bind();
    fe.ui_mesh.draw();
}
