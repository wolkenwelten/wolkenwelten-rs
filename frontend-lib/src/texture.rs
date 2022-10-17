use std::ffi::{c_void, CString};

use gl;

pub struct Texture {
    id: gl::types::GLuint,
}
pub struct TextureArray {
    id: gl::types::GLuint,
}

impl Texture {
    pub fn from_bytes(label: &str, bytes: &'static [u8]) -> Result<Self, image::ImageError> {
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
            gl::ObjectLabel(gl::TEXTURE, id, -1, label.as_ptr());
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST.try_into().unwrap(),
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
        let mut img = image::load_from_memory(bytes)?;
        let tile_size: u32 = (img.width() / 16).try_into().unwrap();

        let mut atlas = image::RgbaImage::new(tile_size, tile_size * 256);
        for y in 0..16 {
            for x in 0..16 {
                let i = x + (y * 16);
                let tile = img.crop(x * tile_size, y * tile_size, tile_size, tile_size);
                let tile = tile.flipv();
                image::imageops::replace(&mut atlas, &tile, 0, (i * tile_size).into());
            }
        }

        let img = image::DynamicImage::from(atlas);
        let img = match img {
            image::DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba8(),
        };

        let label = CString::new(label).unwrap();
        let id = unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, id);
            gl::ObjectLabel(gl::TEXTURE, id, -1, label.as_ptr());
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
                256,
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
