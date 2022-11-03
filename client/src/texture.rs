// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.

pub struct Texture {
    texture: glium::texture::SrgbTexture2d,
}

pub struct TextureArray {
    texture: glium::texture::SrgbTexture2dArray,
}

impl Texture {
    pub fn from_bytes(
        display: &glium::Display,
        bytes: &'static [u8],
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes).unwrap();
        let img = img.flipv().to_rgba8();

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
    pub fn from_bytes(
        display: &glium::Display,
        bytes: &'static [u8],
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?;
        let img = img.to_rgba8();
        let tile_size: u32 = img.width();
        let tile_count = img.height() / tile_size;

        let image_dimensions = (tile_size, tile_size);
        let tile_byte_size = (tile_size * tile_size * 4) as usize;
        let raw = &img.into_raw();

        let tiles = (0..tile_count).map(|y| {
            let from = y as usize * tile_byte_size;
            let to = from + tile_byte_size;
            let raw = &raw[from..to];
            glium::texture::RawImage2d::from_raw_rgba_reversed(raw, image_dimensions)
        }).collect();

        let texture = glium::texture::SrgbTexture2dArray::new(display, tiles).unwrap();

        Ok(Self { texture })
    }

    pub fn texture(&self) -> &glium::texture::SrgbTexture2dArray {
        &self.texture
    }

    pub fn bind(&self) {}
}
