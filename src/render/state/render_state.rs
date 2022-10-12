extern crate sdl2;

use glam::Vec4;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::video::{GLContext, Window};
use crate::{AppState, GameState};
use crate::input::InputState;
use crate::render::Viewport;
use crate::render::state::{MeshList, TextureList};
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
    pub textures:TextureList,

    pub input:InputState,
}

impl RenderState {
    pub fn new() -> RenderState {
        let window_width:i32 = 640;
        let window_height:i32 = 480;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
        gl_attr.set_context_version(3, 0);

        let window = video_subsystem
            .window("RostRegen", window_width.try_into().unwrap(), window_height.try_into().unwrap())
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let mouse = sdl_context.mouse();
        mouse.show_cursor(false);
        mouse.set_relative_mouse_mode(true);
        video_subsystem.disable_screen_saver();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        video_subsystem.gl_set_swap_interval(-1).unwrap();

        unsafe {
            gl::ClearColor(0.32, 0.63, 0.96, 1.0);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::Enable(gl::BLEND);
            gl::Enable(gl::TEXTURE0);
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);
            gl::CullFace(gl::BACK);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::Enable(gl::PROGRAM_POINT_SIZE);
        }

        let viewport = Viewport::for_window(window_width, window_height);
        let event_pump = sdl_context.event_pump().unwrap();
        let shaders = ShaderList::new();
        let meshes = MeshList::new();
        let input = InputState::new();
        let textures = TextureList::new();

        RenderState {
            sdl_context,
            video_subsystem,
            gl_context,
            viewport,
            window,
            event_pump,
            meshes,
            shaders,
            input,
            textures,
        }
    }


    pub fn draw(&self, _app_state: &AppState, game_state: &GameState) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let perspective = glam::Mat4::perspective_rh_gl(
            90.0_f32.to_radians(),
            (self.viewport.w as f32) / (self.viewport.h as f32),
            0.1,
            100.0,
        );

        let view = glam::Mat4::from_rotation_x(game_state.player_rotation[1]);
        let view = view * glam::Mat4::from_rotation_y(game_state.player_rotation[0]);
        let model = glam::Mat4::from_translation(-game_state.player_position);
        let mv = view * model;
        let mvp = perspective * mv;

        self.shaders.colored_mesh.set_used();
        self.shaders.colored_mesh.set_mvp(&mvp);
        self.meshes.triangle.draw();

        self.shaders.mesh.set_used();
        self.shaders.mesh.set_mvp(&mvp);
        self.shaders.mesh.set_color(&Vec4::new(1.0, 1.0, 1.0, 1.0));
        self.textures.blocks.bind();
        self.meshes.ground_plane.draw();
        self.window.gl_swap_window();
    }
}