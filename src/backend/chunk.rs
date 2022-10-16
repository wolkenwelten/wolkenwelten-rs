pub struct ChunkBlockData {
	data: [[[u8; 16]; 16]; 16],
}

impl ChunkBlockData {
	pub fn new() -> Self {
		let mut data = [[[0; 16]; 16]; 16];
		data[8][8][2] = 1;
		data[8][9][2] = 2;
		data[8][7][2] = 3;
		Self { data }
	}

	pub fn get_block(&self, x:u16, y:u16, z:u16) -> u8 { self.data[x as usize][y as usize][z as usize] }
	pub fn _set_block(&mut self, x:u16, y:u16, z:u16, block:u8) { self.data[x as usize][y as usize][z as usize] = block }
}
