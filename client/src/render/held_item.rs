// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use anyhow::Result;
use glam::{Mat4, Vec3};
use wolkenwelten_common::Item;
use wolkenwelten_game::{CharacterAnimation, GameState};

const ANIMATION_DUR: u128 = 250;

fn get_animation_r(ani: CharacterAnimation) -> f32 {
    match ani {
        CharacterAnimation::Hit(i) => {
            let t = i.elapsed().as_millis().clamp(0, ANIMATION_DUR) as f32 / ANIMATION_DUR as f32
                * 180.0;
            t.to_radians().sin()
        }
        _ => 0.0,
    }
}

pub fn draw(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
    projection: &Mat4,
) -> Result<()> {
    let t = (fe.ticks() as f32 / 512.0).sin();
    let r = get_animation_r(game.player().animation());
    let model = Mat4::from_scale(Vec3::new(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0));
    let model = Mat4::from_rotation_x((t * 3.0 - r * 20.0).to_radians()) * model;
    let model = Mat4::from_rotation_y((-10.0_ + t * 1.0).to_radians()) * model;
    let pos = Vec3::new(1.35, -0.9 + t * 0.05, -1.0 - r * 0.5);
    let model = Mat4::from_translation(pos) * model;
    let mvp = projection.mul_mat4(&model);
    let item = game.player().item();

    match item {
        Item::Block(bi) => fe.meshes.blocks[bi.block as usize].draw(
            frame,
            &fe.textures.blocks_raw,
            &fe.shaders.mesh,
            &mvp,
        ),
        Item::None => fe
            .meshes
            .fist
            .draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0), /*
                                                                            _ => fe.meshes
                                                                                .bag
                                                                                .draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
                                                                            */
    }
}
