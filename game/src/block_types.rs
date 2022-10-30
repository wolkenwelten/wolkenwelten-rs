/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use wolkenwelten_common::BlockType;

pub fn load_all() -> Vec<BlockType> {
    vec![
        BlockType::new("Air"),
        BlockType::new("Dirt").with_texture(1),
        BlockType::new("Grass")
            .with_texture(16)
            .with_texture_top(0)
            .with_texture_bottom(1),
        BlockType::new("Stone").with_texture(2),
        BlockType::new("Coal").with_texture(3),
        BlockType::new("Spruce log").with_texture(4),
        BlockType::new("Spruce leaves").with_texture(5),
        BlockType::new("Dry grass")
            .with_texture(22)
            .with_texture_top(6)
            .with_texture_bottom(1),
        BlockType::new("Roots").with_texture(7),
        BlockType::new("Obsidian").with_texture(8),
        BlockType::new("Oak log").with_texture(9),
        BlockType::new("Oak leaves").with_texture(10),
        BlockType::new("Iron ore (hematite)").with_texture(11),
        BlockType::new("Marble block").with_texture(12),
        BlockType::new("Marble pillar")
            .with_texture(13)
            .with_texture_top(12)
            .with_texture_bottom(12),
        BlockType::new("Marble blocks").with_texture(14),
        BlockType::new("Acacia leaves").with_texture(24),
        BlockType::new("Boards").with_texture(17),
        BlockType::new("Crystals").with_texture(18),
        BlockType::new("Sakura leaves").with_texture(19),
        BlockType::new("Birch log").with_texture(20),
        BlockType::new("Flower bush").with_texture(21),
        BlockType::new("Date bush").with_texture(23),
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
