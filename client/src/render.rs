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
use super::GL_VERSION;
use crate::ClientState;
use gl::types::GLint;
use glam::f32::{Mat4, Vec3};
use glam::IVec3;
use wolkenwelten_game::{Entity, GameState, Character};

pub const VIEW_STEPS: i32 = (128 / 16) + 1;
const FADEOUT_START_DISTANCE: f32 = 96.0 * 96.0;
const FADEOUT_DISTANCE: f32 = 32.0 * 32.0;

fn calc_fov(fov:f32, player:&Character) -> f32 {
    let new = 90.0 + ((player.vel.length() - 0.025) * 40.0).clamp(0.0, 90.0);
    (fov + (new - fov) / 32.0).clamp(90.0, 170.0)
}

pub fn set_viewport(fe: &ClientState) {
    unsafe {
        gl::Viewport(
            0,
            0,
            fe.window_width.try_into().unwrap(),
            fe.window_height.try_into().unwrap(),
        )
    }
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
    unsafe {
        GL_VERSION = (major_version, minor_version);
    }
}

pub fn prepare_chunk(fe: &mut ClientState, game: &GameState, pos: IVec3) {
    match fe.world_mesh.get(&pos) {
        Some(_) => {}
        _ => {
            if let Some(chunk) = game.get_chunk_block(pos) {
                let mut mesh = BlockMesh::new();
                mesh.update(chunk, game);
                fe.world_mesh.insert(pos, mesh);
            }
        }
    }
}

pub fn prepare_frame(fe: &mut ClientState, game: &GameState) {
    let fps_text = format!("FPS: {}", fe.fps());
    let pos_text = format!(
        "X:{:8.2} Y:{:8.2} Z:{:8.2}",
        game.player.pos[0], game.player.pos[1], game.player.pos[2]
    );
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
        .push_string(8, 8, 2, 0xFFFFFFFF, fps_text.as_str())
        .push_string(8, 40, 2, 0xFFFFFFFF, pos_text.as_str())
        .push_string(8, 60, 2, 0xFFFFFFFF, rot_text.as_str())
        .push_string(8, 100, 2, 0xFFFFFFFF, col_text.as_str())
        .prepare();

    fe.calc_fps();
    fe.gc(&game.player);
    fe.cur_fov = calc_fov(fe.cur_fov, &game.player);

    let px = (game.player.pos.x as i32) / 16;
    let py = (game.player.pos.y as i32) / 16;
    let pz = (game.player.pos.z as i32) / 16;
    for cx in -VIEW_STEPS..=VIEW_STEPS {
        for cy in -VIEW_STEPS..=VIEW_STEPS {
            for cz in -VIEW_STEPS..=VIEW_STEPS {
                let pos = IVec3::new(cx + px, cy + py, cz + pz);
                prepare_chunk(fe, game, pos);
            }
        }
    }
}

fn render_game(fe: &ClientState, game: &GameState) {
    let projection = glam::Mat4::perspective_rh_gl(
        fe.cur_fov.to_radians(),
        (fe.window_width as f32) / (fe.window_height as f32),
        0.1,
        178.0,
    );

    let view = glam::Mat4::from_rotation_x(game.player.rot[1].to_radians());
    let view = view * glam::Mat4::from_rotation_y(game.player.rot[0].to_radians());
    let view = view * glam::Mat4::from_translation(-game.player.pos);
    let mvp = projection * view;

    fe.shaders.mesh.set_used();
    fe.shaders.mesh.set_color(1.0, 1.0, 1.0, 1.0);
    fe.textures.pear.bind();

    for entity in &game.entities {
        draw_entity(fe, entity, &view, &projection);
    }

    fe.shaders.block.set_used();
    fe.shaders.block.set_mvp(&mvp);
    fe.shaders.block.set_alpha(1.0);
    fe.textures.blocks.bind();

    let frustum = Frustum::extract(&mvp);

    let px = (game.player.pos.x as i32) / 16;
    let py = (game.player.pos.y as i32) / 16;
    let pz = (game.player.pos.z as i32) / 16;
    for cx in -VIEW_STEPS..=VIEW_STEPS {
        for cy in -VIEW_STEPS..=VIEW_STEPS {
            for cz in -VIEW_STEPS..=VIEW_STEPS {
                let x = px + cx;
                let y = py + cy;
                let z = pz + cz;
                let trans_x = x as f32 * 16.0;
                let trans_y = y as f32 * 16.0;
                let trans_z = z as f32 * 16.0;
                if !frustum.contains_cube(Vec3::new(trans_x, trans_y, trans_z), 16.0) {
                    continue;
                }

                let diff_x = trans_x - game.player.pos.x;
                let diff_y = trans_y - game.player.pos.y;
                let diff_z = trans_z - game.player.pos.z;
                let dist = diff_x * diff_x + diff_y * diff_y + diff_z * diff_z;
                if dist > FADEOUT_DISTANCE + FADEOUT_START_DISTANCE {
                    continue;
                }
                let alpha = if dist < FADEOUT_START_DISTANCE {
                    1.0
                } else {
                    1.0 - ((dist - FADEOUT_START_DISTANCE) / FADEOUT_DISTANCE)
                };

                let pos = IVec3::new(x, y, z);
                if let Some(mesh) = fe.world_mesh.get(&pos) {
                    fe.shaders.block.set_alpha(alpha);
                    fe.shaders.block.set_trans(trans_x, trans_y, trans_z);
                    mesh.draw()
                }
            }
        }
    }
}

pub fn render_frame(fe: &ClientState, game: &GameState) {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
    render_game(fe, game);

    let perspective = glam::Mat4::orthographic_rh_gl(
        0.0,
        fe.window_width as f32,
        fe.window_height as f32,
        0.0,
        -10.0,
        10.0,
    );
    fe.shaders.text.set_used();
    fe.shaders.text.set_mvp(&perspective);
    fe.textures.gui.bind();
    fe.ui_mesh.draw();
}
