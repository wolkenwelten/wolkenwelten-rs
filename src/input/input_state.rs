use glam::Vec3;

pub enum Key {
    Up = 0,
    Down,
    Left,
    Right,
    Jump,
    Crouch,
}

pub struct InputState {
    button_states: [bool; 6],
}

impl InputState {
    pub fn new() -> InputState {
        let button_states = [false; 6];

        InputState { button_states }
    }

    pub fn key_down(&mut self, code: Key) {
        self.button_states[code as usize] = true;
    }
    pub fn key_up(&mut self, code: Key) {
        self.button_states[code as usize] = false;
    }
    pub fn get_movement_vector(&self) -> Vec3 {
        Vec3::new (
            ( if self.button_states[Key::Left as usize]   { -1.0 } else { 0.0 }) + ( if self.button_states[Key::Right as usize] { 1.0 } else { 0.0 }),
            ( if self.button_states[Key::Crouch as usize] { -1.0 } else { 0.0 }) + ( if self.button_states[Key::Jump as usize]  { 1.0 } else { 0.0 }),
            ( if self.button_states[Key::Down as usize]   { -1.0 } else { 0.0 }) + ( if self.button_states[Key::Up as usize]    { 1.0 } else { 0.0 }),
        )
    }
}