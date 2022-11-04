// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Texture, TextureArray, TextureLoadError};

#[derive(Debug)]
pub struct TextureList {
    pub blocks: TextureArray,
    pub gui: Texture,
    pub pear: Texture,
    pub sky: Texture,
}

#[derive(Debug)]
pub enum TextureListLoadError {
    TextureLoadError(TextureLoadError),
}
impl From<TextureLoadError> for TextureListLoadError {
    fn from(err: TextureLoadError) -> Self {
        Self::TextureLoadError(err)
    }
}

impl TextureList {
    pub fn new(display: &glium::Display) -> Result<TextureList, TextureListLoadError> {
        let blocks = TextureArray::from_bytes(
            display,
            include_bytes!("../../../assets/textures/blocks.png"),
        )?;
        let gui = Texture::from_bytes(display, include_bytes!("../../../assets/textures/gui.png"))?;
        let sky = Texture::from_bytes(display, include_bytes!("../../../assets/textures/sky.png"))?;
        let pear: Texture =
            Texture::from_bytes(display, include_bytes!("../../../assets/textures/pear.png"))?;
        Ok(TextureList {
            blocks,
            gui,
            pear,
            sky,
        })
    }
}
