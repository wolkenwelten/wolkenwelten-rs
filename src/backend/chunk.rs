pub struct ChunkBlockData {
	data: [[[u8; 16]; 16]; 16],
}

impl ChunkBlockData {
	pub fn new() -> Self {
		let data = [[[0; 16]; 16]; 16];
		Self { data }
	}

	pub fn test() -> Self {
		let mut ret = Self::new();
		ret.set_block(3,8,15,8);
		ret.set_sphere(2, 8,10,8,5);
		ret.set_sphere(1, 8,9,8,5);
		ret.set_sphere(3, 8,8,8,5);
		ret.set_box(3, 14, 3, 12, 2, 3, 3);
		ret.set_box(3, 14, 1, 4, 1, 4, 3);
		ret.set_box(3, 1, 5, 3, 3, 3, 2);
		ret.set_box(3, 2, 2, 14, 2, 5, 2);
		ret
	}

	pub fn get_block(&self, x:i32, y:i32, z:i32) -> u8 { self.data[x as usize][y as usize][z as usize] }
	pub fn set_block(&mut self, block:u8, x:i32, y:i32, z:i32) { self.data[x as usize][y as usize][z as usize] = block }
	pub fn set_sphere(&mut self, block:u8, x:i32, y:i32, z:i32, radius: i32) {
		let rr = radius*radius;
		for cx in -radius..radius {
			for cy in -radius..radius {
				for cz in -radius..radius {
					let dist = (cx*cx)+(cy*cy)+(cz*cz);
					if dist < rr {
						self.data[(x+cx) as usize][(y+cy) as usize][(z+cz) as usize] = block;
					}
				}
			}
		}
	}
	pub fn set_box(&mut self, block:u8, x:i32, y:i32, z:i32, w:i32, h:i32, d:i32) {
		for cx in 0..w {
			for cy in 0..h {
				for cz in 0..d {
					self.data[(x+cx) as usize][(y+cy) as usize][(z+cz) as usize] = block;
				}
			}
		}
	}
}
