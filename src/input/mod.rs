use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::{AppState, RenderState};

pub mod input_state;
pub use self::input_state::{InputState, Key};

impl RenderState {
    pub fn check_events(&mut self, app_state:&mut AppState) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    app_state.running = false;
                },

                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    self.input.key_down(Key::Up);
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    self.input.key_up(Key::Up);
                },

                Event::KeyDown { keycode: Some(Keycode::S), .. }  => {
                    self.input.key_down(Key::Down);
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. }  => {
                    self.input.key_up(Key::Down);
                },

                Event::KeyDown { keycode: Some(Keycode::A), .. }  => {
                    self.input.key_down(Key::Left);
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. }  => {
                    self.input.key_up(Key::Left);
                },

                Event::KeyDown { keycode: Some(Keycode::D), .. }  => {
                    self.input.key_down(Key::Right);
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. }  => {
                    self.input.key_up(Key::Right);
                },

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.input.key_down(Key::Jump);
                },
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    self.input.key_up(Key::Jump);
                },

                Event::KeyDown { keycode: Some(Keycode::LCtrl), .. } => {
                    self.input.key_down(Key::Crouch);
                },
                Event::KeyUp { keycode: Some(Keycode::LCtrl), .. } => {
                    self.input.key_up(Key::Crouch);
                },

                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.input.key_down(Key::RotateUp);
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    self.input.key_up(Key::RotateUp);
                },

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.input.key_down(Key::RotateDown);
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    self.input.key_up(Key::RotateDown);
                },

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.input.key_down(Key::RotateLeft);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    self.input.key_up(Key::RotateLeft);
                },

                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.input.key_down(Key::RotateRight);
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    self.input.key_up(Key::RotateRight);
                },

                Event::KeyDown { keycode: Some(Keycode::LShift), .. } => {
                    self.input.key_down(Key::Sneak);
                },
                Event::KeyUp { keycode: Some(Keycode::LShift), .. } => {
                    self.input.key_up(Key::Sneak);
                },

                Event::MouseMotion {xrel, yrel, ..} => {
                    self.input.mouse_motion(xrel, yrel);
                },
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    self.viewport.update_size(w, h);
                    self.viewport.set_used();
                },
                _ => {}
            }
        }
    }
}