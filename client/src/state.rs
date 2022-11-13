// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::meshes::{BlockMesh, TextMesh};
use crate::RENDER_DISTANCE;
use anyhow::Result;
use glam::{f32::Vec3, i32::IVec3};
use std::{collections::HashMap, time::Instant};
use wolkenwelten_common::CHUNK_SIZE;
use wolkenwelten_game::Character;
use wolkenwelten_particles::ParticleMesh;

pub mod static_meshes;
pub mod static_shaders;
pub mod static_textures;
pub use self::static_meshes::MeshList;
pub use self::static_shaders::ShaderList;
pub use self::static_textures::TextureList;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ClientState {
    instant: Instant,

    block_index_buffer: glium::IndexBuffer<u16>,
    pub world_mesh: HashMap<IVec3, BlockMesh>,

    window_width: u32,
    window_height: u32,

    pub display: glium::Display,
    pub meshes: MeshList,
    pub shaders: ShaderList,
    pub textures: TextureList,

    pub ui_mesh: TextMesh,
    particles: Rc<RefCell<ParticleMesh>>,

    ticks: u64,
    cur_fov: f32,
    cur_fps: u32,
    frame_count: u32,
    last_ticks: u128,
}

impl ClientState {
    pub fn new(display: glium::Display) -> Result<Self> {
        let meshes = MeshList::new(&display)?;
        let shaders = ShaderList::new(&display)?;
        let ui_mesh = TextMesh::new(&display)?;
        let textures = TextureList::new(&display)?;
        let block_index_buffer = BlockMesh::gen_index_buffer(&display, 65536 / 4)?;
        let particles = Rc::new(RefCell::new(ParticleMesh::new()));

        Ok(Self {
            instant: Instant::now(),
            block_index_buffer,
            world_mesh: HashMap::new(),

            window_width: 640,
            window_height: 480,

            display,
            meshes,
            shaders,
            textures,
            ui_mesh,
            particles,

            cur_fov: 90.0,
            cur_fps: 0,
            frame_count: 0,
            ticks: 0,
            last_ticks: 0,
        })
    }

    #[inline]
    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn request_redraw(&mut self) {
        self.display.gl_window().window().request_redraw();
    }

    #[inline]
    pub fn block_indeces(&self) -> &glium::IndexBuffer<u16> {
        &self.block_index_buffer
    }

    #[inline]
    pub fn particles(&self) -> Rc<RefCell<ParticleMesh>> {
        self.particles.clone()
    }

    #[inline]
    pub fn fps(&self) -> u32 {
        self.cur_fps
    }
    pub fn calc_fps(&mut self) {
        let ticks = self.instant.elapsed().as_millis();
        self.ticks = ticks as u64;
        if ticks > self.last_ticks + 1000 {
            self.cur_fps = (((self.frame_count as f64) / ((ticks - self.last_ticks) as f64))
                * 1000.0)
                .round() as u32;
            self.last_ticks = ticks;
            self.frame_count = 0;
        }
        self.frame_count += 1;
    }

    pub fn gc(&mut self, player: &Character) {
        let center = Vec3::new(
            CHUNK_SIZE as f32 / 2.0,
            CHUNK_SIZE as f32 / 2.0,
            CHUNK_SIZE as f32 / 2.0,
        );
        self.world_mesh.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32) + center - player.pos;
            let d = diff.dot(diff);
            d < (RENDER_DISTANCE + (CHUNK_SIZE * 2) as f32)
                * (RENDER_DISTANCE + (CHUNK_SIZE * 2) as f32)
        });
    }

    #[inline]
    pub fn set_fov(&mut self, fov: f32) {
        self.cur_fov = fov;
    }
    #[inline]
    pub fn fov(&self) -> f32 {
        self.cur_fov
    }

    #[inline]
    pub fn window_size(&self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }
    #[inline]
    pub fn set_window_size(&mut self, (w, h): (u32, u32)) {
        self.window_width = w;
        self.window_height = h;
    }
}
