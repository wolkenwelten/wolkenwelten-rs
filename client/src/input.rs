use crate::ClientState;
use glam::swizzles::Vec4Swizzles;
use glam::Vec3;
use wolkenwelten_game::GameState;

#[derive(Debug, Default)]
pub enum Key {
    #[default]
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

#[derive(Debug, Default)]
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
            4.0
        } else {
            1.0
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

pub fn input_tick(game: &mut GameState, fe: &ClientState) {
    let rot_vec = fe.input.get_rotation_movement_vector();

    game.player.rot[0] += (rot_vec[0] * 0.2) + fe.input.xrel() * 16.0;
    game.player.rot[1] += (rot_vec[1] * 0.2) + fe.input.yrel() * 16.0;
    game.player.rot[2] += rot_vec[2] * 0.2;

    if game.player.rot[0] < 0.0 {
        game.player.rot[0] += 360.0;
    }
    if game.player.rot[0] > 360.0 {
        game.player.rot[0] -= 360.0;
    }

    if game.player.rot[1] < -90.0 {
        game.player.rot[1] = -90.0;
    }
    if game.player.rot[1] > 90.0 {
        game.player.rot[1] = 90.0;
    }

    let view = glam::Mat4::from_rotation_y(-game.player.rot[0].to_radians());
    let view = view * glam::Mat4::from_rotation_x(-game.player.rot[1].to_radians());
    let v = glam::Vec4::from((fe.input.get_movement_vector(), 1.0_f32));
    let move_vec = (view * v).xyz();
    let speed = fe.input.get_speed();

    if game.player.no_clip() {
        game.player.vel = move_vec * speed;
        game.player.vel.y += fe.input.get_jump() * speed;
    } else {
        game.player.vel.x = ((move_vec.x * speed * 0.05) + (game.player.vel.x * 0.95)) / 2.0;
        game.player.vel.z = ((move_vec.z * speed * 0.05) + (game.player.vel.z * 0.95)) / 2.0;
        if (fe.input.get_jump() > 0.0) && game.player.may_jump(&game.world) {
            game.player.vel.y = 0.1;
        }
    }
}
