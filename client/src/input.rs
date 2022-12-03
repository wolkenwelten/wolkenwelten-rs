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
use wolkenwelten_core::{GameState, Message, RaycastReturn, Reactor};

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
    Drop,

    Primary,
    Secondary,
    Tertiary,
}

#[derive(Clone, Debug, Default)]
pub struct InputState {
    button_states: EnumMap<Key, bool>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_winit_event(&mut self, reactor: &Reactor<Message>, event: Event<()>) {
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                reactor.dispatch(Message::PlayerTurn {
                    direction: Vec3::new(delta.0 as f32 * 0.05, delta.1 as f32 * 0.05, 0.0),
                });
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
                MouseScrollDelta::LineDelta(_, y) => {
                    reactor.dispatch(Message::PlayerSwitchSelection {
                        delta: y.round() as i32,
                    })
                }
                MouseScrollDelta::PixelDelta(PhysicalPosition { x: _x, y }) => {
                    reactor.dispatch(Message::PlayerSwitchSelection {
                        delta: y.round() as i32,
                    })
                }
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
                } => reactor.dispatch(Message::PlayerNoClip { no_clip: true }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::M),
                    ..
                } => reactor.dispatch(Message::PlayerNoClip { no_clip: false }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key1),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 0 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key2),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 1 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key3),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 2 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key4),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 3 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key5),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 4 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key6),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 5 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key7),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 6 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key8),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 7 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key9),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 8 }),

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Key0),
                    ..
                } => reactor.dispatch(Message::PlayerSelect { i: 9 }),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::E),
                    ..
                } => self.key_up_down(Key::Tertiary, state == ElementState::Pressed),

                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::Q),
                    ..
                } => self.key_up_down(Key::Drop, state == ElementState::Pressed),

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
                } => reactor.dispatch(Message::GameQuit),

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
            1.4
        } else {
            1.0
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

    pub fn tick(&mut self, game: &GameState, reactor: &Reactor<Message>) {
        let view = glam::Mat4::from_rotation_y(-game.player().rot[0].to_radians());
        if game.player().no_clip() {
            let view = view * glam::Mat4::from_rotation_x(-game.player().rot[1].to_radians());
            let v = glam::Vec4::from((self.get_movement_vector(), 1.0_f32));
            let move_vec = (view * v).xyz();

            reactor.dispatch(Message::PlayerFly {
                direction: move_vec * self.get_speed(),
            });
        } else {
            let m = self.get_movement_vector();
            let v = glam::Vec4::from((m, 1.0_f32));
            let move_vec = (view * v).xyz() * self.get_speed();

            reactor.dispatch(Message::PlayerMove {
                direction: Vec3::new(move_vec.x, m.y, move_vec.z),
            });
        };

        if self.button_states[Key::Primary] {
            reactor.dispatch(Message::PlayerStrike);
            let o = game.player().raycast(&game.world(), RaycastReturn::Within);
            if let Some(pos) = o {
                reactor.dispatch(Message::PlayerBlockMine { pos: Some(pos) });
            } else {
                reactor.dispatch(Message::PlayerBlockMine { pos: None });
            }
        } else {
            reactor.dispatch(Message::PlayerBlockMine { pos: None });
        }

        if self.button_states[Key::Secondary] {
            let o = game.player().raycast(&game.world(), RaycastReturn::Front);
            if let Some(pos) = o {
                reactor.dispatch(Message::PlayerBlockPlace { pos });
            }
        }

        if self.button_states[Key::Tertiary] {
            reactor.dispatch(Message::PlayerShoot);
        }

        if self.button_states[Key::Drop] {
            reactor.dispatch(Message::PlayerDropItem);
        }
    }
}
