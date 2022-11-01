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
use std::mem;
use wgpu;
use wgpu::util::DeviceExt;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct MeshVertex {
    pub pos: Vec3,
    pub tex: Vec2,
    pub c: f32,
}

unsafe impl bytemuck::Pod for MeshVertex {}
unsafe impl bytemuck::Zeroable for MeshVertex {}

pub struct Mesh {
    buffer: wgpu::Buffer,
    vertices: Vec<MeshVertex>,
}

impl Mesh {
    pub fn draw(&self) {
        //lf.vao.draw(self.vertex_count);
    }

    pub fn buf(&self) -> &wgpu::Buffer {
        &self.buffer
    }
    pub fn vertex_count(&self) -> u32 {
        self.vertices.len() as u32
    }

    fn from_vec(device: &wgpu::Device, vertices: Vec<MeshVertex>) -> Result<Self, String> {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        Ok(Self { buffer, vertices })
    }

    pub fn from_obj_string(device: &wgpu::Device, s: &str) -> Result<Self, String> {
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
        Self::from_vec(device, vertices)
    }

    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}
