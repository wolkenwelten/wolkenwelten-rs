use gl::types::{GLuint, GLvoid};
use glam::f32::{Vec2, Vec3};

use super::util;
use super::util::Vao;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
struct MeshVertex {
    pub pos: Vec3,
    pub tex: Vec2,
    pub c: f32,
}

impl MeshVertex {
    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();

        unsafe {
            let offset = util::vertex_attrib_pointer(stride, 0, 0, 3, gl::FLOAT, 4, false);
            let offset = util::vertex_attrib_pointer(stride, 1, offset, 2, gl::FLOAT, 4, true);
            util::vertex_attrib_pointer(stride, 2, offset, 1, gl::FLOAT, 4, false);
        }
    }
}

#[derive(Debug, Default)]
pub struct Mesh {
    vao: Vao,
    vertex_count: u32,
}

impl Mesh {
    pub fn draw(&self) {
        self.vao.draw(self.vertex_count);
    }

    fn from_vec(vertices: &Vec<MeshVertex>) -> Result<Self, String> {
        let vbo_size: u32 = (vertices.len() * std::mem::size_of::<MeshVertex>())
            .try_into()
            .unwrap();
        let vao = Vao::new("Block Mesh", vertices.as_ptr() as *const GLvoid, vbo_size);
        MeshVertex::vertex_attrib_pointers();
        let vertex_count: GLuint = vertices.len().try_into().unwrap();
        Ok(Self { vao, vertex_count })
    }

    pub fn from_obj_string(s: &str) -> Result<Self, String> {
        let o = tobj::load_obj_buf(
            &mut s.as_bytes(),
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |_p| unreachable!(),
        )
        .unwrap()
        .0;
        let m = &o[0].mesh;

        let mut vertices: Vec<MeshVertex> = Vec::with_capacity(m.indices.len());
        for i in m.indices.iter() {
            let idx: usize = *i as usize;
            vertices.push(MeshVertex {
                pos: (
                    m.positions[idx * 3],
                    m.positions[idx * 3 + 1],
                    m.positions[idx * 3 + 2],
                )
                    .into(),
                tex: (m.texcoords[idx * 2], 1.0 - m.texcoords[idx * 2 + 1]).into(), // Gotta flip them around for some reason, might be a wrong config option in blender during export
                c: 1.0,
            });
        }
        Self::from_vec(&vertices)
    }
}
