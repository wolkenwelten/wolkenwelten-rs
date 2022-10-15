use crate::render::{Texture, TextureArray};

pub struct TextureList {
	pub blocks: TextureArray,
	pub gui: Texture,
	pub pear: Texture,
}

impl TextureList {
	pub fn new() -> TextureList {
		let blocks = TextureArray::from_bytes("Blocks", include_bytes!("./blocks.png")).unwrap();
		let gui = Texture::from_bytes("GUI", include_bytes!("./gui.png")).unwrap();
		let pear: Texture = Texture::from_bytes("Pear", include_bytes!("./pear.png")).unwrap();
		TextureList {
			blocks,
			gui,
			pear,
		}
	}
}
