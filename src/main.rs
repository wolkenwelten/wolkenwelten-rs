use crate::render::RenderState;
use crate::app::AppState;
use crate::game::GameState;

mod app;
mod game;
mod input;
mod render;

pub fn main() {
	let mut app_state = AppState::new();
	let mut game_state = GameState::new();
	let mut render_state = RenderState::new();

	'running: loop {
		render_state.check_events(&mut app_state);
		render_state.draw(&app_state, &game_state);

		game_state.tick();
		if !game_state.running {
			game_state = GameState::new(); // Auto Restart for now
		}

		app_state.tick();
		if !app_state.running {
			break 'running;
		}
	}
}
