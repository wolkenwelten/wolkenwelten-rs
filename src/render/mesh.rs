use gl::types::*;

use crate::render::Mesh_Vertex;

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
	) -> Result<Mesh, String> {
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
			gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
		}

		let mut vao: gl::types::GLuint = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			Mesh_Vertex::vertex_attrib_pointers();
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}
		Ok(Mesh { vao, vbo, vertex_count })
	}

	pub fn from_obj_string(
		s: &str
	) -> Result<Mesh, String> {
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
				tex: (m.texcoords[idx * 2], m.texcoords[idx * 2 + 1]).into(),
				c: 1.0,
			});
		}
		Mesh::from_vec(&vertices)
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