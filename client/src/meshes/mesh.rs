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
use glium;
use glium::implement_vertex;

#[derive(Copy, Clone, Debug)]
pub struct MeshVertex {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
    pub lightness: f32,
}
implement_vertex!(MeshVertex, pos, tex, lightness);

pub struct Mesh {
    buffer: glium::VertexBuffer<MeshVertex>,
}

impl Mesh {
    pub fn draw(&self) {
        //self.vao.draw(self.vertex_count);
    }

    fn from_vec(display: &glium::Display, vertices: &Vec<MeshVertex>) -> Result<Self, String> {
        let buffer = glium::VertexBuffer::dynamic(display, vertices.as_slice()).unwrap();
        Ok(Self { buffer })
    }

    pub fn buffer(&self) -> &glium::VertexBuffer<MeshVertex> {
        &self.buffer
    }

    pub fn from_obj_string(display: &glium::Display, s: &str) -> Result<Self, String> {
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
        Self::from_vec(display, &vertices)
    }
}
