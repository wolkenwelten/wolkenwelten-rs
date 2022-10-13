use gl::types::*;

use crate::render::mesh::Mesh_Vertex;

pub struct Mesh {
	vao: GLuint,
	vbo: GLuint,
	vertex_count: GLuint,
}

impl Mesh {
	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(
				gl::TRIANGLES,
				0, // starting index
				self.vertex_count.try_into().unwrap(),
			);
		}
	}

	pub fn from_vec(
		vertices: &Vec<Mesh_Vertex>
	) -> Result<Self, String> {
		let vbo_size = (vertices.len() * std::mem::size_of::<Mesh_Vertex>()) as gl::types::GLsizeiptr;
		let vertex_count: GLuint = vertices.len().try_into().unwrap();
		let mut vbo: gl::types::GLuint = 0;
		unsafe {
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER, // target
				vbo_size.try_into().unwrap(), // size of data in bytes
				vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
				gl::STATIC_DRAW, // usage
			);
		}

		let mut vao: gl::types::GLuint = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			Mesh_Vertex::vertex_attrib_pointers();
		}
		Ok(Self { vao, vbo, vertex_count })
	}

	pub fn from_obj_string(
		s: &str
	) -> Result<Self, String> {
		let o = tobj::load_obj_buf(
			&mut s.as_bytes(),
			&tobj::LoadOptions {
				triangulate: true,
				single_index: true,
				..Default::default()
			},
			|_p| unreachable!(),
		).unwrap().0;
		let m = &o[0].mesh;

		let mut vertices: Vec<Mesh_Vertex> = Vec::with_capacity(m.indices.len());
		for i in m.indices.iter() {
			let idx: usize = *i as usize;
			m.positions[idx];
			vertices.push(Mesh_Vertex {
				pos: (m.positions[idx * 3], m.positions[idx * 3 + 1], m.positions[idx * 3 + 2]).into(),
				tex: (m.texcoords[idx * 2], 1.0 - m.texcoords[idx * 2 + 1]).into(), // Gotta flip them around for some reason, might be a wrong config option in blender during export
				c: 1.0,
			});
		}
		Self::from_vec(&vertices)
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &mut self.vbo);
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}
}
