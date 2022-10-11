use std::ffi::CString;

mod vertex;
mod mesh;
mod shader;
mod program;

pub use self::vertex::{f32_f32_f32, Vertex};
pub use self::shader::Shader;
pub use self::program::Program;
pub use self::mesh::Mesh;

pub fn refresh_gl() {
    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
    }
}

pub fn render_frame(i:i32) {
    let r:f32 = 0.4 + (((i as f64) / 100.0).sin() / 5.0) as f32;
    unsafe {
        gl::ClearColor(r, 0.5, 0.8, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
