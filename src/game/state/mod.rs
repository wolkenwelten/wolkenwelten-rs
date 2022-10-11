use glam::Vec3;
use crate::render::RenderState;

pub struct GameState {
    pub running:bool,
    pub ticks_elapsed:u64,
    pub player_position:Vec3,
}

impl GameState {
    pub fn new() -> GameState {
        let running = true;
        let ticks_elapsed:u64 = 0;
        let player_position:Vec3 = Vec3::new(0.0, 0.0, 0.0);

        GameState {
            running,
            ticks_elapsed,
            player_position,
        }
    }

    pub fn check_input(&mut self, render_state: &RenderState) {
        let move_vec = render_state.input.get_movement_vector();
        self.player_position[0] += move_vec[0] * 0.01;
        self.player_position[1] += move_vec[1] * 0.01;
        self.player_position[2] += move_vec[2] * 0.01;
    }

    pub fn tick(&mut self) {
        self.ticks_elapsed += 1;
    }
}