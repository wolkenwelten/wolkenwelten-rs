use std::ffi::CStr;

use glutin::event_loop::EventLoop;
use glutin::window::{CursorGrabMode, Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};

pub fn init_glutin() -> (EventLoop<()>, ContextWrapper<PossiblyCurrent, Window>) {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_decorations(false)
        .with_maximized(true)
        .with_title("RostRegen");

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .with_double_buffer(Some(true))
        .build_windowed(wb, &event_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let _gl = gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    {
        let window = windowed_context.window();
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        window.set_cursor_visible(false);
    }

    let _version = unsafe {
        let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    (event_loop, windowed_context)
}
