extern crate sdl2;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::video::{GLContext, Window};
use crate::AppState;
use crate::render::Viewport;
use crate::render::state::MeshList;
use crate::render::state::ShaderList;

pub struct RenderState {
    pub sdl_context:Sdl,
    pub video_subsystem:VideoSubsystem,
    pub gl_context:GLContext,
    pub window:Window,
    pub event_pump:EventPump,

    pub viewport:Viewport,
    pub meshes:MeshList,
    pub shaders:ShaderList,
}

impl RenderState {
    pub fn new() -> RenderState {
        let window_width:i32 = 640;
        let window_height:i32 = 480;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);

        let window = video_subsystem
            .window("RostRegen", window_width.try_into().unwrap(), window_height.try_into().unwrap())
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let viewport = Viewport::for_window(window_width, window_height);
        let event_pump = sdl_context.event_pump().unwrap();
        let shaders = ShaderList::new();
        let meshes = MeshList::new();

        RenderState {
            sdl_context,
            video_subsystem,
            gl_context,
            viewport,
            window,
            event_pump,
            meshes,
            shaders,
        }
    }

    pub fn draw(&self, app_state: &AppState) {
        let i = app_state.ticks_elapsed;
        let r:f32 = 0.4 + (((i as f64) / 100.0).sin() / 5.0) as f32;
        unsafe {
            gl::ClearColor(r, 0.5, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.shaders.mesh.set_used();
        self.meshes.triangle.draw();
        self.window.gl_swap_window();
    }
}