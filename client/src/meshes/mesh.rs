// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::{Vec2, Vec3};
use glium;
use glium::{implement_vertex, uniform, Surface};
use wolkenwelten_common::BlockType;

use crate::Texture;

#[derive(Copy, Clone, Debug)]
pub struct MeshVertex {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
    pub lightness: f32,
}
implement_vertex!(MeshVertex, pos, tex, lightness);

impl MeshVertex {
    pub fn new(pos: Vec3, tex: Vec2, lightness: f32) -> Self {
        Self {
            pos: [pos.x, pos.y, pos.z],
            tex: [tex.x, tex.y],
            lightness,
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    buffer: glium::VertexBuffer<MeshVertex>,
}

impl Mesh {
    pub fn from_vec(display: &glium::Display, vertices: &Vec<MeshVertex>) -> Result<Self> {
        let buffer = glium::VertexBuffer::dynamic(display, vertices.as_slice())?;
        Ok(Self { buffer })
    }

    pub fn from_vec_static(display: &glium::Display, vertices: &Vec<MeshVertex>) -> Result<Self> {
        let buffer = glium::VertexBuffer::immutable(display, vertices.as_slice())?;
        Ok(Self { buffer })
    }

    pub fn buffer(&self) -> &glium::VertexBuffer<MeshVertex> {
        &self.buffer
    }

    pub fn add_block(
        vertices: &mut Vec<MeshVertex>,
        pos_off: Vec3,
        tex_off: Vec2,
        tex_scale: Vec2,
    ) {
        {
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 1.0, 0.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 0.0),
                tex_off + Vec2::new(tex_scale.x, 0.0),
                1.0,
            ));

            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 1.0, 0.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 0.0),
                tex_off + Vec2::new(0.0, tex_scale.y),
                1.0,
            ));
        }
        {
            let pos_off = pos_off + Vec3::new(0.0, 0.0, 1.0);
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 0.0),
                tex_off + Vec2::new(tex_scale.x, 0.0),
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 1.0, 0.0),
                tex_off + tex_scale,
                1.0,
            ));

            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 1.0, 0.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 0.0),
                tex_off + Vec2::new(0.0, tex_scale.y),
                1.0,
            ));
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
        }

        {
            let pos_off = pos_off + Vec3::new(0.0, 1.0, 0.0);
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 0.0),
                tex_off + Vec2::new(tex_scale.x, 0.0),
                1.0,
            ));

            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 0.0, 1.0),
                tex_off + Vec2::new(0.0, tex_scale.y),
                1.0,
            ));
        }
        {
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 0.0),
                tex_off + Vec2::new(tex_scale.x, 0.0),
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));

            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(1.0, 0.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 0.0, 1.0),
                tex_off + Vec2::new(0.0, tex_scale.y),
                1.0,
            ));
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
        }

        {
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 0.0, 1.0),
                tex_off + Vec2::new(tex_scale.x, 0.0),
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));

            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 0.0),
                tex_off + Vec2::new(0.0, tex_scale.y),
                1.0,
            ));
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
        }
        {
            let pos_off = pos_off + Vec3::new(1.0, 0.0, 0.0);
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 0.0, 1.0),
                tex_off + Vec2::new(tex_scale.x, 0.0),
                1.0,
            ));

            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 1.0),
                tex_off + tex_scale,
                1.0,
            ));
            vertices.push(MeshVertex::new(pos_off, tex_off, 1.0));
            vertices.push(MeshVertex::new(
                pos_off + Vec3::new(0.0, 1.0, 0.0),
                tex_off + Vec2::new(0.0, tex_scale.y),
                1.0,
            ));
        }
    }

    pub fn add_block_type(vertices: &mut Vec<MeshVertex>, block: &BlockType, tile_size: f32) {
        let mut tmp: Vec<MeshVertex> = vec![];
        Self::add_block(&mut tmp, Vec3::new(-0.5, -0.5, -0.5), Vec2::ZERO, Vec2::ONE);
        for (i, v) in tmp.iter_mut().enumerate() {
            let i = i / 6;
            let tex = block.tex()[i] as f32;
            let off = tex * tile_size;
            v.pos = v.pos.map(|f| f * 16.0);
            v.tex = [v.tex[0] * 0.5, off + (v.tex[1] * tile_size / 2.0)];
        }
        vertices.extend(tmp);
    }

    pub fn from_obj_string(display: &glium::Display, s: &str) -> Result<Self> {
        let o = tobj::load_obj_buf(
            &mut s.as_bytes(),
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |_p| unreachable!(),
        )?
        .0;
        let m = &o[0].mesh;

        let vertices = m
            .indices
            .iter()
            .map(|i| {
                let idx: usize = *i as usize;
                MeshVertex {
                    pos: [
                        m.positions[idx * 3],
                        m.positions[idx * 3 + 1],
                        m.positions[idx * 3 + 2],
                    ],
                    tex: [m.texcoords[idx * 2], 1.0 - m.texcoords[idx * 2 + 1]], // Gotta flip them around for some reason, might be a wrong config option in blender during export
                    lightness: 1.0,
                }
            })
            .collect();
        Self::from_vec_static(display, &vertices)
    }

    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        texture: &Texture,
        program: &glium::Program,
        mat_mvp: &glam::Mat4,
    ) -> Result<()> {
        let mat_mvp = mat_mvp.to_cols_array_2d();
        let texture = texture.texture_nn();
        frame.draw(
            self.buffer(),
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            program,
            &uniform! {
                mat_mvp: mat_mvp,
                cur_tex: texture,
            },
            &glium::DrawParameters {
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                blend: glium::draw_parameters::Blend::alpha_blending(),
                depth: glium::draw_parameters::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;
        Ok(())
    }
}
