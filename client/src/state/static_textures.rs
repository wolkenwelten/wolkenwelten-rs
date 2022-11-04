// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Texture, TextureArray};

#[derive(Debug)]
pub struct TextureList {
    pub blocks: TextureArray,
    pub gui: Texture,
    pub pear: Texture,
    pub sky: Texture,
}

impl TextureList {
    pub fn new(display: &glium::Display) -> TextureList {
        let blocks =
            TextureArray::from_bytes(display, include_bytes!("../../../assets/textures/blocks.png"))
                .unwrap();
        let gui =
            Texture::from_bytes(display, include_bytes!("../../../assets/textures/gui.png")).unwrap();
        let sky =
            Texture::from_bytes(display, include_bytes!("../../../assets/textures/sky.png")).unwrap();
        let pear: Texture =
            Texture::from_bytes(display, include_bytes!("../../../assets/textures/pear.png")).unwrap();
        TextureList {
            blocks,
            gui,
            pear,
            sky,
        }
    }
}
