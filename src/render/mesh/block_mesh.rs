use std::ffi::CString;
use super::BlockVertex;
use gl::types::{GLuint, GLvoid};


pub struct BlockMesh {
	vao: GLuint,
	vbo: GLuint,
	vertex_count: u16,
}

impl BlockMesh {
	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count.try_into().unwrap());
		}
	}

	pub fn new(vertices: &Vec<BlockVertex>) -> Result<Self, String> {
		let vbo_size = (vertices.len() * std::mem::size_of::<BlockVertex>()) as gl::types::GLsizeiptr;
		let vertex_count: GLuint = vertices.len().try_into().unwrap();
		let i:u32 = 0;

		let label = CString::new(format!("BlockMesh VAO {i}")).unwrap();
		let vao = unsafe {
			let mut vao = 0;
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			gl::ObjectLabel(gl::ARRAY_BUFFER, vao, -1, label.as_ptr());
			vao
		};

		let label = CString::new(format!("BlockMesh VBO {i}")).unwrap();
		let vbo = unsafe {
			let mut vbo:GLuint = 0;
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				vbo_size.try_into().unwrap(),
				vertices.as_ptr() as *const GLvoid,
				gl::STATIC_DRAW,
			);
			gl::ObjectLabel(gl::BUFFER, vbo, -1, label.as_ptr());
			BlockVertex::vertex_attrib_pointers();
			vbo
		};

		let _vbo_size:u16 = vbo_size.try_into().unwrap();
		let vertex_count:u16 = vertex_count.try_into().unwrap();
		Ok(Self { vao, vbo, vertex_count })
	}

	pub fn test_mesh() -> Self {
		let mut vertices:Vec<BlockVertex> = Vec::with_capacity(6);
		BlockVertex::add_front(&mut vertices, 0,0,0,16,16,16,0,0x0F);

		Self::new(&vertices).unwrap()
	}
}

impl Drop for BlockMesh {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &mut self.vbo);
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}
}
