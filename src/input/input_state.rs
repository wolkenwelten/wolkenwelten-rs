use glam::Vec3;

pub enum Key {
	Up = 0,
	Down,
	Left,
	Right,

	Jump,
	Crouch,
	Sneak,

	RotateUp,
	RotateDown,
	RotateLeft,
	RotateRight,

}

pub struct InputState {
	button_states: [bool; 11],
	mouse_xrel: f32,
	mouse_yrel: f32,
}

impl InputState {
	pub fn new() -> InputState {
		let button_states = [false; 11];
		let mouse_xrel = 0.0;
		let mouse_yrel = 0.0;

		InputState {
			button_states,
			mouse_xrel,
			mouse_yrel,
		}
	}

	pub fn mouse_motion(&mut self, xrel: i32, yrel: i32) {
		self.mouse_xrel = (xrel as f32) * 0.02;
		self.mouse_yrel = (yrel as f32) * 0.02;
	}
	pub fn mouse_flush(&mut self) {
		self.mouse_xrel = 0.0;
		self.mouse_yrel = 0.0;
	}
	pub fn xrel(&self) -> f32 { self.mouse_xrel }
	pub fn yrel(&self) -> f32 { self.mouse_yrel }

	pub fn key_down(&mut self, code: Key) {
		self.button_states[code as usize] = true;
	}
	pub fn key_up(&mut self, code: Key) {
		self.button_states[code as usize] = false;
	}

	pub fn get_speed(&self) -> f32 {
		if self.button_states[Key::Sneak as usize] { 0.1 } else { 0.01 }
	}
	pub fn get_movement_vector(&self) -> Vec3 {
		Vec3::new(
			(if self.button_states[Key::Left as usize] { -1.0 } else { 0.0 }) + (if self.button_states[Key::Right as usize] { 1.0 } else { 0.0 }),
			(if self.button_states[Key::Crouch as usize] { -1.0 } else { 0.0 }) + (if self.button_states[Key::Jump as usize] { 1.0 } else { 0.0 }),
			(if self.button_states[Key::Up as usize] { -1.0 } else { 0.0 }) + (if self.button_states[Key::Down as usize] { 1.0 } else { 0.0 }),
		)
	}

	pub fn get_rotation_movement_vector(&self) -> Vec3 {
		Vec3::new(
			(if self.button_states[Key::RotateLeft as usize] { -1.0 } else { 0.0 }) + (if self.button_states[Key::RotateRight as usize] { 1.0 } else { 0.0 }),
			(if self.button_states[Key::RotateDown as usize] { -1.0 } else { 0.0 }) + (if self.button_states[Key::RotateUp as usize] { 1.0 } else { 0.0 }),
			0.0,
		)
	}
}