use crate::render::Texture;

pub struct TextureList {
    pub blocks:Texture,
    pub gui:Texture,
    pub pear:Texture,
}

impl TextureList {
    pub fn new() -> TextureList {
        let blocks = Texture::from_bytes(include_bytes!("./blocks.png")).unwrap();
        let gui = Texture::from_bytes(include_bytes!("./gui.png")).unwrap();
        let pear:Texture = Texture::from_bytes(include_bytes!("./pear.png")).unwrap();
        TextureList {
            blocks,
            gui,
            pear,
        }
    }
}