/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use crate::render;
use gl;
use gl::types::{GLint, GLuint, GLvoid};
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct Vbo {
    id: GLuint,
}

#[derive(Debug, Default)]
pub struct Vao {
    id: GLuint,
    vbo: Vbo,
}

impl Vbo {
    pub fn buffer_data(vertices: *const GLvoid, vbo_size: u32) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,             // target
                vbo_size.try_into().unwrap(), // size of data in bytes
                vertices,                     // pointer to data
                gl::STATIC_DRAW,              // usage
            )
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn bind_element(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn new(label: &str, vertices: *const GLvoid, vbo_size: u32) -> Self {
        let id: GLuint = unsafe {
            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            vbo
        };
        let label = CString::new(format!("{} VBO {}", label, id)).unwrap();
        if render::can_use_object_labels() {
            unsafe {
                gl::ObjectLabel(gl::BUFFER, id, -1, label.as_ptr());
            }
        }
        Self::buffer_data(vertices, vbo_size);
        Self { id }
    }
}

impl Vao {
    pub fn new(label: &str, vertices: *const GLvoid, vbo_size: u32) -> Self {
        let id: GLuint = unsafe {
            let mut vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            vao
        };
        let vao_label = CString::new(format!("{} VAO {}", label, id)).unwrap();
        if render::can_use_object_labels() {
            unsafe {
                gl::ObjectLabel(gl::VERTEX_ARRAY, id, -1, vao_label.as_ptr());
            }
        }
        let vbo = Vbo::new(label, vertices, vbo_size);
        Self { id, vbo }
    }

    pub fn new_empty(label: &str) -> Self {
        Self::new(label, std::ptr::null::<GLvoid>(), 0)
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
        self.vbo.bind();
    }

    pub fn draw(&self, vertex_count: u32) {
        self.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, vertex_count.try_into().unwrap());
        }
    }

    pub fn draw_elements(&self, start_offset: u32, index_count: u32) {
        self.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                index_count.try_into().unwrap(),
                gl::UNSIGNED_SHORT,
                start_offset as *const GLvoid,
            );
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}
impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

pub unsafe fn vertex_attrib_pointer(
    stride: usize,
    location: usize,
    offset: usize,
    components: usize,
    data_type: u32,
    size: usize,
    normalized: bool,
) -> usize {
    gl::EnableVertexAttribArray(location as GLuint);
    gl::VertexAttribPointer(
        location as GLuint,
        components as i32, // the number of components per generic vertex attribute
        data_type,         // data type
        if normalized { gl::TRUE } else { gl::FALSE }, // normalized (int-to-float conversion)
        stride as GLint,
        offset as *const GLvoid,
    );
    offset + (size * components)
}

pub unsafe fn vertex_attrib_int_pointer(
    stride: usize,
    location: usize,
    offset: usize,
    data_type: u32,
    size: usize,
    components: GLint,
) -> usize {
    gl::EnableVertexAttribArray(location as GLuint);
    gl::VertexAttribIPointer(
        location as GLuint,
        components,
        data_type, // data type
        stride as GLint,
        offset as *const GLvoid,
    );
    offset + size
}
