// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use rgb::RGBA8;
use wolkenwelten_common::BlockType;

pub fn load_all() -> Vec<BlockType> {
    vec![
        BlockType::new("Air"),
        BlockType::new("Dirt").with_texture(1).with_colors(
            RGBA8::new(0x4F, 0x23, 0x0A, 0xFF),
            RGBA8::new(0x45, 0x1B, 0x05, 0xFF),
        ),
        BlockType::new("Grass")
            .with_texture(16)
            .with_texture_top(0)
            .with_texture_bottom(1)
            .with_colors(
                RGBA8::new(0x27, 0x42, 0x00, 0xFF),
                RGBA8::new(0x45, 0x1B, 0x05, 0xFF),
            ),
        BlockType::new("Stone").with_texture(2).with_colors(
            RGBA8::new(0x5E, 0x5E, 0x5E, 0xFF),
            RGBA8::new(0x48, 0x48, 0x48, 0xFF),
        ),
        BlockType::new("Coal").with_texture(3).with_colors(
            RGBA8::new(0x26, 0x26, 0x26, 0xFF),
            RGBA8::new(0x10, 0x10, 0x10, 0xFF),
        ),
        BlockType::new("Spruce log").with_texture(4).with_colors(
            RGBA8::new(0x25, 0x1B, 0x05, 0xFF),
            RGBA8::new(0x1D, 0x16, 0x07, 0xFF),
        ),
        BlockType::new("Spruce leaves").with_texture(5).with_colors(
            RGBA8::new(0x12, 0x2C, 0x01, 0xFF),
            RGBA8::new(0x0F, 0x25, 0x01, 0xFF),
        ),
        BlockType::new("Dry grass")
            .with_texture(22)
            .with_texture_top(6)
            .with_texture_bottom(1)
            .with_colors(
                RGBA8::new(0x4B, 0x64, 0x11, 0xFF),
                RGBA8::new(0x4F, 0x23, 0x0A, 0xFF),
            ),
        BlockType::new("Roots").with_texture(7).with_colors(
            RGBA8::new(0x3E, 0x32, 0x14, 0xFF),
            RGBA8::new(0x29, 0x20, 0x0D, 0xFF),
        ),
        BlockType::new("Obsidian").with_texture(8).with_colors(
            RGBA8::new(0x22, 0x22, 0x22, 0xFF),
            RGBA8::new(0x17, 0x17, 0x17, 0xFF),
        ),
        BlockType::new("Oak log").with_texture(9).with_colors(
            RGBA8::new(0x3C, 0x2C, 0x08, 0xFF),
            RGBA8::new(0x2E, 0x24, 0x08, 0xFF),
        ),
        BlockType::new("Oak leaves").with_texture(10).with_colors(
            RGBA8::new(0x27, 0x42, 0x00, 0xFF),
            RGBA8::new(0x18, 0x33, 0x00, 0xFF),
        ),
        BlockType::new("Iron ore (hematite)")
            .with_texture(11)
            .with_colors(
                RGBA8::new(0x72, 0x5B, 0x5B, 0xFF),
                RGBA8::new(0x5E, 0x5E, 0x5E, 0xFF),
            ),
        BlockType::new("Marble block").with_texture(12).with_colors(
            RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
            RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
        ),
        BlockType::new("Marble pillar")
            .with_texture(13)
            .with_texture_top(12)
            .with_texture_bottom(12)
            .with_colors(
                RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
            ),
        BlockType::new("Marble blocks")
            .with_texture(14)
            .with_colors(
                RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
                RGBA8::new(0xF0, 0xF0, 0xF0, 0xFF),
            ),
        BlockType::new("Acacia leaves")
            .with_texture(24)
            .with_colors(
                RGBA8::new(0x02, 0x30, 0x00, 0xFF),
                RGBA8::new(0x32, 0x6F, 0x1C, 0xFF),
            ),
        BlockType::new("Boards").with_texture(17).with_colors(
            RGBA8::new(0x8F, 0x67, 0x09, 0xFF),
            RGBA8::new(0xAF, 0x80, 0x13, 0xFF),
        ),
        BlockType::new("Crystals").with_texture(18).with_colors(
            RGBA8::new(0xE8, 0x7C, 0x99, 0xFF),
            RGBA8::new(0xB5, 0x24, 0x4D, 0xFF),
        ),
        BlockType::new("Sakura leaves")
            .with_texture(19)
            .with_colors(
                RGBA8::new(0xE8, 0x7C, 0x99, 0xFF),
                RGBA8::new(0xB5, 0x25, 0x4D, 0xFF),
            ),
        BlockType::new("Birch log").with_texture(20).with_colors(
            RGBA8::new(0x55, 0x52, 0x52, 0xFF),
            RGBA8::new(0xA5, 0xA2, 0xA2, 0xFF),
        ),
        BlockType::new("Flower bush").with_texture(21).with_colors(
            RGBA8::new(0x27, 0x42, 0x00, 0xFF),
            RGBA8::new(0x18, 0x33, 0x00, 0xFF),
        ),
        BlockType::new("Date bush").with_texture(23).with_colors(
            RGBA8::new(0x4F, 0x33, 0x00, 0xFF),
            RGBA8::new(0x94, 0x83, 0x12, 0xFF),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use wolkenwelten_common::Side;

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
        let blocks = load_all();
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
