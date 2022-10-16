use gl::types::{GLuint, GLvoid};
use std::ffi::CString;

pub struct VBO {
	id: GLuint,
}

pub struct VAO {
	id: GLuint,
	_vbo: VBO,
}

impl VBO {
	pub fn buffer_data(vertices:*const GLvoid, vbo_size:u32) {
		if vbo_size > 0 {
			unsafe {
				gl::BufferData(
					gl::ARRAY_BUFFER, // target
					vbo_size.try_into().unwrap(), // size of data in bytes
					vertices, // pointer to data
					gl::STATIC_DRAW, // usage
				)
			}
		}
	}
	pub fn new(label:&str, vertices:*const GLvoid, vbo_size:u32) -> Self {
		let label = CString::new(format!("{label} VBO")).unwrap();
		let id: GLuint = unsafe {
			let mut vbo:GLuint = 0;
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::ObjectLabel(gl::BUFFER, vbo, -1, label.as_ptr());
			vbo
		};
		Self::buffer_data(vertices, vbo_size);
		Self { id }

	}
}

impl VAO {
	pub fn new(label:&str, vertices:*const GLvoid, vbo_size:u32) -> Self {
		let vao_label = CString::new(format!("{label} VAO")).unwrap();
		let id: GLuint = unsafe {
			let mut vao: GLuint = 0;
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			gl::ObjectLabel(gl::BUFFER, vao, -1, vao_label.as_ptr());
			vao
		};
		let vbo = VBO::new(label, vertices, vbo_size);
		Self { id, _vbo:vbo }
	}

	pub fn new_empty(label:&str) -> Self {
		Self::new(label, 0 as *const GLvoid, 0)
	}

	pub fn bind(&self) {
		unsafe { gl::BindVertexArray(self.id) }
	}

	pub fn draw(&self, vertex_count:u32) {
		self.bind();
		unsafe {
			gl::DrawArrays(gl::TRIANGLES, 0, vertex_count.try_into().unwrap());
		}
	}
}

impl Drop for VAO {
	fn drop(&mut self) {
		unsafe { gl::DeleteVertexArrays(1, &mut self.id) }
	}
}
impl Drop for VBO {
	fn drop(&mut self) {
		unsafe { gl::DeleteBuffers(1, &mut self.id) }
	}
}
