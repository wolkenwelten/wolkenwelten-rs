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
use wolkenwelten_common::{BlockType, Message, Reactor};

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
    particles: Rc<RefCell<Vec<ParticleVertex>>>,
    rng: Rc<RefCell<XorShiftRng>>,
}

impl Default for ParticleMesh {
    fn default() -> Self {
        Self {
            particles: Rc::new(RefCell::new(vec![])),
            rng: Rc::new(RefCell::new(XorShiftRng::from_entropy())),
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

    /// Return whether there are any particles currently active
    pub fn is_empty(&self) -> bool {
        self.particles.borrow().is_empty()
    }

    /// Return the amount of particles currently active
    pub fn len(&self) -> usize {
        self.particles.borrow().len()
    }

    /// Here we do the actual particle updates, as soon as a particle's size goes below 1,
    /// or it is far away it will be removed from the list.
    fn update(
        particles: &mut Vec<ParticleVertex>,
        player_pos: Vec3,
        delta: f32,
        render_distance: f32,
    ) {
        particles.retain_mut(|p| {
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
    fn hsv_shift(
        rng: &mut XorShiftRng,
        color: Srgb,
        saturation_shift: f32,
        lightness_shift: f32,
    ) -> Srgb {
        let sr = saturation_shift;
        let ls = lightness_shift;
        let color: Hsv = Hsv::from_color(color);
        let color = color.into_components();
        let color = (
            color.0,
            color.1 * rng.gen_range(1.0 - sr..1.0 + sr),
            color.2 * rng.gen_range(1.0 - ls..1.0 + ls),
        );
        let color = Hsv::from_components(color);
        Srgb::from_color(color)
    }

    /// Create a new effect that should resemble an explosion, power determines the
    /// overall size of the fireball.
    fn fx_explosion(
        particles: &mut Vec<ParticleVertex>,
        rng: &mut XorShiftRng,
        pos: Vec3,
        power: f32,
    ) {
        let power = power * 0.66;
        for _ in 1..512 {
            let pos = [
                pos.x + rng.gen_range(-power..power),
                pos.y + rng.gen_range(-power..power),
                pos.z + rng.gen_range(-power..power),
                192.0,
            ];
            let vel = [
                rng.gen_range(-0.3..0.3),
                rng.gen_range(-0.2..0.4),
                rng.gen_range(-0.3..0.3),
                -7.0,
            ];
            let color = [
                rng.gen_range(0xf0..0xff),
                rng.gen_range(0x30..0x50),
                rng.gen_range(0x18..0x28),
                0xFF,
            ];
            particles.push(ParticleVertex { pos, vel, color });
        }

        let power = power * 0.66;
        for _ in 1..256 {
            let pos = [
                pos.x + rng.gen_range(-power..power),
                pos.y + rng.gen_range(-power..power),
                pos.z + rng.gen_range(-power..power),
                256.0,
            ];
            let vel = [
                rng.gen_range(-0.3..0.3),
                rng.gen_range(-0.2..0.4),
                rng.gen_range(-0.3..0.3),
                -10.0,
            ];
            let color = [
                rng.gen_range(0xc0..0xe0),
                rng.gen_range(0x10..0x18),
                rng.gen_range(0x04..0x0a),
                0xFF,
            ];
            particles.push(ParticleVertex { pos, vel, color });
        }

        let power = power * 0.66;
        for _ in 1..128 {
            let pos = [
                pos.x + rng.gen_range(-power..power),
                pos.y + rng.gen_range(-power..power),
                pos.z + rng.gen_range(-power..power),
                320.0,
            ];
            let vel = [
                rng.gen_range(-0.4..0.4),
                rng.gen_range(-0.3..0.5),
                rng.gen_range(-0.4..0.4),
                -13.0,
            ];
            let color = [
                rng.gen_range(0xa0..0xc0),
                rng.gen_range(0x08..0x10),
                rng.gen_range(0x04..0x0a),
                0xFF,
            ];
            particles.push(ParticleVertex { pos, vel, color });
        }
    }

    /// Create a breaking block effect, the color will be hsv shifted so that
    /// it doesn't look quite as boring/uniform.
    fn fx_block_break(
        particles: &mut Vec<ParticleVertex>,
        rng: &mut XorShiftRng,
        pos: IVec3,
        color: [RGBA8; 2],
        part_count: usize,
    ) {
        for color in color.iter() {
            let color: Srgb = Srgb::from_components((
                color.r as f32 / 256.0,
                color.g as f32 / 256.0,
                color.b as f32 / 256.0,
            ));
            for _ in 0..part_count {
                let pos = [
                    pos.x as f32 + rng.gen_range(0.0..1.0),
                    pos.y as f32 + rng.gen_range(0.0..1.0),
                    pos.z as f32 + rng.gen_range(0.0..1.0),
                    200.0,
                ];
                let vel = [
                    rng.gen_range(-0.02..0.02),
                    rng.gen_range(0.00..0.04),
                    rng.gen_range(-0.02..0.02),
                    -13.0,
                ];

                let color = Self::hsv_shift(rng, color, 0.1, 0.1);
                let color: [u8; 3] = color.into_format().into_raw();
                let color = [color[0], color[1], color[2], 0xFF];
                particles.push(ParticleVertex { pos, vel, color });
            }
        }
    }

    /// Create a new block placement effect, for now it just relays everything to the block break
    /// method.
    fn fx_block_place(
        particles: &mut Vec<ParticleVertex>,
        rng: &mut XorShiftRng,
        pos: IVec3,
        color: [RGBA8; 2],
    ) {
        Self::fx_block_break(particles, rng, pos, color, 64)
    }

    pub fn add_handler(&self, reactor: &mut Reactor<Message>, blocks: Rc<RefCell<Vec<BlockType>>>) {
        {
            let particles = self.particles.clone();
            let last_update = RefCell::new(0);
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::DrawFrame(player_pos, ticks, render_distance) = msg {
                    let delta = (ticks - *last_update.borrow()) as f32 / (1000.0 / 60.0);
                    last_update.replace(ticks);
                    Self::update(
                        &mut particles.borrow_mut(),
                        player_pos,
                        delta,
                        render_distance,
                    )
                }
            };
            reactor.add_sink(Message::DrawFrame(Vec3::ZERO, 0, 0.0), Box::new(f));
        }
        {
            let particles = self.particles.clone();
            let rng = self.rng.clone();
            let blocks = blocks.clone();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::BlockBreak(pos, b) = msg {
                    if let Some(bt) = blocks.borrow().get(b as usize) {
                        let color = bt.colors();
                        Self::fx_block_break(
                            &mut particles.borrow_mut(),
                            &mut rng.borrow_mut(),
                            pos,
                            color,
                            128,
                        );
                    }
                }
            };
            reactor.add_sink(Message::BlockBreak(IVec3::ZERO, 0), Box::new(f));
        }
        {
            let particles = self.particles.clone();
            let rng = self.rng.clone();
            let blocks = blocks.clone();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::BlockMine(pos, b) = msg {
                    if let Some(bt) = blocks.borrow().get(b as usize) {
                        let color = bt.colors();
                        Self::fx_block_break(
                            &mut particles.borrow_mut(),
                            &mut rng.borrow_mut(),
                            pos,
                            color,
                            2,
                        );
                    }
                }
            };
            reactor.add_sink(Message::BlockMine(IVec3::ZERO, 0), Box::new(f));
        }
        {
            let particles = self.particles.clone();
            let rng = self.rng.clone();
            let blocks = blocks.clone();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::BlockPlace(pos, b) = msg {
                    if let Some(bt) = blocks.borrow().get(b as usize) {
                        let color = bt.colors();
                        Self::fx_block_place(
                            &mut particles.borrow_mut(),
                            &mut rng.borrow_mut(),
                            pos,
                            color,
                        );
                    }
                }
            };
            reactor.add_sink(Message::BlockPlace(IVec3::ZERO, 0), Box::new(f));
        }
        {
            let particles = self.particles.clone();
            let rng = self.rng.clone();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::EntityCollision(pos) = msg {
                    Self::fx_explosion(
                        &mut particles.borrow_mut(),
                        &mut rng.borrow_mut(),
                        pos,
                        9.0,
                    );
                }
            };
            reactor.add_sink(Message::EntityCollision(Vec3::ZERO), Box::new(f));
        }
    }

    /// Draw all the active particles
    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        display: &glium::Display,
        program: &glium::Program,
        mvp: &Mat4,
    ) -> Result<(), glium::DrawError> {
        let particles = self.particles.borrow();
        if particles.is_empty() {
            return Ok(());
        }

        let data = particles.as_slice();
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
