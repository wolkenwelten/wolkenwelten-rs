use gl;

unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize, components: usize, data_type: u32, size: usize, normalized: bool) -> usize {
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

unsafe fn vertex_attrib_int_pointer(stride: usize, location: usize, offset: usize, data_type: u32, size: usize) -> usize {
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

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct BlockVertex {
	xyz: u16, // We've got 1 bit left here
	texture_index: u8, // Right now we don't really use 256 distinct block faces, ~32 should suffice for a long time
	side_and_light: u8, // And another one here as well
}

#[derive(Copy, Clone)]
pub enum Side {
	Front = 0,
	Back,
	Top,
	Bottom,
	Left,
	Right,
}

impl From<Side> for u8 {
	fn from(item: Side) -> Self { item as u8 }
}

impl BlockVertex {
	pub fn new(x:u16, y:u16, z:u16, texture_index:u8, side:u8, light:u8) -> Self {
		let xyz:u16 = x | (y << 5) | (z << 10);
		let side_and_light = side | (light << 4);
		Self { xyz, texture_index, side_and_light }
	}

	pub fn add_front(vertices:&mut Vec<Self>, x:u16, y:u16, z:u16, w:u16, h:u16, _d:u16, texture_index:u8, light:u8) {
		let side:u8 = Side::Front.into();
		vertices.push(Self::new(x,y,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y,z,texture_index,side,light));
		vertices.push(Self::new(x+w,y+h,z,texture_index,side,light));

		vertices.push(Self::new(x+w,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x,y+h,z,texture_index,side,light));
		vertices.push(Self::new(x,y,z,texture_index,side,light));
	}

	pub fn vertex_attrib_pointers() {
		let stride = std::mem::size_of::<Self>();
		unsafe {
			let offset = vertex_attrib_int_pointer(stride, 0, 0, gl::UNSIGNED_SHORT, 2);
			let offset = vertex_attrib_int_pointer(stride, 1, offset,gl::UNSIGNED_BYTE, 1);
			vertex_attrib_int_pointer(stride, 2, offset, gl::UNSIGNED_BYTE, 1);
		}
	}
}
