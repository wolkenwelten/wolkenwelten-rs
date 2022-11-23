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
