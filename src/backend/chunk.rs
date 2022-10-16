pub struct ChunkBlockData {
	data: [[[u8; 16]; 16]; 16],
}

impl ChunkBlockData {
	pub fn new() -> Self {
		let mut data = [[[0; 16]; 16]; 16];
		data[8][8][8] = 1;
		data[8][9][8] = 2;
		data[8][7][8] = 3;
		data[7][8][8] = 4;
		data[9][8][8] = 5;
		data[10][8][8] = 6;
		data[6][8][8] = 7;
		data[11][8][8] = 8;
		data[5][8][8] = 9;
		Self { data }
	}

	pub fn get_block(&self, x:u16, y:u16, z:u16) -> u8 { self.data[x as usize][y as usize][z as usize] }
	pub fn _set_block(&mut self, x:u16, y:u16, z:u16, block:u8) { self.data[x as usize][y as usize][z as usize] = block }
}
