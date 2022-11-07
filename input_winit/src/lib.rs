// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use enum_map::{Enum, EnumMap};
use glam::swizzles::Vec4Swizzles;
use glam::Vec3;
use winit::dpi::PhysicalPosition;
use winit::event::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
    WindowEvent,
};
use wolkenwelten_common::{InputEvent, Message, SyncEvent};
use wolkenwelten_game::{GameState, RaycastReturn};

#[derive(Clone, Copy, Debug, Default, Enum)]
pub enum Key {
    #[default]
    Up = 0,
    Down,
    Left,
    Right,

    Jump,
    Crouch,
    Sprint,

    Primary,
    Secondary,
    Tertiary,
}

#[derive(Clone, Debug, Default)]
pub struct InputState {
    button_states: EnumMap<Key, bool>,
    queue: Vec<Message>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush_queue(&mut self, msgs: &mut Vec<Message>) {
        if self.queue.is_empty() {
            return;
        }
        msgs.append(&mut self.queue);
        self.queue.clear();
    }

    pub fn handle_winit_event(&mut self, event: Event<()>) {
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                self.queue.push(
                    InputEvent::PlayerTurn(Vec3::new(
                        delta.0 as f32 * 0.05,
                        delta.1 as f32 * 0.05,
                        0.0,
                    ))
                    .into(),
                );
            }

            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    },
                ..
            } => self.key_up_down(Key::Primary, state == ElementState::Pressed),

            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        button: MouseButton::Right,
                        state,
                        ..
                    },
                ..
            } => self.key_up_down(Key::Secondary, state == ElementState::Pressed),

            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        button: MouseButton::Middle,
                        state,
                        ..
                    },
                ..
            } => self.key_up_down(Key::Tertiary, state == ElementState::Pressed),

            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => match delta {
                MouseScrollDelta::LineDelta(_, y) => self
                    .queue
                    .push(InputEvent::PlayerSwitchSelection(y.round() as i32).into()),
                MouseScrollDelta::PixelDelta(PhysicalPosition { x: _x, y }) => self
                    .queue
                    .push(InputEvent::PlayerSwitchSelection(y.round() as i32).into()),
            },

            Event::DeviceEvent {
                event: DeviceEvent::Key(input),
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::N),
                    ..
                } => self.queue.push(InputEvent::PlayerNoClip(true).into()),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::M),
                    ..
                } => self.queue.push(InputEvent::PlayerNoClip(false).into()),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::E),
                    ..
                } => self.key_up_down(Key::Tertiary, state == ElementState::Pressed),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::W),
                    ..
                } => self.key_up_down(Key::Up, state == ElementState::Pressed),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::S),
                    ..
                } => self.key_up_down(Key::Down, state == ElementState::Pressed),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => self.queue.push(SyncEvent::GameQuit(0).into()),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::A),
                    ..
                } => self.key_up_down(Key::Left, state == ElementState::Pressed),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::D),
                    ..
                } => self.key_up_down(Key::Right, state == ElementState::Pressed),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                } => self.key_up_down(Key::Jump, state == ElementState::Pressed),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::C),
                    ..
                } => self.key_up_down(Key::Crouch, state == ElementState::Pressed),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::LShift),
                    ..
                } => self.key_up_down(Key::Sprint, state == ElementState::Pressed),

                _ => (),
            },
            _ => {}
        }
    }

    #[inline]
    pub fn key_up_down(&mut self, k: Key, state: bool) {
        self.button_states[k] = state;
    }

    #[inline]
    pub fn get_speed(&self) -> f32 {
        if self.button_states[Key::Sprint] {
            4.0
        } else {
            2.0
        }
    }

    pub fn get_movement_vector(&self) -> Vec3 {
        Vec3::new(
            (if self.button_states[Key::Left] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::Right] {
                1.0
            } else {
                0.0
            }),
            (if self.button_states[Key::Crouch] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::Jump] {
                1.0
            } else {
                0.0
            }),
            (if self.button_states[Key::Up] {
                -1.0
            } else {
                0.0
            }) + (if self.button_states[Key::Down] {
                1.0
            } else {
                0.0
            }),
        )
    }

    fn input_tick_no_clip(&mut self, game: &GameState, msg: &mut Vec<Message>) {
        let view = glam::Mat4::from_rotation_y(-game.player().rot[0].to_radians());
        let view = view * glam::Mat4::from_rotation_x(-game.player().rot[1].to_radians());
        let v = glam::Vec4::from((self.get_movement_vector(), 1.0_f32));
        let move_vec = (view * v).xyz();

        msg.push(InputEvent::PlayerFly(move_vec * self.get_speed()).into());
    }

    fn input_tick_default(&mut self, game: &GameState, msg: &mut Vec<Message>) {
        let view = glam::Mat4::from_rotation_y(-game.player().rot[0].to_radians());
        let m = self.get_movement_vector();
        let v = glam::Vec4::from((m, 1.0_f32));
        let move_vec = (view * v).xyz() * self.get_speed();

        msg.push(InputEvent::PlayerMove(Vec3::new(move_vec.x, m.y, move_vec.z)).into());
    }

    pub fn tick(&mut self, game: &GameState) -> Vec<Message> {
        let mut msgs: Vec<Message> = vec![];
        self.flush_queue(&mut msgs);
        if game.player().no_clip() {
            self.input_tick_no_clip(game, &mut msgs);
        } else {
            self.input_tick_default(game, &mut msgs);
        };

        if self.button_states[Key::Primary] {
            if let Some(pos) = game.player().raycast(game.world(), RaycastReturn::Within) {
                msgs.push(InputEvent::PlayerBlockMine(pos).into());
            }
        }

        if self.button_states[Key::Secondary] {
            if let Some(pos) = game.player().raycast(game.world(), RaycastReturn::Front) {
                msgs.push(InputEvent::PlayerBlockPlace(pos).into());
            }
        }

        if self.button_states[Key::Tertiary] {
            msgs.push(InputEvent::PlayerShoot.into());
        }

        msgs
    }
}
