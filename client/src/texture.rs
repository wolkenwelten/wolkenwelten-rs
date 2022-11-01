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
use glium;
use std::ffi::{c_void, CString};

pub struct Texture {
    texture: glium::texture::SrgbTexture2d,
}

pub struct TextureArray {
    id: i32,
}

impl Texture {
    pub fn from_bytes(
        display: &glium::Display,
        label: &str,
        bytes: &'static [u8],
        linear: bool,
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes).unwrap();
        let img = img.flipv().to_rgba8();

        //let label = CString::new(label).unwrap();
        let image_dimensions = img.dimensions();
        let img =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, img).unwrap();
        Ok(Self { texture })
    }

    pub fn texture(&self) -> &glium::texture::SrgbTexture2d {
        &self.texture
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
        /*
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
         */
        let id = 23;
        Ok(Self { id })
    }

    pub fn bind(&self) {}
}
