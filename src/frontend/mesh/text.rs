use gl::types::*;

use crate::frontend::mesh::{VAO, VBO, Vertex2D};

pub struct TextMesh {
	vao: VAO,
	vertex_count: GLuint,

	finished: bool,
	vertices: Vec<Vertex2D>,
}

impl TextMesh {

	pub fn empty(&mut self) -> &mut Self {
		self.vertices.clear();
		self.finished = false;
		self
	}

	pub fn prepare(&mut self) -> &mut Self {
		if !self.finished {
			self.vao.bind();
			self.vertex_count = self.vertices.len() as u32;
			let vbo_size = (self.vertices.len() * std::mem::size_of::<Vertex2D>()) as gl::types::GLsizeiptr;
			VBO::buffer_data(self.vertices.as_ptr() as *const GLvoid, vbo_size.try_into().unwrap());
			self.finished = true;
		}
		self
	}

	pub fn draw(&self) {
		self.vao.draw(self.vertex_count)
	}

	pub fn new() -> Self {
		let vao = VAO::new_empty("Text Mesh");
		Vertex2D::vertex_attrib_pointers();
		Self {
			vao,
			vertex_count: 0,
			finished: false,
			vertices: Vec::with_capacity(8),
		}
	}

	pub fn push_vertex(&mut self, x:i16, y:i16, u:i16, v:i16, rgba:u32) -> &mut Self {
		self.vertices.push( Vertex2D { x, y, u, v, rgba });
		self
	}

	pub fn push_box(&mut self, x:i16, y:i16, w:i16, h:i16, u:i16, v:i16, uw:i16, vh:i16, rgba:u32) -> &mut Self {
		self.push_vertex(x,y+h,u,v+vh,rgba)
			.push_vertex(x+w,y,u+uw,v,rgba)
			.push_vertex(x,y,u,v,rgba)

			.push_vertex(x+w,y,u+uw,v,rgba)
			.push_vertex(x,y+h,u,v+vh,rgba)
			.push_vertex(x+w,y+h,u+uw,v+vh,rgba)
	}

	pub fn push_glyph(&mut self, x:i16, y:i16, size:i16, rgba:u32, c:char) -> &mut Self {
		let glyph_width:i16 = (8 * size) as i16;

		if x < -glyph_width {return self;}
		if y < -glyph_width {return self;}
		if c == '\0' { return self; }
		if c == ' ' { return self; }

		let cc = c as u8;
		let u = 32 + ((cc & 0xF) as i16 * size);
		let voff = if size == 1 {128-16} else {128};
		let v = voff - ((((cc >> 4) & 0xF) + 1) as i16 * size);

		self.push_box(x, y, glyph_width, glyph_width, u, v, size, size, rgba)
	}

	pub fn push_string(&mut self, x:i16, y:i16, size:i32, rgba:u32, text:&str) -> &mut Self {
		let glyph_width:i32 = 8*size;
		for (i,c) in text.chars().enumerate() {
			let x:i16 = (x + ((i as i32)*glyph_width) as i16).try_into().unwrap();

			self.push_glyph(x, y, size as i16, rgba, c);
		}
		self
	}
}
