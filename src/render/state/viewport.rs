use gl;

pub struct Viewport {
	pub w: u32,
	pub h: u32,
}

impl Viewport {
	pub fn for_window(w: u32, h: u32) -> Viewport {
		Viewport { w, h }
	}

	pub fn update_size(&mut self, w: u32, h: u32) {
		self.w = w;
		self.h = h;
	}

	pub fn set_used(&self) {
		unsafe {
			gl::Viewport(0, 0, self.w.try_into().unwrap(), self.h.try_into().unwrap());
		}
	}
}
