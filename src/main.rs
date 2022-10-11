extern crate sdl2;
extern crate gl;

use crate::render::RenderState;
use crate::app::AppState;

mod app;
mod input;
mod render;

pub fn main() {
	let mut app_state = AppState::new();
	let mut render_state = RenderState::new();

	'running: loop {
		render_state.check_events(&mut app_state);
		render_state.draw(&app_state);

		app_state.tick();
		if !app_state.running {
			break 'running;
		}
	}
}
