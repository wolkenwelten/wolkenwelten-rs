// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Side;
use rgb::RGBA8;

#[derive(Clone, Copy, Debug, Default)]
pub enum MiningCategory {
    #[default]
    None,
    Axe(u8),
    Pickaxe(u8),
    Shovel(u8),
}

const WHITE: RGBA8 = RGBA8::new(255, 255, 255, 255);
#[derive(Clone, Debug, Default)]
pub struct BlockType {
    name: String,
    texture_index: [u8; 6],
    colors: [RGBA8; 2],
    mining_cat: MiningCategory,
    block_health: u16,
}

impl BlockType {
    fn init_vox_types() -> Vec<Self> {
        let mut ret = vec![];
        for i in 0..255 {
            ret.push(Self::new("").with_texture(i));
        }
        ret
    }
    #[inline]
    pub fn get_vox_types() -> Vec<Self> {
        Self::init_vox_types()
    }

    pub fn new(name: &str) -> Self {
        let texture_index: [u8; 6] = [0; 6];
        let name = name.to_string();
        let colors = [WHITE; 2];
        Self {
            mining_cat: MiningCategory::None,
            name,
            texture_index,
            colors,
            block_health: 50,
        }
    }
    pub fn with_colors(mut self, a: RGBA8, b: RGBA8) -> Self {
        self.colors = [a, b];
        self
    }
    pub fn with_texture(mut self, tex: u8) -> Self {
        self.texture_index = [tex; 6];
        self
    }
    pub fn with_texture_side(mut self, tex: u8, side: Side) -> Self {
        let i: usize = side.into();
        self.texture_index[i] = tex;
        self
    }
    pub fn with_mining_cat(mut self, cat: MiningCategory) -> Self {
        self.mining_cat = cat;
        self
    }
    pub fn with_block_health(mut self, block_health: u16) -> Self {
        self.block_health = block_health;
        self
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn colors(&self) -> [RGBA8; 2] {
        self.colors
    }

    pub fn with_texture_top(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Top)
    }
    pub fn with_texture_bottom(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Bottom)
    }

    #[inline]
    pub fn tex(&self) -> [u8; 6] {
        self.texture_index
    }
    #[inline]
    pub fn tex_front(&self) -> u8 {
        self.texture_index[Side::Front as usize]
    }
    #[inline]
    pub fn tex_back(&self) -> u8 {
        self.texture_index[Side::Back as usize]
    }
    #[inline]
    pub fn tex_left(&self) -> u8 {
        self.texture_index[Side::Left as usize]
    }
    #[inline]
    pub fn tex_right(&self) -> u8 {
        self.texture_index[Side::Right as usize]
    }
    #[inline]
    pub fn tex_top(&self) -> u8 {
        self.texture_index[Side::Top as usize]
    }
    #[inline]
    pub fn tex_bottom(&self) -> u8 {
        self.texture_index[Side::Bottom as usize]
    }
    #[inline]
    pub fn mining_cat(&self) -> MiningCategory {
        self.mining_cat
    }
    #[inline]
    pub fn block_health(&self) -> u16 {
        self.block_health
    }
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = &self.name;
        write!(f, "<BlockType name={} />", name)
    }
}

impl BlockType {
    pub fn new_default() -> Vec<Self> {
        vec![
            BlockType::new("Air"),
            BlockType::new("Dirt")
                .with_texture(1)
                .with_colors(
                    RGBA8::new(0x11, 0x0A, 0x00, 0xFF),
                    RGBA8::new(0x20, 0x12, 0x00, 0xFF),
                )
                .with_mining_cat(MiningCategory::Shovel(1))
                .with_block_health(200),
            BlockType::new("Grass")
                .with_texture(16)
                .with_texture_top(0)
                .with_texture_bottom(1)
                .with_colors(
                    RGBA8::new(0x08, 0x12, 0x00, 0xFF),
                    RGBA8::new(0x11, 0x0A, 0x00, 0xFF),
                )
                .with_mining_cat(MiningCategory::Shovel(1))
                .with_block_health(250),
            BlockType::new("Stone")
                .with_texture(2)
                .with_colors(
                    RGBA8::new(0x5E, 0x5E, 0x5E, 0xFF),
                    RGBA8::new(0x48, 0x48, 0x48, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(1))
                .with_block_health(800),
            BlockType::new("Coal")
                .with_texture(3)
                .with_colors(
                    RGBA8::new(0x26, 0x26, 0x26, 0xFF),
                    RGBA8::new(0x10, 0x10, 0x10, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(1))
                .with_block_health(700),
            BlockType::new("Spruce log")
                .with_texture(4)
                .with_colors(
                    RGBA8::new(0x25, 0x1B, 0x05, 0xFF),
                    RGBA8::new(0x1D, 0x16, 0x07, 0xFF),
                )
                .with_mining_cat(MiningCategory::Axe(1))
                .with_block_health(600),
            BlockType::new("Spruce leaves")
                .with_texture(5)
                .with_colors(
                    RGBA8::new(0x12, 0x2C, 0x01, 0xFF),
                    RGBA8::new(0x0F, 0x25, 0x01, 0xFF),
                )
                .with_block_health(100),
            BlockType::new("Dry grass")
                .with_texture(22)
                .with_texture_top(6)
                .with_texture_bottom(1)
                .with_colors(
                    RGBA8::new(0x4B, 0x64, 0x11, 0xFF),
                    RGBA8::new(0x4F, 0x23, 0x0A, 0xFF),
                )
                .with_mining_cat(MiningCategory::Shovel(1))
                .with_block_health(200),
            BlockType::new("Roots")
                .with_texture(7)
                .with_colors(
                    RGBA8::new(0x3E, 0x32, 0x14, 0xFF),
                    RGBA8::new(0x29, 0x20, 0x0D, 0xFF),
                )
                .with_mining_cat(MiningCategory::Shovel(1))
                .with_block_health(500),
            BlockType::new("Obsidian")
                .with_texture(8)
                .with_colors(
                    RGBA8::new(0x22, 0x22, 0x22, 0xFF),
                    RGBA8::new(0x17, 0x17, 0x17, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(2))
                .with_block_health(1400),
            BlockType::new("Oak log")
                .with_texture(9)
                .with_colors(
                    RGBA8::new(0x3C, 0x2C, 0x08, 0xFF),
                    RGBA8::new(0x2E, 0x24, 0x08, 0xFF),
                )
                .with_mining_cat(MiningCategory::Axe(1))
                .with_block_health(700),
            BlockType::new("Oak leaves")
                .with_texture(10)
                .with_colors(
                    RGBA8::new(0x27, 0x42, 0x00, 0xFF),
                    RGBA8::new(0x18, 0x33, 0x00, 0xFF),
                )
                .with_block_health(100),
            BlockType::new("Iron ore (hematite)")
                .with_texture(11)
                .with_colors(
                    RGBA8::new(0x72, 0x5B, 0x5B, 0xFF),
                    RGBA8::new(0x5E, 0x5E, 0x5E, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(1))
                .with_block_health(1000),
            BlockType::new("Marble block")
                .with_texture(12)
                .with_colors(
                    RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                    RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(1))
                .with_block_health(1000),
            BlockType::new("Marble pillar")
                .with_texture(13)
                .with_texture_top(12)
                .with_texture_bottom(12)
                .with_colors(
                    RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                    RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(1))
                .with_block_health(1000),
            BlockType::new("Marble blocks")
                .with_texture(14)
                .with_colors(
                    RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                    RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(1))
                .with_block_health(1000),
            BlockType::new("Acacia leaves")
                .with_texture(15)
                .with_colors(
                    RGBA8::new(0x02, 0x30, 0x00, 0xFF),
                    RGBA8::new(0x32, 0x6F, 0x1C, 0xFF),
                )
                .with_block_health(100),
            BlockType::new("Boards")
                .with_texture(17)
                .with_colors(
                    RGBA8::new(0x8F, 0x67, 0x09, 0xFF),
                    RGBA8::new(0xAF, 0x80, 0x13, 0xFF),
                )
                .with_mining_cat(MiningCategory::Axe(1))
                .with_block_health(400),
            BlockType::new("Crystals")
                .with_texture(18)
                .with_colors(
                    RGBA8::new(0xE8, 0x7C, 0x99, 0xFF),
                    RGBA8::new(0xB5, 0x24, 0x4D, 0xFF),
                )
                .with_mining_cat(MiningCategory::Pickaxe(3))
                .with_block_health(2000),
            BlockType::new("Sakura leaves")
                .with_texture(19)
                .with_colors(
                    RGBA8::new(0xE8, 0x7C, 0x99, 0xFF),
                    RGBA8::new(0xB5, 0x25, 0x4D, 0xFF),
                )
                .with_block_health(100),
            BlockType::new("Birch log")
                .with_texture(20)
                .with_colors(
                    RGBA8::new(0x55, 0x52, 0x52, 0xFF),
                    RGBA8::new(0xA5, 0xA2, 0xA2, 0xFF),
                )
                .with_mining_cat(MiningCategory::Axe(1))
                .with_block_health(600),
            BlockType::new("Flower bush")
                .with_texture(21)
                .with_colors(
                    RGBA8::new(0x27, 0x42, 0x00, 0xFF),
                    RGBA8::new(0x18, 0x33, 0x00, 0xFF),
                )
                .with_block_health(100),
            BlockType::new("Date bush")
                .with_texture(23)
                .with_colors(
                    RGBA8::new(0x4F, 0x33, 0x00, 0xFF),
                    RGBA8::new(0x94, 0x83, 0x12, 0xFF),
                )
                .with_block_health(100),
            BlockType::new("Sand")
                .with_texture(24)
                .with_colors(
                    RGBA8::new(0xEC, 0xD1, 0x95, 0xFF),
                    RGBA8::new(0xD3, 0xA7, 0x48, 0xFF),
                )
                .with_block_health(140),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Side;

    #[test]
    fn test_side() {
        assert_eq!(u8::from(Side::Front), 0);
        assert_eq!(u8::from(Side::Back), 1);
        assert_eq!(u8::from(Side::Right), 5);
        assert_eq!(Side::Left, Side::Left);
        assert_eq!(Side::Left, Side::Left.clone());
        assert_ne!(Side::Left, Side::Right);
        assert_eq!(Side::default(), Side::Front);
    }

    #[test]
    fn test_block_types() {
        let blocks = BlockType::new_default();
        assert_eq!(blocks[1].tex_right(), 1);
        assert_eq!(blocks[1].tex_left(), 1);
        assert_eq!(blocks[1].tex_top(), 1);
        assert_eq!(blocks[1].tex_bottom(), 1);
        assert_eq!(blocks[1].tex_front(), 1);
        assert_eq!(blocks[1].tex_back(), 1);
        let b = blocks[1].clone().with_texture_bottom(2);
        assert_eq!(b.tex_bottom(), 2);
        assert_eq!(b.tex_top(), 1);
        let b = b.with_texture_top(2);
        assert_eq!(b.tex_bottom(), 2);
        assert_eq!(b.tex_top(), 2);
        assert_eq!(b.tex_left(), 1);
        assert!(blocks.len() > 8);
        assert_eq!(blocks[1].name(), "Dirt");
        assert_ne!(blocks[2].tex_top(), blocks[2].tex_left());
        let dis = format!("{}", blocks[3]);
        assert_eq!(dis, "<BlockType name=Stone />");
    }
}
