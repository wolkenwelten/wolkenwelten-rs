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
use crate::Texture;
use wgpu;

pub struct TextureList {
    //pub blocks: TextureArray,
    pub gui: Texture,
    pub pear: Texture,
    pub sky: Texture,
}

impl TextureList {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> TextureList {
        /*
        let blocks = TextureArray::from_bytes(
            "Blocks Texture",
            include_bytes!("../assets/textures/blocks.png"),
        )
        .unwrap();
         */
        let gui = Texture::from_bytes(
            device,
            queue,
            include_bytes!("../assets/textures/gui.png"),
            "GUI Texture",
        )
        .unwrap();
        let sky = Texture::from_bytes(
            device,
            queue,
            include_bytes!("../assets/textures/sky.png"),
            "Sky Texture",
        )
        .unwrap();
        let pear: Texture = Texture::from_bytes(
            device,
            queue,
            include_bytes!("../assets/textures/pear.png"),
            "Pear Texture",
        )
        .unwrap();
        TextureList { gui, pear, sky }
    }
}
