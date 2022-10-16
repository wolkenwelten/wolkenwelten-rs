use gl;

pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize, components: usize, data_type: u32, size: usize, normalized: bool) -> usize {
	gl::EnableVertexAttribArray(location as gl::types::GLuint);
	gl::VertexAttribPointer(
		location as gl::types::GLuint,
		components as i32, // the number of components per generic vertex attribute
		data_type, // data type
		if normalized { gl::TRUE } else { gl::FALSE }, // normalized (int-to-float conversion)
		stride as gl::types::GLint,
		offset as *const gl::types::GLvoid,
	);
	offset + (size * components)
}

pub unsafe fn vertex_attrib_int_pointer(stride: usize, location: usize, offset: usize, data_type: u32, size: usize) -> usize {
	gl::EnableVertexAttribArray(location as gl::types::GLuint);
	gl::VertexAttribIPointer(
		location as gl::types::GLuint,
		1,
		data_type, // data type
		stride as gl::types::GLint,
		offset as *const gl::types::GLvoid,
	);
	offset + size
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
	pub d0: f32,
	pub d1: f32,
	pub d2: f32,
}

impl f32_f32_f32 {
	pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
		f32_f32_f32 { d0, d1, d2 }
	}
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
	fn from(other: (f32, f32, f32)) -> Self {
		f32_f32_f32::new(other.0, other.1, other.2)
	}
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32 {
	pub d0: f32,
	pub d1: f32,
}

impl f32_f32 {
	pub fn new(d0: f32, d1: f32) -> f32_f32 {
		f32_f32 { d0, d1 }
	}
}

impl From<(f32, f32)> for f32_f32 {
	fn from(other: (f32, f32)) -> Self {
		f32_f32::new(other.0, other.1)
	}
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct MeshVertex {
	pub pos: f32_f32_f32,
	pub tex: f32_f32,
	pub c: f32,
}

impl MeshVertex {
	pub fn vertex_attrib_pointers() {
		let stride = std::mem::size_of::<Self>();

		unsafe {
			let offset = vertex_attrib_pointer(stride, 0, 0, 3, gl::FLOAT, 4, false);
			let offset = vertex_attrib_pointer(stride, 1, offset, 2, gl::FLOAT, 4, true);
			vertex_attrib_pointer(stride, 2, offset, 1, gl::FLOAT, 4, false);
		}
	}
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex2D {
	pub x:i16,
	pub y:i16,

	pub u:i16,
	pub v:i16,

	pub rgba:u32,
}

impl Vertex2D {
	pub fn vertex_attrib_pointers() {
		let stride = std::mem::size_of::<Self>();

		unsafe {
			let offset = vertex_attrib_pointer(stride, 0, 0, 2, gl::SHORT, 2, false);
			let offset = vertex_attrib_pointer(stride, 1, offset, 2, gl::SHORT, 2, false);
			vertex_attrib_pointer(stride, 2, offset, 4, gl::UNSIGNED_BYTE, 1, true);
		}
	}
}
