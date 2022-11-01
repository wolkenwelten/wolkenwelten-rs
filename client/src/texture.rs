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
        let mut tiles = vec![];
        let tile_byte_size = (tile_size * tile_size * 4) as usize;
        let raw = &img.into_raw();
        for y in 0..tile_count {
            let from = y as usize * tile_byte_size;
            let to = from + tile_byte_size;
            let raw = &raw[from..to];
            let img = glium::texture::RawImage2d::from_raw_rgba_reversed(raw, image_dimensions);
            tiles.push(img);
        }
        let texture = glium::texture::SrgbTexture2dArray::new(display, tiles).unwrap();

        Ok(Self { texture })
    }

    pub fn texture(&self) -> &glium::texture::SrgbTexture2dArray {
        &self.texture
    }

    pub fn bind(&self) {}
}
