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
use crate::Side;

#[derive(Clone, Debug, Default)]
pub struct BlockType {
    name: String,
    texture_index: [u8; 6],
}

impl BlockType {
    pub fn new(name: &str) -> Self {
        let texture_index: [u8; 6] = [0; 6];
        let name = name.to_string();
        Self {
            name,
            texture_index,
        }
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
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_texture_top(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Top)
    }
    pub fn with_texture_bottom(self, tex: u8) -> Self {
        self.with_texture_side(tex, Side::Bottom)
    }

    pub fn tex_front(&self) -> u8 {
        self.texture_index[Side::Front as usize]
    }
    pub fn tex_back(&self) -> u8 {
        self.texture_index[Side::Back as usize]
    }
    pub fn tex_left(&self) -> u8 {
        self.texture_index[Side::Left as usize]
    }
    pub fn tex_right(&self) -> u8 {
        self.texture_index[Side::Right as usize]
    }
    pub fn tex_top(&self) -> u8 {
        self.texture_index[Side::Top as usize]
    }
    pub fn tex_bottom(&self) -> u8 {
        self.texture_index[Side::Bottom as usize]
    }
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = &self.name;
        write!(f, "<BlockType name={} />", name)
    }
}
