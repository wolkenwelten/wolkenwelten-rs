use crate::{Texture, TextureArray};

pub struct TextureList {
    pub blocks: TextureArray,
    pub gui: Texture,
    pub pear: Texture,
}

impl TextureList {
    pub fn new() -> TextureList {
        let blocks = TextureArray::from_bytes(
            "Blocks Texture",
            include_bytes!("../assets/textures/blocks.png"),
        )
        .unwrap();
        let gui = Texture::from_bytes("GUI Texture", include_bytes!("../assets/textures/gui.png"))
            .unwrap();
        let pear: Texture = Texture::from_bytes(
            "Pear Texture",
            include_bytes!("../assets/textures/pear.png"),
        )
        .unwrap();
        TextureList { blocks, gui, pear }
    }
}
