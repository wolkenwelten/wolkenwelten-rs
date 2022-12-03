// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Texture, TextureArray};
use anyhow::Result;
use wolkenwelten_core::GameState;

/// A colleciton of all the textures included with WW
#[derive(Debug)]
pub struct TextureList {
    pub blocks: TextureArray,
    pub blocks_raw: Texture,
    pub gui: Texture,
}

impl TextureList {
    /// Initialize a new TextureList with all the textures initialized/loaded
    pub fn new(display: &glium::Display, game: &GameState) -> Result<TextureList> {
        let block_bytes = include_bytes!("../../assets/blocks.png");
        let blocks = TextureArray::from_bytes(display, block_bytes)?;
        let blocks_raw = Texture::from_bytes(display, block_bytes)?;
        let gui = Texture::gui_texture(
            display,
            include_bytes!("../../assets/gui.png"),
            block_bytes,
            game,
        )?;
        Ok(TextureList {
            blocks,
            blocks_raw,
            gui,
        })
    }
}
