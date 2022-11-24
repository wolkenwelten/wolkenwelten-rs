// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Texture, TextureArray};
use anyhow::Result;
use wolkenwelten_game::GameState;

/// A colleciton of all the textures included with WW
#[derive(Debug)]
pub struct TextureList {
    pub blocks: TextureArray,
    pub blocks_raw: Texture,
    pub mining: Texture,
    pub gui: Texture,
    pub sky: Texture,
    pub shadow: Texture,
}

impl TextureList {
    /// Initialize a new TextureList with all the textures initialized/loaded
    pub fn new(display: &glium::Display, game: &GameState) -> Result<TextureList> {
        let block_bytes = include_bytes!("../../../assets/textures/blocks.png");
        let blocks = TextureArray::from_bytes(display, block_bytes)?;
        let blocks_raw = Texture::from_bytes(display, block_bytes)?;
        let gui = Texture::gui_texture(
            display,
            include_bytes!("../../../assets/textures/gui.png"),
            block_bytes,
            game,
        )?;
        let shadow = Texture::from_bytes(
            display,
            include_bytes!("../../../assets/textures/shadow.png"),
        )?;
        let sky = Texture::from_bytes(display, include_bytes!("../../../assets/textures/sky.png"))?;
        let mining = Texture::from_bytes(
            display,
            include_bytes!("../../../assets/textures/mining.png"),
        )?;
        Ok(TextureList {
            blocks,
            blocks_raw,
            gui,
            mining,
            shadow,
            sky,
        })
    }
}
