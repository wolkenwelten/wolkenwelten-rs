// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glium;
use glium::implement_vertex;

#[derive(Debug)]
pub enum MeshCreationError {
    LoadError(tobj::LoadError),
    BufferCreationError(glium::vertex::BufferCreationError),
}

impl From<tobj::LoadError> for MeshCreationError {
    fn from(err: tobj::LoadError) -> Self {
        Self::LoadError(err)
    }
}
impl From<glium::vertex::BufferCreationError> for MeshCreationError {
    fn from(err: glium::vertex::BufferCreationError) -> Self {
        Self::BufferCreationError(err)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MeshVertex {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
    pub lightness: f32,
}
implement_vertex!(MeshVertex, pos, tex, lightness);

#[derive(Debug)]
pub struct Mesh {
    buffer: glium::VertexBuffer<MeshVertex>,
}

impl Mesh {
    fn from_vec(
        display: &glium::Display,
        vertices: &Vec<MeshVertex>,
    ) -> Result<Self, MeshCreationError> {
        let buffer = glium::VertexBuffer::persistent(display, vertices.as_slice())?;
        Ok(Self { buffer })
    }

    pub fn buffer(&self) -> &glium::VertexBuffer<MeshVertex> {
        &self.buffer
    }

    pub fn from_obj_string(display: &glium::Display, s: &str) -> Result<Self, MeshCreationError> {
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
        Ok(Self::from_vec(display, &vertices)?)
    }
}
