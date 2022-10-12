use std;
use std::ffi::CString;

use gl;
use gl::types::{GLint, GLuint};
use glam::{Mat4, Vec4};

use super::shader::Shader;

pub struct Program {
	id: GLuint,
	location_mvp: GLint,
	location_color: GLint,
}

impl Program {
	pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
		let program_id = unsafe { gl::CreateProgram() };

		for shader in shaders {
			unsafe { gl::AttachShader(program_id, shader.id()); }
		}

		unsafe { gl::LinkProgram(program_id); }

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

		let name: CString = CString::new("inColor").expect("CString::new failed");
		let location_color: GLint = unsafe { gl::GetUniformLocation(program_id, name.as_ptr()) };

		Ok(Program {
			id: program_id,
			location_mvp,
			location_color,
		})
	}

	pub fn from_shader_sources(vert_source: &str, frag_source: &str) -> Result<Program, String> {
		let vert_shader = Shader::from_vert_source(&CString::new(vert_source).unwrap()).unwrap();
		let frag_shader = Shader::from_frag_source(&CString::new(frag_source).unwrap()).unwrap();
		Program::from_shaders(&[vert_shader, frag_shader])
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

	pub fn set_color(&self, c: &Vec4) {
		unsafe {
			gl::Uniform4f(self.location_color, c.x, c.y, c.z, c.w);
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