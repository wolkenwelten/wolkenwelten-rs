/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use glam::f32::{Vec2, Vec3};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
struct MeshVertex {
    pub pos: Vec3,
    pub tex: Vec2,
    pub c: f32,
}
/*
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
 */

#[derive(Debug, Default)]
pub struct Mesh {
    vertex_count: usize,
}

impl Mesh {
    pub fn draw(&self) {
        //self.vao.draw(self.vertex_count);
    }

    fn from_vec(vertices: &Vec<MeshVertex>) -> Result<Self, String> {
        let vbo_size: u32 = (vertices.len() * std::mem::size_of::<MeshVertex>())
            .try_into()
            .unwrap();
        //let vao = Vao::new("Block Mesh", vertices.as_ptr() as *const GLvoid, vbo_size);
        //MeshVertex::vertex_attrib_pointers();
        let vertex_count = vertices.len();
        Ok(Self { vertex_count })
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

        let vertices = m
            .indices
            .iter()
            .map(|i| {
                let idx: usize = *i as usize;
                MeshVertex {
                    pos: (
                        m.positions[idx * 3],
                        m.positions[idx * 3 + 1],
                        m.positions[idx * 3 + 2],
                    )
                        .into(),
                    tex: (m.texcoords[idx * 2], 1.0 - m.texcoords[idx * 2 + 1]).into(), // Gotta flip them around for some reason, might be a wrong config option in blender during export
                    c: 1.0,
                }
            })
            .collect();
        Self::from_vec(&vertices)
    }
}
