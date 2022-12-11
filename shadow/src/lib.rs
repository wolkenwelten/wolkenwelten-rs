// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::Vec3;
use glium::{draw_parameters::PolygonOffset, uniform, Surface};
use wolkenwelten_client::{Mesh, MeshVertex, RenderInitArgs, RenderPassArgs, ShaderList, Texture};
use wolkenwelten_core::GameState;

fn add_vert(
    vertices: &mut Vec<MeshVertex>,
    pos: Vec3,
    pos_off: [f32; 2],
    tex: [f32; 2],
    lightness: f32,
) {
    let pos = [pos.x + pos_off[0], pos.y, pos.z + pos_off[1]];
    vertices.push(MeshVertex {
        pos,
        tex,
        lightness,
    })
}

fn add_quad(vertices: &mut Vec<MeshVertex>, pos: Vec3, lightness: f32, size: f32) {
    let pos = pos + Vec3::new(-(size / 2.0), 0.0, -(size / 2.0));
    add_vert(vertices, pos, [0.0, 0.0], [0.0, 0.0], lightness);
    add_vert(vertices, pos, [size, size], [1.0, 1.0], lightness);
    add_vert(vertices, pos, [size, 0.0], [1.0, 0.0], lightness);

    add_vert(vertices, pos, [size, size], [1.0, 1.0], lightness);
    add_vert(vertices, pos, [0.0, 0.0], [0.0, 0.0], lightness);
    add_vert(vertices, pos, [0.0, size], [0.0, 1.0], lightness);
}

fn add_shadow(vertices: &mut Vec<MeshVertex>, p: Vec3, game: &GameState, size: f32) {
    for off_y in 0..8 {
        let y = p.y - off_y as f32;
        let pos = Vec3::new(p.x, y.floor(), p.z);
        if game.world().is_solid(pos) {
            let d = ((p.y - pos.y).abs() / 8.0).clamp(0.0, 1.0);
            let lightness = 1.0 - d;
            let size = size * (d + 0.75);
            add_quad(vertices, pos + Vec3::new(0.0, 1.0, 0.0), lightness, size);
            return;
        }
    }
}

fn prepare(args: &RenderPassArgs) -> Result<Mesh> {
    let mut vertices: Vec<MeshVertex> = vec![];
    let entity_positions = args.render_reactor.entity_pos();
    for p in entity_positions.iter() {
        add_shadow(&mut vertices, p.pos(), args.game, p.size());
    }
    let mesh = Mesh::from_vec(&args.fe.display, &vertices)?;
    Ok(mesh)
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    let program = ShaderList::new_point_program(
        &args.fe.display,
        include_str!("./shadow.vert"),
        include_str!("./shadow.frag"),
    )
    .expect("Couldn't compile shadow shader");

    let texture = Texture::from_bytes(&args.fe.display, include_bytes!("../assets/shadow.png"))
        .expect("Couldn't load shadow texture");

    args.render_reactor
        .post_world_render
        .push(Box::new(move |args| {
            let mesh = prepare(&args).expect("Couldn't create shadow mesh");
            let mvp = args.projection * args.view;
            let mat_mvp = mvp.to_cols_array_2d();
            let texture = texture.texture();
            args.frame
                .draw(
                    mesh.buffer(),
                    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program,
                    &uniform! {
                        mat_mvp: mat_mvp,
                        cur_tex: texture,
                    },
                    &glium::DrawParameters {
                        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        blend: glium::draw_parameters::Blend {
                            color: glium::draw_parameters::BlendingFunction::Addition {
                                source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                                destination:
                                    glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                            },
                            alpha: glium::draw_parameters::BlendingFunction::Addition {
                                source: glium::draw_parameters::LinearBlendingFactor::One,
                                destination:
                                    glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                            },
                            constant_value: (0.0, 0.0, 0.0, 0.0),
                        },
                        depth: glium::draw_parameters::Depth {
                            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                            write: false,
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
                )
                .expect("Couldn't render shadows");
            args
        }));
    args
}
