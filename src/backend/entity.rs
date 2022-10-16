use glam::f32::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Entity {
	pos:Vec3,
	rot:Vec3,
	vel:Vec3,
}

impl Entity {
	pub fn new(pos:Vec3) -> Self {
		let rot = Vec3::new(0.0, 0.0, 0.0);
		let vel = Vec3::new(0.0, 0.0, 0.0);
		Self {
			pos,
			rot,
			vel,
		}
	}
	pub fn pos(&self) -> Vec3 { self.pos }
	pub fn rot(&self) -> Vec3 { self.rot }

	pub fn set_vel(&mut self, vel:&Vec3) { self.vel = vel.clone(); }

	pub fn tick(&mut self) {
		self.pos += self.vel;
		self.vel.y -= 0.001;

		self.rot.x += self.vel.x;
		self.rot.y += self.vel.y;

		if self.pos.y < -7.5 {
			self.pos.y = -7.499;
			self.vel.y *= -0.99;
		}

		if self.pos.y > 7.5 {
			self.pos.y = 7.499;
			self.vel.y *= -0.99;
		}

		if self.pos.x < -7.5 {
			self.pos.x = -7.499;
			self.vel.x *= -0.99;
		}

		if self.pos.x > 7.5 {
			self.pos.x = 7.499;
			self.vel.x *= -0.99;
		}

		if self.pos.z < -7.5 {
			self.pos.z = -7.499;
			self.vel.z *= -0.99;
		}

		if self.pos.z > 7.5 {
			self.pos.z = 7.499;
			self.vel.z *= -0.99;
		}
	}
}
