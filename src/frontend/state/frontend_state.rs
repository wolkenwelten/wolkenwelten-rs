use glam::f32::{Mat4, Vec3};
use std::time::{Instant};
use crate::{GameState};
use crate::backend::{ChunkBlockData, Entity};
use crate::frontend::input::InputState;
use crate::frontend::mesh::{BlockMesh, TextMesh};
use crate::frontend::state::{MeshList, TextureList};
use crate::frontend::state::ShaderList;
use crate::frontend::Viewport;

pub struct FrontendState {
	pub instant: Instant,

	pub world_mesh: BlockMesh,

	pub viewport: Viewport,
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
	pub fn new() -> FrontendState {
		let last_ticks = 0;
		let window_width = 640;
		let window_height = 480;
		unsafe {
			gl::ClearColor(0.32, 0.63, 0.96, 1.0);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			gl::Enable(gl::BLEND);
			gl::Enable(gl::TEXTURE0);
			gl::Enable(gl::CULL_FACE);
			gl::FrontFace(gl::CCW);
			gl::CullFace(gl::BACK);

			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::LEQUAL);

			gl::Enable(gl::PROGRAM_POINT_SIZE);
		}
		FrontendState {
			instant: Instant::now(),
			world_mesh: BlockMesh::new_empty(),

			viewport: Viewport::for_window(window_width, window_height),
			meshes: MeshList::new(),
			shaders: ShaderList::new(),
			input:  InputState::new(),
			textures: TextureList::new(),
			ui_mesh: TextMesh::new(),

			cur_fps: 0,
			frame_count: 0,
			last_ticks,
		}
	}

	pub fn update_world(&mut self, chunk:&ChunkBlockData) {
		self.world_mesh = chunk.into();
	}

	pub fn fps(&self) -> u32 { self.cur_fps }

	pub fn calc_fps(&mut self) -> &mut FrontendState {
		let ticks = self.instant.elapsed().as_millis();
		if ticks > self.last_ticks + 1000 {
			self.cur_fps = (((self.frame_count as f32) / ((ticks - self.last_ticks) as f32)) * 1000.0) as u32;
			self.last_ticks = ticks;
			self.frame_count = 0;
		}
		self.frame_count += 1;
		self
	}

	pub fn prepare(&mut self, game_state: &mut GameState) -> &mut FrontendState {
		let fps_text = format!("FPS: {}", self.fps());
		let pos_text = format!("X:{:8.2} Y:{:8.2} Z:{:8.2}", game_state.player_position[0], game_state.player_position[1], game_state.player_position[2]);
		let rot_text = format!("Y:{:8.2} P:{:8.2} R:{:8.2}", game_state.player_rotation[0], game_state.player_rotation[1], game_state.player_rotation[2]);
		self.ui_mesh.empty()
			.push_string(8,8,2,0xFFFFFFFF, fps_text.as_str())
			.push_string(8,40,1,0xFFFFFFFF, pos_text.as_str())
			.push_string(8,50,1,0xFFFFFFFF, rot_text.as_str())
			.prepare();

		self.calc_fps()
	}

	pub fn draw(&self, game_state: &GameState) {
		self.viewport.set_used();
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		};
		game_state.draw(self);
		let trans = Vec3::new(0.0,0.0,0.0);
		self.shaders.block.set_trans(&trans);
		self.world_mesh.draw();

		let perspective = glam::Mat4::orthographic_rh_gl(
			0.0,
			self.viewport.w as f32,

			self.viewport.h as f32,
			0.0,

			-10.0,
			10.0,
		);

		self.shaders.text.set_used();
		self.shaders.text.set_mvp(&perspective);
		self.textures.gui.bind();
		self.ui_mesh.draw();
	}

	pub fn draw_entity(&self, entity:&Entity, view: &Mat4, projection: &Mat4) {
		let rot = entity.rot();
		let pos = entity.pos();

		let model = Mat4::from_rotation_x(rot.x);
		let model = Mat4::from_rotation_y(rot.y) * model;
		let model = Mat4::from_translation(pos) * model;
		let vp = projection.mul_mat4(view);
		let mvp = vp.mul_mat4(&model);

		self.shaders.mesh.set_mvp(&mvp);
		self.textures.pear.bind();
		self.meshes.pear.draw();
	}
}
