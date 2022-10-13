extern crate sdl2;

use sdl2::{EventPump, Sdl, TimerSubsystem, VideoSubsystem};
use sdl2::video::{GLContext, Window};

use crate::{AppState, GameState};
use crate::input::InputState;
use crate::render::mesh::TextMesh;
use crate::render::state::{MeshList, TextureList};
use crate::render::state::ShaderList;
use crate::render::Viewport;

pub struct RenderState {
	pub sdl_context: Sdl,
	pub video_subsystem: VideoSubsystem,
	pub timer_subsystem: TimerSubsystem,
	pub gl_context: GLContext,
	pub window: Window,
	pub event_pump: EventPump,

	pub viewport: Viewport,
	pub meshes: MeshList,
	pub shaders: ShaderList,
	pub textures: TextureList,

	pub ui_mesh: TextMesh,

	pub input: InputState,

	pub cur_fps: u32,
	pub frame_count: u32,
	pub last_ticks: u32,
}

impl RenderState {
	pub fn new() -> RenderState {
		let window_width: i32 = 640;
		let window_height: i32 = 480;

		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();
		let timer_subsystem = sdl_context.timer().unwrap();
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

		let event_pump = sdl_context.event_pump().unwrap();
		let last_ticks = timer_subsystem.ticks();

		RenderState {
			sdl_context,
			video_subsystem,
			timer_subsystem,
			gl_context,
			window,
			event_pump,
			viewport: Viewport::for_window(window_width, window_height),
			meshes: MeshList::new(),
			shaders: ShaderList::new(),
			input:  InputState::new(),
			textures: TextureList::new(),
			ui_mesh: TextMesh::new(),

			cur_fps: 0,
			frame_count: 0,
			last_ticks,
		}
	}

	pub fn fps(&self) -> u32 { self.cur_fps }

	pub fn calc_fps(&mut self) -> &mut RenderState {
		let ticks = self.timer_subsystem.ticks();
		if ticks > self.last_ticks + 1000 {
			self.cur_fps = (((self.frame_count as f32) / ((ticks - self.last_ticks) as f32)) * 1000.0) as u32;
			self.last_ticks = ticks;
			self.frame_count = 0;
		}
		self.frame_count += 1;
		self
	}

	pub fn prepare(&mut self, _app_state: &mut AppState, game_state: &mut GameState) -> &mut RenderState {
		let fps_text = format!("FPS: {}", self.fps());
		let pos_text = format!("X:{:8.2} Y:{:8.2} Z:{:8.2}", game_state.player_position[0], game_state.player_position[1], game_state.player_position[2]);
		let rot_text = format!("Y:{:8.2} P:{:8.2} R:{:8.2}", game_state.player_rotation[0], game_state.player_rotation[1], game_state.player_rotation[2]);
		self.ui_mesh.empty()
			.push_string(8,8,2,0xFFFFFFFF, fps_text.as_str())
			.push_string(8,40,1,0xFFFFFFFF, pos_text.as_str())
			.push_string(8,50,1,0xFFFFFFFF, rot_text.as_str())
			.prepare();
		self.calc_fps()
	}

	pub fn draw(&self, _app_state: &AppState, game_state: &GameState) {
		self.viewport.set_used();
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		};

		game_state.draw(self);


		let perspective = glam::Mat4::orthographic_rh_gl(
			0.0,
			self.viewport.w as f32,

			self.viewport.h as f32,
			0.0,

			-10.0,
			10.0,
		);

		self.shaders.text_shader.set_used();
		self.shaders.text_shader.set_mvp(&perspective);
		self.textures.gui.bind();
		self.ui_mesh.draw();

		self.window.gl_swap_window();
	}
}
