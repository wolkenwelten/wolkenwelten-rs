/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use crate::{ClientState, InputEvent};
use glam::swizzles::Vec4Swizzles;
use glam::{Vec3, Vec3Swizzles};
use winit::event::MouseButton;
use wolkenwelten_game::{Entity, GameState, RaycastReturn};

const CHARACTER_ACCELERATION: f32 = 0.08;
const CHARACTER_STOP_RATE: f32 = CHARACTER_ACCELERATION * 3.0;

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
struct MouseState {
    left: bool,
    middle: bool,
    right: bool,
}

#[derive(Debug, Default)]
pub struct InputState {
    button_states: [bool; 11],
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
            _ => todo!(),
        }
    }

    pub fn get_speed(&self) -> f32 {
        if self.button_states[Key::Sprint as usize] {
            4.0
        } else {
            2.0
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

fn input_tick_no_clip(game: &mut GameState, fe: &ClientState) -> Vec<InputEvent> {
    let view = glam::Mat4::from_rotation_y(-game.player.rot[0].to_radians());
    let view = view * glam::Mat4::from_rotation_x(-game.player.rot[1].to_radians());
    let v = glam::Vec4::from((fe.input.get_movement_vector(), 1.0_f32));
    let move_vec = (view * v).xyz();
    let speed = fe.input.get_speed() * 0.15;
    game.player.vel = move_vec * speed;
    game.player.vel.y += fe.input.get_jump() * speed;

    Vec::new()
}

fn input_tick_default(game: &mut GameState, fe: &ClientState) -> Vec<InputEvent> {
    let mut events = Vec::new();
    let view = glam::Mat4::from_rotation_y(-game.player.rot[0].to_radians());
    let view = view * glam::Mat4::from_rotation_x(-game.player.rot[1].to_radians());
    let v = glam::Vec4::from((fe.input.get_movement_vector(), 1.0_f32));
    let move_vec = (view * v).xyz() * fe.input.get_speed() * 0.02;
    // Different rates for moving/stopping since this makes the player feel more responsive
    let acc = if move_vec.xz().length() > 0.001 {
        CHARACTER_ACCELERATION
    } else {
        CHARACTER_STOP_RATE
    };

    let acc = if game.player.may_jump(&game.world) {
        acc
    } else {
        acc * 0.2
    };

    game.player.vel.x += (move_vec.x - game.player.vel.x) * acc;
    game.player.vel.z += (move_vec.z - game.player.vel.z) * acc;

    if (fe.input.get_jump() > 0.0) && game.player.may_jump(&game.world) {
        game.player.jump();
        events.push(InputEvent::PlayerJump());
    }
    events
}

pub fn input_tick(game: &mut GameState, fe: &ClientState) -> Vec<InputEvent> {
    let rot_vec = fe.input.get_rotation_movement_vector();
    let now = game.get_millis();

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

    let mut events = if game.player.no_clip() {
        input_tick_no_clip(game, fe)
    } else {
        input_tick_default(game, fe)
    };

    if fe.input.mouse.left && game.player.may_act(now) {
        if let Some(pos) = game.player.raycast(&game.world, RaycastReturn::Within) {
            game.player.set_cooldown(now + 300);
            game.world.set_block(pos, 0);
            events.push(InputEvent::PlayerBlockMine(pos));
        }
    }

    if fe.input.mouse.right && game.player.may_act(now) {
        if let Some(pos) = game.player.raycast(&game.world, RaycastReturn::Front) {
            game.player.set_cooldown(now + 300);
            game.world.set_block(pos, game.player.block_selection());
            events.push(InputEvent::PlayerBlockPlace(pos));
        }
    }

    if fe.input.mouse.middle && game.player.may_act(now) {
        game.player.set_cooldown(now + 600);
        let mut e = Entity::new();
        e.set_pos(game.player.pos());
        e.set_vel(game.player.direction() * 0.4);
        game.push_entity(e);
        events.push(InputEvent::PlayerShoot());
    }

    events
}
