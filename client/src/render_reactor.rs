// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::render::chungus::chungus_pass;
use crate::ClientState;
use glam::{Mat4, Vec3};
use glium::Surface;
use wolkenwelten_common::{ChunkRequestQueue, Message, Reactor, CHUNK_SIZE};
use wolkenwelten_game::GameState;

pub struct RenderPassArgs<'a> {
    pub frame: &'a mut glium::Frame,
    pub fe: &'a ClientState,
    pub game: &'a GameState,
    pub projection: Mat4,
    pub view: Mat4,
    pub request: &'a mut ChunkRequestQueue,
    pub render_distance: f32,
    pub render_reactor: &'a RenderReactor,
}

pub struct RenderInitArgs<'a> {
    pub reactor: &'a mut Reactor<Message>,
    pub render_reactor: &'a mut RenderReactor,
    pub fe: &'a ClientState,
    pub game: &'a GameState,
}

pub type RenderPass = Box<dyn Fn(RenderPassArgs) -> RenderPassArgs>;
pub type RenderInit = Box<dyn Fn(RenderInitArgs) -> RenderInitArgs>;
pub type EntityProvider = Box<dyn Fn(&mut Vec<Vec3>)>;
pub struct RenderReactor {
    pub pre_world_render: Vec<RenderPass>,
    pub world_render: Vec<RenderPass>,
    pub post_world_render: Vec<RenderPass>,

    pub hud_3d_render: Vec<RenderPass>,
    pub hud_2d_render: Vec<RenderPass>,

    pub entity_provider: Vec<EntityProvider>,
}

fn clear_pass(mut a: RenderPassArgs) -> RenderPassArgs {
    let (window_width, window_height) = a.fe.window_size();
    a.frame.clear(
        None,
        Some((0.05, 0.13, 0.96, 1.0)),
        true,
        Some(4096.0),
        None,
    );
    a.projection = Mat4::perspective_rh_gl(
        a.fe.fov().to_radians(),
        (window_width as f32) / (window_height as f32),
        0.1,
        a.render_distance + CHUNK_SIZE as f32 * 2.0,
    );

    a
}

fn view_pass(mut a: RenderPassArgs) -> RenderPassArgs {
    let view = Mat4::from_rotation_x(a.game.player().rot[1].to_radians());
    let view = view * Mat4::from_rotation_y(a.game.player().rot[0].to_radians());
    a.view = view * Mat4::from_translation(-a.game.player().pos);
    a
}

impl Default for RenderReactor {
    fn default() -> Self {
        Self {
            pre_world_render: vec![Box::new(clear_pass)],
            world_render: vec![Box::new(view_pass), Box::new(chungus_pass)],
            post_world_render: vec![],

            hud_3d_render: vec![],
            hud_2d_render: vec![],

            entity_provider: vec![],
        }
    }
}

impl RenderReactor {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn entity_pos(&self) -> Vec<Vec3> {
        let mut r = vec![];
        for p in self.entity_provider.iter() {
            p(&mut r);
        }
        r
    }

    pub fn init(
        &mut self,
        reactor: &mut Reactor<Message>,
        fe: &ClientState,
        game: &GameState,
        init: Vec<RenderInit>,
    ) {
        let args = RenderInitArgs {
            render_reactor: self,
            reactor,
            fe,
            game,
        };
        init.iter().fold(args, |args, λ| λ(args));
    }

    pub fn run(
        &self,
        frame: &mut glium::Frame,
        fe: &ClientState,
        game: &GameState,
        request: &mut ChunkRequestQueue,
        render_distance: f32,
    ) {
        let args = RenderPassArgs {
            frame,
            fe,
            game,
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            request,
            render_distance,
            render_reactor: self,
        };
        let iter = self.hud_2d_render.iter();
        let iter = self.hud_3d_render.iter().chain(iter);
        let iter = self.post_world_render.iter().chain(iter);
        let iter = self.world_render.iter().chain(iter);
        let iter = self.pre_world_render.iter().chain(iter);
        iter.fold(args, |args, λ| λ(args));
    }
}
