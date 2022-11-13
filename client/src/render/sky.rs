// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{ClientState, RENDER_DISTANCE};
use glam::{Mat4, Vec3};
use glium::{uniform, DrawError, Surface};
use wolkenwelten_common::CHUNK_SIZE;

pub fn draw(
    frame: &mut glium::Frame,
    fe: &ClientState,
    view: &Mat4,
    projection: &Mat4,
) -> Result<(), DrawError> {
    let s = RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0;
    let view = *view * Mat4::from_scale(Vec3::new(s, s, s));
    let mat_mvp = (*projection * view).to_cols_array_2d();
    let in_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    frame.draw(
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
}
