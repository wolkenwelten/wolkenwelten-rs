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
use crate::Program;

#[derive(Debug, Default)]
pub struct ShaderList {
    pub block: Program,
    pub mesh: Program,
    pub text: Program,
}

impl ShaderList {
    pub fn new() -> Self {
        let mesh = Program::from_shader_sources(
            "Mesh Shader",
            include_str!("../shaders/mesh.vert"),
            include_str!("../shaders/mesh.frag"),
        )
        .unwrap();

        let text = Program::from_shader_sources(
            "Text Shader",
            include_str!("../shaders/text.vert"),
            include_str!("../shaders/text.frag"),
        )
        .unwrap();

        let block = Program::from_shader_sources(
            "Block Shader",
            include_str!("../shaders/block.vert"),
            include_str!("../shaders/block.frag"),
        )
        .unwrap();

        Self { block, mesh, text }
    }
}
