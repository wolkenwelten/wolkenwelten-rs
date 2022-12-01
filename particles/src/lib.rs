// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{IVec3, Mat4, Vec3};
use glium::implement_vertex;
use glium::Surface;
use palette::{convert::FromColor, Hsv, Pixel, Srgb};
use rand::prelude::*;
use rand::Rng;
use rand_xorshift::XorShiftRng;
use rgb::RGBA8;
use std::cell::RefCell;
use std::rc::Rc;
use wolkenwelten_client::{RenderInitArgs, ShaderList};
use wolkenwelten_common::{Message, Reactor};

/// The single particle, will be sent as is to the shader as is.
#[derive(Copy, Clone, Debug)]
struct ParticleVertex {
    pos: [f32; 4],
    vel: [f32; 4],
    color: [u8; 4],
}
implement_vertex!(ParticleVertex, pos normalize(false), color normalize(true));

#[derive(Clone, Debug)]
pub struct ParticleMesh {
    particles: Vec<ParticleVertex>,
    rng: XorShiftRng,
}

impl Default for ParticleMesh {
    fn default() -> Self {
        Self {
            particles: vec![],
            rng: XorShiftRng::from_entropy(),
        }
    }
}

impl ParticleMesh {
    /// Construct a new ParticleMesh, for the most part a single ParticleMesh should
    /// suffice for the entire client, since we just add particles to that mesh, and update
    /// it every frame.
    pub fn new() -> Self {
        Default::default()
    }

    /// Here we do the actual particle updates, as soon as a particle's size goes below 1,
    /// or it is far away it will be removed from the list.
    fn update(&mut self, player_pos: Vec3, delta: f32, render_distance: f32) {
        self.particles.retain_mut(|p| {
            p.vel[1] -= 0.001 * delta;
            p.pos[0] += p.vel[0] * delta;
            p.pos[1] += p.vel[1] * delta;
            p.pos[2] += p.vel[2] * delta;
            p.pos[3] += p.vel[3] * delta;

            let dx = (player_pos.x - p.pos[0]).abs();
            let dy = (player_pos.y - p.pos[1]).abs();
            let dz = (player_pos.z - p.pos[2]).abs();
            let d = dx + dy + dz;

            (p.pos[3] > 1.0) && (d < (render_distance * 2.0))
        })
    }

    /// Return a new color with the saturation/lightness shifted by +-shift
    fn hsv_shift(&mut self, color: Srgb, saturation_shift: f32, lightness_shift: f32) -> Srgb {
        let sr = saturation_shift;
        let ls = lightness_shift;
        let color: Hsv = Hsv::from_color(color);
        let color = color.into_components();
        let color = (
            color.0,
            color.1 * self.rng.gen_range(1.0 - sr..1.0 + sr),
            color.2 * self.rng.gen_range(1.0 - ls..1.0 + ls),
        );
        let color = Hsv::from_components(color);
        Srgb::from_color(color)
    }

    /// Create a new effect that should resemble an explosion, power determines the
    /// overall size of the fireball.
    fn fx_explosion(&mut self, pos: Vec3, power: f32) {
        let power = power * 0.66;
        for _ in 1..512 {
            let pos = [
                pos.x + self.rng.gen_range(-power..power),
                pos.y + self.rng.gen_range(-power..power),
                pos.z + self.rng.gen_range(-power..power),
                192.0,
            ];
            let vel = [
                self.rng.gen_range(-0.3..0.3),
                self.rng.gen_range(-0.2..0.4),
                self.rng.gen_range(-0.3..0.3),
                -7.0,
            ];
            let color = [
                self.rng.gen_range(0xf0..0xff),
                self.rng.gen_range(0x30..0x50),
                self.rng.gen_range(0x18..0x28),
                0xFF,
            ];
            self.particles.push(ParticleVertex { pos, vel, color });
        }

        let power = power * 0.66;
        for _ in 1..256 {
            let pos = [
                pos.x + self.rng.gen_range(-power..power),
                pos.y + self.rng.gen_range(-power..power),
                pos.z + self.rng.gen_range(-power..power),
                256.0,
            ];
            let vel = [
                self.rng.gen_range(-0.3..0.3),
                self.rng.gen_range(-0.2..0.4),
                self.rng.gen_range(-0.3..0.3),
                -10.0,
            ];
            let color = [
                self.rng.gen_range(0xc0..0xe0),
                self.rng.gen_range(0x10..0x18),
                self.rng.gen_range(0x04..0x0a),
                0xFF,
            ];
            self.particles.push(ParticleVertex { pos, vel, color });
        }

        let power = power * 0.66;
        for _ in 1..128 {
            let pos = [
                pos.x + self.rng.gen_range(-power..power),
                pos.y + self.rng.gen_range(-power..power),
                pos.z + self.rng.gen_range(-power..power),
                320.0,
            ];
            let vel = [
                self.rng.gen_range(-0.4..0.4),
                self.rng.gen_range(-0.3..0.5),
                self.rng.gen_range(-0.4..0.4),
                -13.0,
            ];
            let color = [
                self.rng.gen_range(0xa0..0xc0),
                self.rng.gen_range(0x08..0x10),
                self.rng.gen_range(0x04..0x0a),
                0xFF,
            ];
            self.particles.push(ParticleVertex { pos, vel, color });
        }
    }

    /// Create a breaking block effect, the color will be hsv shifted so that
    /// it doesn't look quite as boring/uniform.
    fn fx_block_break(&mut self, pos: IVec3, color: [RGBA8; 2], part_count: usize) {
        for color in color.iter() {
            let color: Srgb = Srgb::from_components((
                color.r as f32 / 256.0,
                color.g as f32 / 256.0,
                color.b as f32 / 256.0,
            ));
            for _ in 0..part_count {
                let pos = [
                    pos.x as f32 + self.rng.gen_range(0.0..1.0),
                    pos.y as f32 + self.rng.gen_range(0.0..1.0),
                    pos.z as f32 + self.rng.gen_range(0.0..1.0),
                    200.0,
                ];
                let vel = [
                    self.rng.gen_range(-0.02..0.02),
                    self.rng.gen_range(0.00..0.04),
                    self.rng.gen_range(-0.02..0.02),
                    -13.0,
                ];

                let color = self.hsv_shift(color, 0.1, 0.1);
                let color: [u8; 3] = color.into_format().into_raw();
                let color = [color[0], color[1], color[2], 0xFF];
                self.particles.push(ParticleVertex { pos, vel, color });
            }
        }
    }

    /// Create a new block placement effect, for now it just relays everything to the block break
    /// method.
    fn fx_block_place(&mut self, pos: IVec3, color: [RGBA8; 2]) {
        self.fx_block_break(pos, color, 64)
    }

    /// Draw all the active particles
    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        display: &glium::Display,
        program: &glium::Program,
        mvp: &Mat4,
    ) -> Result<(), glium::DrawError> {
        if self.particles.is_empty() {
            return Ok(());
        }

        let data = self.particles.as_slice();
        let buffer: glium::VertexBuffer<ParticleVertex> =
            glium::VertexBuffer::dynamic(display, data).unwrap();
        let mat_mvp = mvp.to_cols_array_2d();
        let size_mul: f32 = 1.0;

        frame.draw(
            &buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::Points),
            program,
            &glium::uniform! {
                mat_mvp: mat_mvp,
                size_mul: size_mul,
            },
            &glium::DrawParameters {
                depth: glium::draw_parameters::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    }
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    let particles: Rc<RefCell<ParticleMesh>> = Rc::new(RefCell::new(ParticleMesh::new()));
    {
        let program = ShaderList::new_point_program(
            &args.fe.display,
            include_str!("./particle.vert"),
            include_str!("./particle.frag"),
        )
        .expect("Error while compiling particle shader");
        let particles = particles.clone();
        args.render_reactor
            .post_world_render
            .push(Box::new(move |args| {
                let mvp = args.projection * args.view;
                {
                    let particles = particles.borrow();
                    let _ = particles.draw(args.frame, &args.fe.display, &program, &mvp);
                }
                args
            }));
    }
    {
        let particles = particles.clone();
        let last_update = RefCell::new(0);
        let f = move |_: &Reactor<Message>, msg: Message| {
            if let Message::DrawFrame {
                player_pos,
                ticks,
                render_distance,
                ..
            } = msg
            {
                let delta = (ticks - *last_update.borrow()) as f32 / (1000.0 / 60.0);
                last_update.replace(ticks);
                particles
                    .borrow_mut()
                    .update(player_pos, delta, render_distance)
            }
        };
        args.reactor.add_sink(
            Message::DrawFrame {
                player_pos: Vec3::ZERO,
                ticks: 0,
                render_distance: 0.0,
            },
            Box::new(f),
        );
    }
    {
        let particles = particles.clone();
        let blocks = args.game.world().blocks.clone();
        let f = move |_: &Reactor<Message>, msg: Message| {
            if let Message::BlockBreak { pos, block } = msg {
                if let Some(bt) = blocks.borrow().get(block as usize) {
                    let color = bt.colors();
                    particles.borrow_mut().fx_block_break(pos, color, 128);
                }
            }
        };
        args.reactor.add_sink(
            Message::BlockBreak {
                pos: IVec3::ZERO,
                block: 0,
            },
            Box::new(f),
        );
    }
    {
        let particles = particles.clone();
        let blocks = args.game.world().blocks.clone();
        let f = move |_: &Reactor<Message>, msg: Message| {
            if let Message::BlockMine { pos, block } = msg {
                if let Some(bt) = blocks.borrow().get(block as usize) {
                    let color = bt.colors();
                    particles.borrow_mut().fx_block_break(pos, color, 2);
                }
            }
        };
        args.reactor.add_sink(
            Message::BlockMine {
                pos: IVec3::ZERO,
                block: 0,
            },
            Box::new(f),
        );
    }
    {
        let particles = particles.clone();
        let blocks = args.game.world().blocks.clone();
        let f = move |_: &Reactor<Message>, msg: Message| {
            if let Message::BlockPlace { pos, block } = msg {
                if let Some(bt) = blocks.borrow().get(block as usize) {
                    let color = bt.colors();
                    particles.borrow_mut().fx_block_place(pos, color);
                }
            }
        };
        args.reactor.add_sink(
            Message::BlockPlace {
                pos: IVec3::ZERO,
                block: 0,
            },
            Box::new(f),
        );
    }
    {
        let f = move |_: &Reactor<Message>, msg: Message| {
            if let Message::EntityCollision { pos } = msg {
                particles.borrow_mut().fx_explosion(pos, 9.0);
            }
        };
        args.reactor
            .add_sink(Message::EntityCollision { pos: Vec3::ZERO }, Box::new(f));
    }
    args
}
