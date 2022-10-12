use gl::types::*;

use crate::render::mesh::Vertex2D;

pub struct TextMesh {
	vao: GLuint,
	vbo: GLuint,
	vertex_count: GLuint,

	finished: bool,
	vertices: Vec<Vertex2D>,
}

impl TextMesh {

	pub fn empty(&mut self) -> &mut TextMesh {
		self.vertices.clear();
		self.finished = false;
		self
	}

	pub fn prepare(&mut self) -> &mut TextMesh {
		if self.vao == 0 {
			unsafe {
				gl::GenVertexArrays(1, &mut self.vao);
				gl::BindVertexArray(self.vao);
			}
		}
		unsafe {
			gl::BindVertexArray(self.vao);
		}


		if self.vbo == 0 {
			unsafe {
				gl::GenBuffers(1, &mut self.vbo);
				gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
				Vertex2D::vertex_attrib_pointers();
			}
		} else {
			unsafe {
				gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			}
		}

		if !self.finished {
			self.vertex_count = self.vertices.len() as u32;
			let vbo_size = (self.vertices.len() * std::mem::size_of::<Vertex2D>()) as gl::types::GLsizeiptr;
			unsafe {
				gl::BufferData(
					gl::ARRAY_BUFFER,
					vbo_size.try_into().unwrap(), // size of data in bytes
					self.vertices.as_ptr() as *const gl::types::GLvoid,
					gl::STATIC_DRAW,
				);
			}
			self.finished = true;
		}

		self
	}

	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(
				gl::TRIANGLES,
				0,
				self.vertex_count.try_into().unwrap(),
			);
		}
	}

	pub fn new() -> TextMesh {
		TextMesh {
			vao: 0,
			vbo: 0,
			vertex_count: 0,
			finished: false,
			vertices: Vec::with_capacity(8),
		}
	}

	pub fn add_vertex(&mut self, x:i16, y:i16, u:i16, v:i16, rgba:u32) -> &mut TextMesh {
		self.vertices.push( Vertex2D { x,y,u,v,rgba });
		self
	}

	pub fn push_box(&mut self, x:i16, y:i16, w:i16, h:i16, u:i16, v:i16, uw:i16, vh:i16, rgba:u32) -> &mut TextMesh {
		self.add_vertex(x,y+h,u,v+vh,rgba)
			.add_vertex(x+w,y,u+uw,v,rgba)
			.add_vertex(x,y,u,v,rgba)

			.add_vertex(x+w,y,u+uw,v,rgba)
			.add_vertex(x,y+h,u,v+vh,rgba)
			.add_vertex(x+w,y+h,u+uw,v+vh,rgba)
	}
}

impl Drop for TextMesh {
	fn drop(&mut self) {
		unsafe {
			if self.vbo != 0 {
				gl::DeleteBuffers(1, &mut self.vbo);
			}
			if self.vao != 0 {
				gl::DeleteVertexArrays(1, &mut self.vao);
			}
		}
	}
}