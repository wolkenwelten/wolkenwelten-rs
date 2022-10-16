use glam::{Vec3, Vec4};
use crate::backend::Entity;
use rand::Rng;
use super::ChunkBlockData;

use crate::frontend::FrontendState;

pub struct GameState {
	pub running: bool,
	pub ticks_elapsed: u64,
	pub player_position: Vec3,
	pub player_rotation: Vec3,

	pub entities: Vec<Entity>,
	pub world:ChunkBlockData,
}

impl GameState {
	pub fn new() -> Self {
		let running = true;
		let ticks_elapsed: u64 = 0;
		let player_position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
		let player_rotation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
		let mut entities:Vec<Entity> = Vec::with_capacity(16);
		let mut rng = rand::thread_rng();
		for x in -2..=2 {
			for z in -2..=2 {
				let y:f32 = (rng.gen::<f32>()*5.0) + 2.5;
				let mut e = Entity::new(Vec3::new(x as f32, y, z as f32));
				let vx:f32 = (rng.gen::<f32>() - 0.5)*0.05;
				let vy:f32 = (rng.gen::<f32>() - 0.5)*0.01;
				let vz:f32 = (rng.gen::<f32>() - 0.5)*0.05;
				let vel = Vec3::new(vx, vy, vz);
				e.set_vel(&vel);

				entities.push(e);
			}
		}
		let world = ChunkBlockData::new();

		Self {
			running,
			ticks_elapsed,
			player_position,
			player_rotation,

			entities,
			world,
		}
	}
	pub fn _push_entity (&mut self, e:Entity) { self.entities.push(e); }

	pub fn check_input(&mut self, render_state: &FrontendState) {
		let rot_vec = render_state.input.get_rotation_movement_vector();

		self.player_rotation[0] += (rot_vec[0] * 0.2) + render_state.input.xrel() * 16.0;
		self.player_rotation[1] += (rot_vec[1] * 0.2) + render_state.input.yrel() * 16.0;
		self.player_rotation[2] += rot_vec[2] * 0.2;

		if self.player_rotation[0] < 0.0 { self.player_rotation[0] += 360.0; }
		if self.player_rotation[0] > 360.0 { self.player_rotation[0] -= 360.0; }

		if self.player_rotation[1] < -90.0 { self.player_rotation[1] = -90.0; }
		if self.player_rotation[1] >  90.0 { self.player_rotation[1] =  90.0; }


		let view = glam::Mat4::from_rotation_y(-self.player_rotation[0].to_radians());
		let view = view * glam::Mat4::from_rotation_x(-self.player_rotation[1].to_radians());
		let v = glam::Vec4::from((render_state.input.get_movement_vector(), 1.0_f32));
		let move_vec = view * v;
		let speed = render_state.input.get_speed();
		self.player_position[0] +=  move_vec[0] * speed;
		self.player_position[1] += (move_vec[1] * speed) + (render_state.input.get_jump() * speed);
		self.player_position[2] +=  move_vec[2] * speed;
	}

	pub fn tick(&mut self) {
		self.ticks_elapsed += 1;
		for e in &mut self.entities {
			e.tick();
		}
	}

	pub fn draw(&self, render_state:&FrontendState) {
		let projection = glam::Mat4::perspective_rh_gl(
			90.0_f32.to_radians(),
			(render_state.viewport.w as f32) / (render_state.viewport.h as f32),
			0.1,
			100.0,
		);

		let view = glam::Mat4::from_rotation_x(self.player_rotation[1].to_radians());
		let view = view * glam::Mat4::from_rotation_y(self.player_rotation[0].to_radians());
		let view = view * glam::Mat4::from_translation(-self.player_position);
		let mvp = projection * view;

		render_state.shaders.mesh.set_used();
		let c = Vec4::new(1.0, 1.0, 1.0, 1.0);
		render_state.shaders.mesh.set_color(&c);
		render_state.textures.pear.bind();

		for e in &self.entities {
			render_state.draw_entity(&e, &view, &projection);
		}

		render_state.shaders.block.set_used();
		render_state.shaders.block.set_mvp(&mvp);
		let trans: Vec3 = Vec3::new(-8.0,-8.0,-8.0);
		render_state.shaders.block.set_trans(&trans);
		render_state.shaders.block.set_alpha(1.0);
		render_state.textures.blocks.bind();
		render_state.meshes.voxel_test.draw();
	}
}
