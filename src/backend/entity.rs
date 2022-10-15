use glam::f32::Vec3;
use glam::Mat4;
use crate::RenderState;

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

	pub fn draw(&self, render_state:&RenderState, view: &Mat4, projection: &Mat4) {
		let model = Mat4::from_rotation_x(self.rot.x);
		let model = Mat4::from_rotation_y(self.rot.y) * model;
		let model = Mat4::from_translation(self.pos) * model;
		let vp = projection.mul_mat4(view);
		let mvp = vp.mul_mat4(&model);

		render_state.shaders.mesh.set_mvp(&mvp);
		render_state.textures.pear.bind();
		render_state.meshes.pear.draw();
	}
}
