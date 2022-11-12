// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use anyhow::Result;
use glam::{Mat4, Vec3};

pub fn draw(frame: &mut glium::Frame, fe: &ClientState, projection: &Mat4) -> Result<()> {
    let t = (fe.ticks() as f32 / 512.0).sin();
    let model = Mat4::from_scale(Vec3::new(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0));
    let model = Mat4::from_rotation_x((t * 6.0).to_radians()) * model;
    let model = Mat4::from_rotation_y((-10.0_ + t * 1.0).to_radians()) * model;
    let pos = Vec3::new(1.0, -0.5 + t * 0.05, -1.0);
    let model = Mat4::from_translation(pos) * model;
    let mvp = projection.mul_mat4(&model);

    fe.meshes
        .grenade
        .draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
}
