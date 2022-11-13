// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Texture, TextureArray};
use anyhow::Result;

/// A colleciton of all the textures included with WW
#[derive(Debug)]
pub struct TextureList {
    pub blocks: TextureArray,
    pub gui: Texture,
    pub pear: Texture,
    pub sky: Texture,
}

impl TextureList {
    /// Initialize a new TextureList with all the textures initialized/loaded
    pub fn new(display: &glium::Display) -> Result<TextureList> {
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
