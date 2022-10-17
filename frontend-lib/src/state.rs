pub use self::static_meshes::MeshList;
pub use self::static_shaders::ShaderList;
pub use self::static_textures::TextureList;
use crate::input::InputState;
use crate::meshes::{BlockMesh, TextMesh};
use std::time::Instant;

pub mod static_meshes;
pub mod static_shaders;
pub mod static_textures;

pub struct FrontendState {
    pub instant: Instant,

    pub world_mesh: BlockMesh,

    pub window_width: u32,
    pub window_height: u32,

    pub meshes: MeshList,
    pub shaders: ShaderList,
    pub textures: TextureList,

    pub ui_mesh: TextMesh,

    pub input: InputState,

    pub cur_fps: u32,
    pub frame_count: u32,
    pub last_ticks: u128,
}

impl FrontendState {
    pub fn new() -> Self {
        let last_ticks = 0;
        let window_width = 640;
        let window_height = 480;

        Self {
            instant: Instant::now(),
            world_mesh: BlockMesh::new(),

            window_width,
            window_height,

            meshes: MeshList::new(),
            shaders: ShaderList::new(),
            input: InputState::new(),
            textures: TextureList::new(),
            ui_mesh: TextMesh::new(),

            cur_fps: 0,
            frame_count: 0,
            last_ticks,
        }
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
}
