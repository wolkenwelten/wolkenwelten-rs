use glam::Vec3;

use crate::render::RenderState;

pub struct GameState {
	pub running: bool,
	pub ticks_elapsed: u64,
	pub player_position: Vec3,
	pub player_rotation: Vec3,
}

impl GameState {
	pub fn new() -> GameState {
		let running = true;
		let ticks_elapsed: u64 = 0;
		let player_position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
		let player_rotation: Vec3 = Vec3::new(0.0, 0.0, 0.0);

		GameState {
			running,
			ticks_elapsed,
			player_position,
			player_rotation,
		}
	}

	pub fn check_input(&mut self, render_state: &RenderState) {
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
		self.player_position[0] += move_vec[0] * speed;
		self.player_position[1] += move_vec[1] * speed;
		self.player_position[2] += move_vec[2] * speed;
	}

	pub fn tick(&mut self) {
		self.ticks_elapsed += 1;
	}
}