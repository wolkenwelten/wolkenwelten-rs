use gl::types::*;

use crate::render::Colored_Mesh_Vertex;

pub struct ColoredMesh {
	vao: GLuint,
	vbo: GLuint,
	vertex_count: GLuint,
}

impl ColoredMesh {
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
		vertices: &Vec<Colored_Mesh_Vertex>
	) -> Result<ColoredMesh, String> {
		let vbo_size = (vertices.len() * std::mem::size_of::<Colored_Mesh_Vertex>()) as gl::types::GLsizeiptr;
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
			Colored_Mesh_Vertex::vertex_attrib_pointers();
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}
		Ok(ColoredMesh { vao, vbo, vertex_count })
	}
}

impl Drop for ColoredMesh {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &mut self.vbo);
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}
}