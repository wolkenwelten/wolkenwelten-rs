use std::ffi::c_void;

use gl;

pub struct Texture {
	id: gl::types::GLuint,
}

impl Texture {
	pub fn from_bytes(
		bytes: &'static [u8]
	) -> Result<Texture, image::ImageError> {
		let img = image::load_from_memory(bytes)?;
		let width: u16 = img.width().try_into().unwrap();
		let height: u16 = img.height().try_into().unwrap();
		let img = img.flipv();
		let img = match img {
			image::DynamicImage::ImageRgba8(img) => img,
			x => x.to_rgba8()
		};

		let mut id = 0;
		unsafe {
			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());

			gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA.try_into().unwrap(),
                           width.try_into().unwrap(),
                           height.try_into().unwrap(),
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           (&img as &[u8]).as_ptr() as *const c_void,
            );
		};

		Ok(Texture { id })
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}
}

impl Drop for Texture {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteTextures(1, std::ptr::addr_of_mut!(self.id));
		}
	}
}

