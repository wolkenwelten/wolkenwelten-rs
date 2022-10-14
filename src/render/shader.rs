use std;
use std::ffi::{CStr, CString};

use gl;
use gl::types::GLuint;

pub struct Shader {
	id:GLuint,
}

fn cstr_from_string (s:String) -> CString {
	CString::new(s).unwrap()
}

const VERT_PREFIX:&str = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
	r"#version 300 es
precision mediump float;
precision mediump int;

#line 0"
} else if cfg!(target_os = "macos") {
	r"#version 330 core
precision mediump float;
precision mediump int;

#line 0"
} else {
	r"#version 130
#line 0"};


const FRAG_PREFIX:&str = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
	r"#version 300 es
precision mediump float;
precision mediump int;
precision mediump sampler2DArray;

#line 0"
} else if cfg!(target_os = "macos") {
	r"#version 330 core
precision mediump float;
precision mediump int;
precision mediump sampler2DArray;

#line 0"
} else {
	r"#version 130
#line 0"}
;


impl Shader {
	pub fn from_source(
		source: &CStr,
		kind: gl::types::GLenum,
	) -> Result<Self, String> {
		let id = shader_from_source(source, kind)?;
		Ok(Self { id })
	}

	pub fn from_vert_source(source: &str) -> Result<Self, String> {
		let s:String = format!("{VERT_PREFIX}\n{source}");
		Self::from_source(&cstr_from_string(s), gl::VERTEX_SHADER)
	}

	pub fn from_frag_source(source: &str) -> Result<Self, String> {
		let s:String = format!("{FRAG_PREFIX}\n{source}");
		Self::from_source(&cstr_from_string(s), gl::FRAGMENT_SHADER)
	}

	pub fn id(&self) -> gl::types::GLuint {
		self.id
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.id);
		}
	}
}

fn shader_from_source(
	source: &CStr,
	kind: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
	let id = unsafe { gl::CreateShader(kind) };
	unsafe {
		gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
		gl::CompileShader(id);
	}

	let mut success: gl::types::GLint = 1;
	unsafe {
		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
	}

	if success == 0 {
		let mut len: gl::types::GLint = 0;
		unsafe {
			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
		}

		let error = super::create_whitespace_cstring_with_len(len as usize);

		unsafe {
			gl::GetShaderInfoLog(
				id,
				len,
				std::ptr::null_mut(),
				error.as_ptr() as *mut gl::types::GLchar,
			);
		}
		Err(error.to_string_lossy().into_owned())
	} else {
		Ok(id)
	}
}
