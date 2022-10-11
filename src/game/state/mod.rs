use crate::game::Vector;

pub struct GameState {
    pub running:bool,
    pub ticks_elapsed:u64,
    pub player_position:Vector,
}

impl GameState {
    pub fn new() -> GameState {
        let running = true;
        let ticks_elapsed:u64 = 0;
        let player_position = Vector::new(0.0, 0.0 ,0.0 ,0.0);

        GameState {
            running,
            ticks_elapsed,
            player_position,
        }
    }

    pub fn tick(&mut self) {
        self.ticks_elapsed += 1;
    }
}