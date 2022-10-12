use gl;

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
pub struct Colored_Mesh_Vertex {
    pub pos: f32_f32_f32,
    pub clr: f32_f32_f32,
}

impl Colored_Mesh_Vertex {
    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();

        unsafe {
            let offset = 0;
            let offset = vertex_attrib_pointer(stride, 0, offset, 3);
            vertex_attrib_pointer(stride, 1, offset, 3);
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Mesh_Vertex {
    pub pos: f32_f32_f32,
    pub tex: f32_f32,
    pub c: f32,
}

impl Mesh_Vertex {
    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();

        unsafe {
            let offset = 0;
            let offset = vertex_attrib_pointer(stride, 0, offset, 3);
            let offset = vertex_attrib_pointer(stride, 1, offset, 2);
            vertex_attrib_pointer(stride, 2, offset, 1);
        }
    }
}

unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize, components: usize) -> usize {
    gl::EnableVertexAttribArray(location as gl::types::GLuint);
    gl::VertexAttribPointer(
        location as gl::types::GLuint,
        components as i32, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        stride as gl::types::GLint,
        offset as *const gl::types::GLvoid
    );
    offset + (components * 4)
}