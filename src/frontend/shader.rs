use gl;
use gl::types::{GLint, GLuint};
use glam::Mat4;
use std;
use std::ffi::{CStr, CString};

pub struct Program {
    id: GLuint,
    location_mvp: GLint,
    location_trans: GLint,
    location_color: GLint,
    location_alpha: GLint,
}

struct Shader {
    id: GLuint,
}

const VERT_PREFIX: &str = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
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
#line 0"
};

const FRAG_PREFIX: &str = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
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
#line 0"
};

impl Shader {
    fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Self, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Self { id })
    }

    fn from_vert_source(source: &str) -> Result<Self, String> {
        let s: String = format!("{VERT_PREFIX}\n{source}");
        Self::from_source(&cstr_from_string(s), gl::VERTEX_SHADER)
    }

    fn from_frag_source(source: &str) -> Result<Self, String> {
        let s: String = format!("{FRAG_PREFIX}\n{source}");
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

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
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

fn cstr_from_string(s: String) -> CString {
    CString::new(s).unwrap()
}

impl Program {
    fn bind_attrib_locations(program: GLuint) {
        unsafe {
            gl::BindAttribLocation(program, 0, cstr_from_string("pos".to_string()).as_ptr());
            gl::BindAttribLocation(program, 1, cstr_from_string("tex".to_string()).as_ptr());
            gl::BindAttribLocation(
                program,
                1,
                cstr_from_string("textureIndex".to_string()).as_ptr(),
            );
            gl::BindAttribLocation(program, 2, cstr_from_string("lval".to_string()).as_ptr());
            gl::BindAttribLocation(program, 2, cstr_from_string("color".to_string()).as_ptr());
            gl::BindAttribLocation(
                program,
                2,
                cstr_from_string("packedSideAndLight".to_string()).as_ptr(),
            );
        }
    }

    fn from_shaders(program_label: &str, shaders: &[Shader]) -> Result<Self, String> {
        let program_id: GLuint = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        Program::bind_attrib_locations(program_id);

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
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
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

    pub fn from_shader_sources(
        program_label: &str,
        vert_source: &str,
        frag_source: &str,
    ) -> Result<Self, String> {
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
            gl::UniformMatrix4fv(
                self.location_mvp,
                1,
                gl::FALSE,
                mvp.to_cols_array().as_ptr(),
            );
        }
    }

    pub fn set_trans(&self, trans_x: f32, trans_y: f32, trans_z: f32) {
        unsafe { gl::Uniform3f(self.location_trans, trans_x, trans_y, trans_z) }
    }

    pub fn set_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Uniform4f(self.location_color, r, g, b, a);
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
