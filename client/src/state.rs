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
pub use self::static_meshes::MeshList;
pub use self::static_shaders::ShaderList;
pub use self::static_textures::TextureList;
use crate::input::InputState;
use crate::meshes::{BlockMesh, TextMesh};
use glam::f32::Vec3;
use glam::i32::IVec3;
use std::collections::HashMap;
use std::time::Instant;
use wolkenwelten_game::Character;

pub mod static_meshes;
pub mod static_shaders;
pub mod static_textures;

#[derive(Debug)]
pub struct ClientState {
    pub instant: Instant,

    pub world_mesh: HashMap<IVec3, BlockMesh>,

    pub window_width: u32,
    pub window_height: u32,

    pub meshes: MeshList,
    pub shaders: ShaderList,
    pub textures: TextureList,

    pub ui_mesh: TextMesh,

    pub input: InputState,

    pub cur_fov: f32,
    pub cur_fps: u32,
    pub frame_count: u32,
    pub last_ticks: u128,
}

impl Default for ClientState {
    fn default() -> Self {
        let last_ticks = 0;
        let window_width = 640;
        let window_height = 480;

        Self {
            instant: Instant::now(),
            world_mesh: HashMap::new(),

            window_width,
            window_height,

            meshes: MeshList::new(),
            shaders: ShaderList::new(),
            input: InputState::new(),
            textures: TextureList::new(),
            ui_mesh: TextMesh::new(),

            cur_fov: 90.0,
            cur_fps: 0,
            frame_count: 0,
            last_ticks,
        }
    }
}

impl ClientState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fps(&self) -> u32 {
        self.cur_fps
    }
    pub fn calc_fps(&mut self) {
        let ticks = self.instant.elapsed().as_millis();
        if ticks > self.last_ticks + 1000 {
            self.cur_fps =
                (((self.frame_count as f32) / ((ticks - self.last_ticks) as f32)) * 1000.0) as u32;
            self.last_ticks = ticks;
            self.frame_count = 0;
        }
        self.frame_count += 1;
    }
    pub fn set_window_size(&mut self, new_width: u32, new_height: u32) {
        self.window_width = new_width;
        self.window_height = new_height;
    }

    pub fn gc(&mut self, player: &Character) {
        let mut removal_queue: Vec<IVec3> = Vec::new();
        for pos in self.world_mesh.keys() {
            let diff: Vec3 = (pos.as_vec3() * 16.0) - player.pos;
            let d = diff.dot(diff);
            if d > (256.0 * 256.0) {
                removal_queue.push(*pos);
            }
        }
        for pos in removal_queue {
            self.world_mesh.remove(&pos);
        }
    }
}
