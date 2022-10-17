use crate::FrontendState;
use glam::Vec3;
use rostregen_backend::GameState;

pub enum Key {
    Up = 0,
    Down,
    Left,
    Right,

    Jump,
    Crouch,
    Sprint,

    RotateUp,
    RotateDown,
    RotateLeft,
    RotateRight,
}

pub struct InputState {
    button_states: [bool; 11],
    mouse_xrel: f32,
    mouse_yrel: f32,
}

const MOUSE_ACCELERATION: f32 = 0.03;

impl InputState {
    pub fn new() -> Self {
        let button_states = [false; 11];
        let mouse_xrel = 0.0;
        let mouse_yrel = 0.0;

        Self {
            button_states,
            mouse_xrel,
            mouse_yrel,
        }
    }

    pub fn mouse_motion(&mut self, xrel: f32, yrel: f32) {
        self.mouse_xrel = (xrel as f32) * MOUSE_ACCELERATION;
        self.mouse_yrel = (yrel as f32) * MOUSE_ACCELERATION;
    }
    pub fn mouse_flush(&mut self) {
        self.mouse_xrel = 0.0;
        self.mouse_yrel = 0.0;
    }
    pub fn xrel(&self) -> f32 {
        self.mouse_xrel
    }
    pub fn yrel(&self) -> f32 {
        self.mouse_yrel
    }

    pub fn key_down(&mut self, code: Key) {
        self.button_states[code as usize] = true;
    }
    pub fn key_up(&mut self, code: Key) {
        self.button_states[code as usize] = false;
    }

    pub fn get_speed(&self) -> f32 {
        if self.button_states[Key::Sprint as usize] {
            0.1
        } else {
            0.01
        }
    }
    pub fn get_jump(&self) -> f32 {
        (if self.button_states[Key::Crouch as usize] {
            -1.0
        } else {
            0.0
        }) + (if self.button_states[Key::Jump as usize] {
            1.0
        } else {
            0.0
        })
    }
    pub fn get_movement_vector(&self) -> Vec3 {
        Vec3::new(
            (if self.button_states[Key::Left as usize] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::Right as usize] {
                1.0
            } else {
                0.0
            }),
            0.0,
            (if self.button_states[Key::Up as usize] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::Down as usize] {
                1.0
            } else {
                0.0
            }),
        )
    }

    pub fn get_rotation_movement_vector(&self) -> Vec3 {
        Vec3::new(
            (if self.button_states[Key::RotateLeft as usize] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::RotateRight as usize] {
                1.0
            } else {
                0.0
            }),
            (if self.button_states[Key::RotateDown as usize] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::RotateUp as usize] {
                1.0
            } else {
                0.0
            }),
            0.0,
        )
    }
}

pub fn input_tick(game: &mut GameState, fe: &FrontendState) {
    let rot_vec = fe.input.get_rotation_movement_vector();

    game.player_rotation[0] += (rot_vec[0] * 0.2) + fe.input.xrel() * 16.0;
    game.player_rotation[1] += (rot_vec[1] * 0.2) + fe.input.yrel() * 16.0;
    game.player_rotation[2] += rot_vec[2] * 0.2;

    if game.player_rotation[0] < 0.0 {
        game.player_rotation[0] += 360.0;
    }
    if game.player_rotation[0] > 360.0 {
        game.player_rotation[0] -= 360.0;
    }

    if game.player_rotation[1] < -90.0 {
        game.player_rotation[1] = -90.0;
    }
    if game.player_rotation[1] > 90.0 {
        game.player_rotation[1] = 90.0;
    }

    let view = glam::Mat4::from_rotation_y(-game.player_rotation[0].to_radians());
    let view = view * glam::Mat4::from_rotation_x(-game.player_rotation[1].to_radians());
    let v = glam::Vec4::from((fe.input.get_movement_vector(), 1.0_f32));
    let move_vec = view * v;
    let speed = fe.input.get_speed();
    game.player_position[0] += move_vec[0] * speed;
    game.player_position[1] += (move_vec[1] * speed) + (fe.input.get_jump() * speed);
    game.player_position[2] += move_vec[2] * speed;
}
