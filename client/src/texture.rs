// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glium::{
    texture::{RawImage2d, Texture2d, Texture2dArray},
    uniforms::Sampler,
};
use glutin::surface::WindowSurface;
use image::{DynamicImage, Rgba};
use wolkenwelten_core::BLOCKS;

#[derive(Debug)]
pub struct Texture {
    texture: glium::texture::Texture2d,
}

#[derive(Debug)]
pub struct TextureArray {
    texture: glium::texture::Texture2dArray,
}

impl Texture {
    pub fn from_image(display: &glium::Display<WindowSurface>, img: DynamicImage) -> Result<Self> {
        let img = img.to_rgba8();

        let image_dimensions = img.dimensions();
        let img = RawImage2d::from_raw_rgba(img.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, img)?;
        Ok(Self { texture })
    }

    pub fn from_bytes(display: &glium::Display<WindowSurface>, bytes: &'static [u8]) -> Result<Self> {
        Self::from_image(display, image::load_from_memory(bytes)?)
    }

    pub fn build_block_icons(
        block_bytes: &'static [u8],
        tile_size: u32,
    ) -> Result<Vec<DynamicImage>> {
        let bimg = image::load_from_memory(block_bytes)?;
        let bimg = bimg.to_rgba8();

        BLOCKS.with(|blocks| {
            let blocks = blocks.borrow();
            let mut ret = Vec::with_capacity(blocks.len());
            for block in blocks.iter() {
                let mut img = image::RgbaImage::new(tile_size, tile_size);
                {
                    // Top
                    let tex = block.tex_top() as u32;
                    let yoff = tex * tile_size * 2;
                    for x in 0..tile_size {
                        for y in 0..tile_size {
                            let pixel = bimg.get_pixel(x, y + yoff);
                            let x = tile_size / 2 + x / 2 - y / 2;
                            let y = x / 2 + y / 2 - tile_size / 4;
                            img.put_pixel(x, y, *pixel);
                        }
                    }
                }
                {
                    // Front
                    let tex = block.tex_front() as u32;
                    let yoff = tex * tile_size * 2;
                    for x in 0..tile_size {
                        for y in 0..tile_size {
                            let pixel = bimg.get_pixel(x, (tile_size - y) + yoff); // Gotta flip it because of reasons
                            let pixel = Rgba([
                                pixel.0[0] - pixel.0[0] / 6,
                                pixel.0[1] - pixel.0[1] / 6,
                                pixel.0[2] - pixel.0[2] / 6,
                                pixel.0[3],
                            ]);
                            let x = x / 2;
                            let y = tile_size / 4 + y / 2 + x / 2;
                            img.put_pixel(x, y, pixel);
                        }
                    }
                }
                {
                    // Right
                    let tex = block.tex_right() as u32;
                    let yoff = tex * tile_size * 2;
                    for x in 0..tile_size {
                        for y in 0..tile_size {
                            let pixel = bimg.get_pixel(x, (tile_size - y) + yoff); // Gotta flip it because of reasons
                            let pixel = Rgba([
                                pixel.0[0] - pixel.0[0] / 4,
                                pixel.0[1] - pixel.0[1] / 4,
                                pixel.0[2] - pixel.0[2] / 4,
                                pixel.0[3],
                            ]);
                            let x = tile_size / 2 + x / 2;
                            let y = tile_size / 4 + y / 2 + (tile_size - x) / 2;
                            img.put_pixel(x, y, pixel);
                        }
                    }
                }
                ret.push(img.into());
            }
            Ok(ret)
        })
    }

    pub fn gui_texture(
        display: &glium::Display<WindowSurface>,
        gui_bytes: &'static [u8],
        block_bytes: &'static [u8],
    ) -> Result<Self> {
        let img = image::load_from_memory(gui_bytes)?;
        let mut img = img.to_rgba8();
        let image_dimensions = img.dimensions();
        let tile_size = image_dimensions.0 / 32;

        let icons = Self::build_block_icons(block_bytes, tile_size)?;
        for (i, icon) in icons.iter().enumerate() {
            let x = (i as u32 % 32) * tile_size;
            let y = (i as u32 / 32) * tile_size;
            image::imageops::overlay(&mut img, icon, x as i64, y as i64);
        }

        let img = RawImage2d::from_raw_rgba(img.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, img)?;
        Ok(Self { texture })
    }

    pub fn texture(&self) -> &Texture2d {
        &self.texture
    }

    pub fn texture_nn(&self) -> Sampler<'_, glium::texture::Texture2d> {
        self.texture()
            .sampled()
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
            .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
    }
}

impl TextureArray {
    pub fn from_bytes(display: &glium::Display<WindowSurface>, bytes: &'static [u8]) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        let img = img.to_rgba8();
        let tile_size: u32 = img.width();
        let tile_count = img.height() / tile_size;

        let image_dimensions = (tile_size, tile_size);
        let tile_byte_size = (tile_size * tile_size * 4) as usize;
        let raw = &img.into_raw();

        let tiles = (0..tile_count)
            .map(|y| {
                let from = y as usize * tile_byte_size;
                let to = from + tile_byte_size;
                let raw = &raw[from..to];
                RawImage2d::from_raw_rgba_reversed(raw, image_dimensions)
            })
            .collect();

        let texture = Texture2dArray::new(display, tiles)?;

        Ok(Self { texture })
    }

    pub fn texture(&self) -> &Texture2dArray {
        &self.texture
    }
}
