use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub fn check_events(event_pump:&mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true;
            },
            _ => {}
        }
    }
    return false;
}