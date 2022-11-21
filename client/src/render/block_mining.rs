// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{ClientState, Mesh, MeshVertex};
use anyhow::Result;
use glam::{Mat4, Vec2};
use glium::{draw_parameters::PolygonOffset, uniform, Surface};
use wolkenwelten_game::GameState;

pub fn draw(
    frame: &mut glium::Frame,
    fe: &ClientState,
    game: &GameState,
    mvp: &Mat4,
) -> Result<()> {
    let mut vertices: Vec<MeshVertex> = vec![];
    let tex_scale = Vec2::new(1.0 / 8.0, -1.0);
    let step = 1.0 / 8.0;
    for (&pos, &m) in game.mining().iter() {
        let pos_off = pos.as_vec3();
        let i = ((m.damage as f32 / m.block_health as f32) * 8.0).floor();
        let tex_off = Vec2::new(i * step, 1.0);
        Mesh::add_block(&mut vertices, pos_off, tex_off, tex_scale);
    }
    let m = Mesh::from_vec(&fe.display, &vertices)?;
    let in_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    let mat_mvp = mvp.to_cols_array_2d();
    let cur_tex = fe
        .textures
        .mining
        .texture()
        .sampled()
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
        .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp);

    frame.draw(
        m.buffer(),
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &fe.shaders.mesh,
        &uniform! {
            mat_mvp: mat_mvp,
            in_color: in_color,
            cur_tex: cur_tex,
        },
        &glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::draw_parameters::Blend {
                color: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::One,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            depth: glium::draw_parameters::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                ..Default::default()
            },
            polygon_offset: PolygonOffset {
                factor: -8.0,
                units: -8.0,
                fill: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )?;
    Ok(())
}
