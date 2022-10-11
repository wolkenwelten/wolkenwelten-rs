use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::{AppState, RenderState};

impl RenderState {
    pub fn check_events(&mut self, app_state:&mut AppState) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    app_state.running = false;
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