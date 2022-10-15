use std;
use std::ffi::CString;

use gl;
use gl::types::{GLint, GLuint};
use glam::{Mat4, Vec3, Vec4};

use super::shader::Shader;

fn cstr_from_string (s:String) -> CString {
	CString::new(s).unwrap()
}

pub struct Program {
	id: GLuint,
	location_mvp: GLint,
	location_trans: GLint,
	location_color: GLint,
	location_alpha: GLint,
}

fn bind_attrib_locations(program: GLuint) {
	unsafe {
		gl::BindAttribLocation(program, 0, cstr_from_string("pos".to_string()).as_ptr());
		gl::BindAttribLocation(program, 1, cstr_from_string("tex".to_string()).as_ptr());
		gl::BindAttribLocation(program, 1, cstr_from_string("textureIndex".to_string()).as_ptr());
		gl::BindAttribLocation(program, 2, cstr_from_string("lval".to_string()).as_ptr());
		gl::BindAttribLocation(program, 2, cstr_from_string("color".to_string()).as_ptr());
		gl::BindAttribLocation(program, 2, cstr_from_string("packedSideAndLight".to_string()).as_ptr());
	}
}

impl Program {
	pub fn from_shaders(program_label: &str, shaders: &[Shader]) -> Result<Self, String> {
		let program_id:GLuint = unsafe { gl::CreateProgram() };

		for shader in shaders {
			unsafe { gl::AttachShader(program_id, shader.id()); }
		}

		bind_attrib_locations(program_id);

		let label = CString::new(program_label).unwrap();
		unsafe {
			gl::ObjectLabel(gl::PROGRAM, program_id, -1, label.as_ptr());
			gl::LinkProgram(program_id);
		}

		let mut success: gl::types::GLint = 1;
		unsafe {
			gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
		}

		if success == 0 {
			let mut len: gl::types::GLint = 0;
			unsafe {
				gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
			}

			let error = super::create_whitespace_cstring_with_len(len as usize);

			unsafe {
				gl::GetProgramInfoLog(
					program_id,
					len,
					std::ptr::null_mut(),
					error.as_ptr() as *mut gl::types::GLchar,
				);
			}

			return Err(error.to_string_lossy().into_owned());
		}

		for shader in shaders {
			unsafe { gl::DetachShader(program_id, shader.id()); }
		}
		let name: CString = CString::new("matMVP").expect("CString::new failed");
		let location_mvp: GLint = unsafe { gl::GetUniformLocation(program_id, name.as_ptr()) };

		let name: CString = CString::new("transPos").expect("CString::new failed");
		let location_trans: GLint = unsafe { gl::GetUniformLocation(program_id, name.as_ptr()) };

		let name: CString = CString::new("inColor").expect("CString::new failed");
		let location_color: GLint = unsafe { gl::GetUniformLocation(program_id, name.as_ptr()) };

		let name: CString = CString::new("colorAlpha").expect("CString::new failed");
		let location_alpha: GLint = unsafe { gl::GetUniformLocation(program_id, name.as_ptr()) };

		Ok(Self {
			id: program_id,
			location_mvp,
			location_trans,
			location_color,
			location_alpha,
		})
	}

	pub fn from_shader_sources(program_label:&str, vert_source: &str, frag_source: &str) -> Result<Self, String> {
		let vert_shader = Shader::from_vert_source(vert_source).unwrap();
		let frag_shader = Shader::from_frag_source(frag_source).unwrap();
		Self::from_shaders(program_label, &[vert_shader, frag_shader])
	}

	pub fn set_used(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
	}

	pub fn set_mvp(&self, mvp: &Mat4) {
		unsafe {
			gl::UniformMatrix4fv(self.location_mvp, 1, gl::FALSE, mvp.to_cols_array().as_ptr());
		}
	}

	pub fn set_trans(&self, trans: &Vec3) {
		unsafe {
			gl::Uniform3f(self.location_trans, trans.x, trans.y, trans.z);
		}
	}

	pub fn set_color(&self, c: &Vec4) {
		unsafe {
			gl::Uniform4f(self.location_color, c.x, c.y, c.z, c.w);
		}
	}

	pub fn set_alpha(&self, c: f32) {
		unsafe {
			gl::Uniform1f(self.location_alpha, c);
		}
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}
