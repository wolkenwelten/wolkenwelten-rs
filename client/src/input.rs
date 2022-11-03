// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::ClientState;
use glam::swizzles::Vec4Swizzles;
use glam::Vec3;
use winit::event::MouseButton;
use wolkenwelten_common::InputEvent;
use wolkenwelten_game::{GameState, RaycastReturn};

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
    Shoot,
}

#[derive(Debug, Default)]
struct MouseState {
    left: bool,
    middle: bool,
    right: bool,
}

#[derive(Debug, Default)]
pub struct InputState {
    button_states: [bool; 32],
    mouse_xrel: f32,
    mouse_yrel: f32,
    mouse: MouseState,
}

const MOUSE_ACCELERATION: f32 = 0.03;

impl InputState {
    pub fn new() -> Self {
        Self::default()
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
    pub fn key_up_down(&mut self, code: Key, pressed: bool) {
        self.button_states[code as usize] = pressed;
    }

    pub fn set_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.mouse.left = pressed,
            MouseButton::Middle => self.mouse.middle = pressed,
            MouseButton::Right => self.mouse.right = pressed,
            _ => (),
        }
    }

    pub fn get_speed(&self) -> f32 {
        if self.button_states[Key::Sprint as usize] {
            4.0
        } else {
            2.0
        }
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
            (if self.button_states[Key::Crouch as usize] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::Jump as usize] {
                1.0
            } else {
                0.0
            }),
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
}

fn input_tick_no_clip(game: &GameState, fe: &ClientState) -> Vec<InputEvent> {
    let view = glam::Mat4::from_rotation_y(-game.player.rot[0].to_radians());
    let view = view * glam::Mat4::from_rotation_x(-game.player.rot[1].to_radians());
    let v = glam::Vec4::from((fe.input.get_movement_vector(), 1.0_f32));
    let move_vec = (view * v).xyz();

    vec![InputEvent::PlayerFly(move_vec * fe.input.get_speed())]
}

fn input_tick_default(game: &GameState, fe: &ClientState) -> Vec<InputEvent> {
    let view = glam::Mat4::from_rotation_y(-game.player.rot[0].to_radians());
    let m = fe.input.get_movement_vector();
    let v = glam::Vec4::from((m, 1.0_f32));
    let move_vec = (view * v).xyz() * fe.input.get_speed();

    vec![InputEvent::PlayerMove(Vec3::new(move_vec.x, m.y, move_vec.z))]
}

pub fn input_tick(game: &GameState, fe: &ClientState) -> Vec<InputEvent> {
    let mut events = if game.player.no_clip() {
        input_tick_no_clip(game, fe)
    } else {
        input_tick_default(game, fe)
    };

    if fe.input.mouse.left {
        if let Some(pos) = game.player.raycast(&game.world, RaycastReturn::Within) {
            events.push(InputEvent::PlayerBlockMine(pos));
        }
    }

    if fe.input.mouse.right {
        if let Some(pos) = game.player.raycast(&game.world, RaycastReturn::Front) {
            events.push(InputEvent::PlayerBlockPlace(pos));
        }
    }

    if fe.input.mouse.middle || fe.input.button_states[Key::Shoot as usize] {
        events.push(InputEvent::PlayerShoot());
    }

    events
}
