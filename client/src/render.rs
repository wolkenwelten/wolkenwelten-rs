use crate::ClientState;
use glam::f32::Mat4;
use rostregen_game::{Entity, GameState, ChunkPosition};
use super::meshes::BlockMesh;

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
}

pub fn prepare_chunk(fe: &mut ClientState, game: &GameState, pos:&ChunkPosition) {
    match fe.world_mesh.get(pos) {
        Some(_) => {}
        _ => match game.get_chunk_block(pos) {
            Some(chnk) => {
                let mut mesh = BlockMesh::new();
                mesh.update(chnk, &game);
                fe.world_mesh.insert(pos.clone(), mesh);
            }
            _ => {}
        }
    }
}

pub fn prepare_frame(fe: &mut ClientState, game: &GameState) {
    let fps_text = format!("FPS: {}", fe.fps());
    let pos_text = format!(
        "X:{:8.2} Y:{:8.2} Z:{:8.2}",
        game.player_position[0], game.player_position[1], game.player_position[2]
    );
    let rot_text = format!(
        "Y:{:8.2} P:{:8.2} R:{:8.2}",
        game.player_rotation[0], game.player_rotation[1], game.player_rotation[2]
    );

    fe.ui_mesh
        .push_string(8, 8, 2, 0xFFFFFFFF, fps_text.as_str())
        .push_string(8, 40, 1, 0xFFFFFFFF, pos_text.as_str())
        .push_string(8, 50, 1, 0xFFFFFFFF, rot_text.as_str())
        .prepare();

    fe.calc_fps();

    for x in -2..=2 {
        for y in -2..=2 {
            for z in -2..=2 {
                let pos = ChunkPosition::new(x,y,z);
                prepare_chunk(fe, game, &pos);
            }
        }
    }
}

fn render_game(fe: &ClientState, game: &GameState) {
    let projection = glam::Mat4::perspective_rh_gl(
        90.0_f32.to_radians(),
        (fe.window_width as f32) / (fe.window_height as f32),
        0.1,
        100.0,
    );

    let view = glam::Mat4::from_rotation_x(game.player_rotation[1].to_radians());
    let view = view * glam::Mat4::from_rotation_y(game.player_rotation[0].to_radians());
    let view = view * glam::Mat4::from_translation(-game.player_position);
    let mvp = projection * view;

    fe.shaders.mesh.set_used();
    fe.shaders.mesh.set_color(1.0, 1.0, 1.0, 1.0);
    fe.textures.pear.bind();

    for entity in &game.entities {
        draw_entity(&fe, &entity, &view, &projection);
    }

    fe.shaders.block.set_used();
    fe.shaders.block.set_mvp(&mvp);
    fe.shaders.block.set_alpha(1.0);
    fe.textures.blocks.bind();

    for x in -2..=2 {
        for y in -2..=2 {
            for z in -2..=2 {
                fe.shaders.block.set_trans((x as f32)*16.0, (y as f32)*16.0, (z as f32)*16.0);
                let pos = ChunkPosition::new(x,y,z);
                match fe.world_mesh.get(&pos) {
                    Some(mesh) => mesh.draw(),
                    _ => ()
                }
            }
        }
    }
}

pub fn render_frame(fe: &ClientState, game: &GameState) {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
    render_game(&fe, &game);

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
