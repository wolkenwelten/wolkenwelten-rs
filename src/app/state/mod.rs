pub struct AppState {
	pub running: bool,
	pub ticks_elapsed: u64,
}

impl AppState {
	pub fn new() -> AppState {
		let running = true;
		let ticks_elapsed: u64 = 0;

		AppState {
			running,
			ticks_elapsed,
		}
	}

	pub fn tick(&mut self) {
		self.ticks_elapsed += 1;
	}
}