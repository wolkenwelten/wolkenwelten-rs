pub struct ChunkBlockData {
	data: [[[u8; 16]; 16]; 16],
}

impl ChunkBlockData {
	pub fn new() -> Self {
		let mut data = [[[0; 16]; 16]; 16];
		data[8][8][2] = 1;
		data[8][9][2] = 2;
		data[8][7][2] = 3;
		data[7][8][2] = 4;
		data[9][8][2] = 5;
		data[10][8][2] = 6;
		data[6][8][2] = 7;
		data[11][8][2] = 8;
		data[5][8][2] = 9;
		Self { data }
	}

	pub fn get_block(&self, x:u16, y:u16, z:u16) -> u8 { self.data[x as usize][y as usize][z as usize] }
	pub fn _set_block(&mut self, x:u16, y:u16, z:u16, block:u8) { self.data[x as usize][y as usize][z as usize] = block }
}
