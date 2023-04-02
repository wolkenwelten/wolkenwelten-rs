// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Texture, TextureArray};
use anyhow::Result;
use glutin::surface::WindowSurface;

/// A colleciton of all the textures included with WW
#[derive(Debug)]
pub struct TextureList {
    pub blocks: TextureArray,
    pub fluids: TextureArray,
    pub blocks_raw: Texture,
    pub gui: Texture,
}

impl TextureList {
    /// Initialize a new TextureList with all the textures initialized/loaded
    pub fn new(display: &glium::Display<WindowSurface>) -> Result<TextureList> {
        let block_bytes = include_bytes!("../../assets/blocks.png");
        let blocks = TextureArray::from_bytes(display, block_bytes)?;
        let fluids = TextureArray::from_bytes(display, include_bytes!("../../assets/fluids.png"))?;
        let blocks_raw = Texture::from_bytes(display, block_bytes)?;
        let gui =
            Texture::gui_texture(display, include_bytes!("../../assets/gui.png"), block_bytes)?;
        Ok(TextureList {
            blocks,
            blocks_raw,
            fluids,
            gui,
        })
    }
}
