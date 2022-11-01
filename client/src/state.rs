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
use super::render;
use crate::input::InputState;
use crate::meshes::BlockMesh;
use crate::RENDER_DISTANCE;
use glam::f32::Vec3;
use glam::i32::IVec3;
use glam::Mat4;
use std::collections::HashMap;
use std::time::Instant;
use wgpu::util::DeviceExt;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;
use wolkenwelten_common::CHUNK_SIZE;
use wolkenwelten_game::{Character, GameState};

pub mod static_meshes;
pub mod static_shaders;
pub mod static_textures;

pub struct ClientState {
    instant: Instant,

    pub block_index_buffer: Vec<u16>,
    pub world_mesh: HashMap<IVec3, BlockMesh>,

    window_width: u32,
    window_height: u32,

    pub device: Device,
    pub surface: Surface,
    pub queue: Queue,
    pub surface_config: SurfaceConfiguration,
    pub window: Window,

    pub meshes: MeshList,
    pub shaders: ShaderList,
    pub textures: TextureList,

    pub input: InputState,

    cur_fov: f32,
    cur_fps: u32,
    frame_count: u32,
    last_ticks: u128,

    wireframe: bool,
}

impl ClientState {
    pub async fn new(window: Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_config);

        let textures = TextureList::new(&device, &queue);
        let shaders = ShaderList::new(&device, &surface_config);
        let meshes = MeshList::new(&device);

        Self {
            instant: Instant::now(),
            block_index_buffer: BlockMesh::gen_index_buffer(65536 / 4),
            world_mesh: HashMap::new(),

            window_width: 640,
            window_height: 480,

            device,
            queue,
            surface,
            surface_config,
            window,

            meshes,
            shaders,
            input: InputState::new(),
            textures,

            cur_fov: 90.0,
            cur_fps: 0,
            frame_count: 0,
            last_ticks: 0,
            wireframe: false,
        }
    }

    pub fn block_indeces(&self) -> &Vec<u16> {
        &self.block_index_buffer
    }

    pub fn fps(&self) -> u32 {
        self.cur_fps
    }
    pub fn calc_fps(&mut self) {
        let ticks = self.instant.elapsed().as_millis();
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
        self.world_mesh.retain(|&pos, _| {
            let diff: Vec3 = (pos.as_vec3() * CHUNK_SIZE as f32) - player.pos;
            let d = diff.dot(diff);
            d < (RENDER_DISTANCE * RENDER_DISTANCE)
        });
    }

    pub fn set_wireframe(&mut self, w: bool) {
        self.wireframe = w;
    }
    pub fn wireframe(&self) -> bool {
        self.wireframe
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.cur_fov = fov;
    }
    pub fn fov(&self) -> f32 {
        self.cur_fov
    }

    pub fn window_size(&self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }
    pub fn set_window_size(&mut self, (w, h): (u32, u32)) {
        if w > 0 && h > 0 {
            self.window_width = w;
            self.window_height = h;
            self.surface_config.width = w;
            self.surface_config.height = h;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn render_entities(fe: &ClientState, game: &GameState) {
        let (window_width, window_height) = fe.window_size();
        let projection = Mat4::perspective_rh_gl(
            fe.fov().to_radians(),
            (window_width as f32) / (window_height as f32),
            0.1,
            RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
        );
        let view = Mat4::from_rotation_x(game.player.rot[1].to_radians());
        let view = view * Mat4::from_rotation_y(game.player.rot[0].to_radians());
        let view = view * Mat4::from_translation(-game.player.pos);

        let mvp = projection * view;

        /*
        game.entities
            .iter()
            .for_each(|entity| draw_entity(fe, entity, &view, &projection));
         */

        //render_chungus(fe, game, &mvp);
    }

    pub fn render_frame(&mut self, game: &GameState) {
        let output = self.surface.get_current_texture().unwrap();
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let camera_bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("camera_bind_group_layout"),
                });

        let (window_width, window_height) = self.window_size();
        let projection = Mat4::perspective_rh_gl(
            self.fov().to_radians(),
            (window_width as f32) / (window_height as f32),
            0.1,
            RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0,
        );
        let view = Mat4::from_rotation_x(game.player.rot[1].to_radians());
        let view = view * Mat4::from_rotation_y(game.player.rot[0].to_radians());

        let s = RENDER_DISTANCE + CHUNK_SIZE as f32 * 2.0;
        let sky_view = view * Mat4::from_scale(Vec3::new(s, s, s));
        let mvp = projection * sky_view;
        let sky_camera_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(mvp.as_ref()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let sky_camera_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: sky_camera_buffer.as_entire_binding(),
            }],
            label: Some("sky_camera_bind_group"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.32,
                            g: 0.63,
                            b: 0.96,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.shaders.mesh);
            render_pass.set_bind_group(0, &self.textures.sky.group, &[]); // NEW!
            render_pass.set_bind_group(1, &sky_camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.meshes.dome.buf().slice(..));
            render_pass.draw(0..self.meshes.dome.vertex_count(), 0..1); // 3.
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
