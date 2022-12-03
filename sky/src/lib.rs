// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{Mat4, Vec3};
use glium::{uniform, Surface};
use wolkenwelten_client::{Mesh, RenderInitArgs, Texture};
use wolkenwelten_core::CHUNK_SIZE;

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    let dome_mesh = Mesh::from_obj_string(&args.fe.display, include_str!("../assets/skydome.obj"))
        .expect("Couldn't load skydome mesh");
    let sky_texture = Texture::from_bytes(&args.fe.display, include_bytes!("../assets/sky.png"))
        .expect("Couldn't load sky texture");

    args.render_reactor
        .pre_world_render
        .push(Box::new(move |args| {
            let view = Mat4::from_rotation_x(args.game.player().rot[1].to_radians());
            let view = view * Mat4::from_rotation_y(args.game.player().rot[0].to_radians());

            let s = args.render_distance + CHUNK_SIZE as f32 * 2.0;
            let view = view * Mat4::from_scale(Vec3::new(s, s, s));
            let mat_mvp = (args.projection * view).to_cols_array_2d();
            let in_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

            let _ = args.frame.draw(
                dome_mesh.buffer(),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &args.fe.shaders.mesh,
                &uniform! {
                    mat_mvp: mat_mvp,
                    in_color: in_color,
                    cur_tex: sky_texture.texture(),
                },
                &glium::DrawParameters {
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    ..Default::default()
                },
            );
            args
        }));
    args
}
