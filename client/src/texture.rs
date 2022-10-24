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
use std::ffi::{c_void, CString};
use gl::types::GLuint;

#[derive(Debug, Default)]
pub struct Texture {
    id: GLuint,
}
#[derive(Debug, Default)]
pub struct TextureArray {
    id: GLuint,
}

impl Texture {
    pub fn from_bytes(
        label: &str,
        bytes: &'static [u8],
        linear: bool,
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?;
        let width: u16 = img.width().try_into().unwrap();
        let height: u16 = img.height().try_into().unwrap();

        let img = match img {
            image::DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba8(),
        };

        let label = CString::new(label).unwrap();
        let id = unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            if render::can_use_object_labels() {
                gl::ObjectLabel(gl::TEXTURE, id, -1, label.as_ptr());
            }
            let filter = if linear { gl::LINEAR } else { gl::NEAREST };
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                filter.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                filter.try_into().unwrap(),
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA.try_into().unwrap(),
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                (&img as &[u8]).as_ptr() as *const c_void,
            );
            id
        };
        Ok(Self { id })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl TextureArray {
    pub fn from_bytes(label: &str, bytes: &'static [u8]) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?;
        let tile_size: u32 = img.width();
        let tile_count = img.height() / tile_size;

        let img = match img {
            image::DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba8(),
        };

        let label = CString::new(label).unwrap();
        let id = unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, id);
            if render::can_use_object_labels() {
                gl::ObjectLabel(gl::TEXTURE, id, -1, label.as_ptr());
            }
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST.try_into().unwrap(),
            );
            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA.try_into().unwrap(),
                tile_size.try_into().unwrap(),
                tile_size.try_into().unwrap(),
                tile_count.try_into().unwrap(),
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                (&img as &[u8]).as_ptr() as *const c_void,
            );
            id
        };
        Ok(Self { id })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for TextureArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, std::ptr::addr_of_mut!(self.id));
        }
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, std::ptr::addr_of_mut!(self.id));
        }
    }
}
