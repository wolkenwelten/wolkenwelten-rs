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
use wolkenwelten_common::{BlockType, GameEvent, Message, SyncEvent};

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
    last_update: u64,
}

impl Default for ParticleMesh {
    fn default() -> Self {
        Self {
            particles: vec![],
            last_update: 0,
            rng: XorShiftRng::from_entropy(),
        }
    }
}

impl ParticleMesh {
    /// Construct a new ParticleMesh
    pub fn new() -> Self {
        Default::default()
    }

    /// Return whether there are any particles currently active
    pub fn is_empty(&self) -> bool {
        self.particles.is_empty()
    }

    /// Return the amount of particles currently active
    pub fn len(&self) -> usize {
        self.particles.len()
    }

    /// Here we do the actual particle updates, as soon as a particle's size goes below 1,
    /// or it is far away it will be removed from the list.
    fn update(&mut self, player_pos: glam::Vec3, ticks: u64, render_distance: f32) {
        let delta = (ticks - self.last_update) as f32 / (1000.0 / 60.0);
        self.last_update = ticks;

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

    /// Interal function creating particles that resemble an explosion
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

    /// Interal function creating particles that look like a block breaking apart
    fn fx_block_break(&mut self, pos: IVec3, color: [RGBA8; 2]) {
        for color in color.iter() {
            let color: Srgb = Srgb::from_components((
                color.r as f32 / 256.0,
                color.g as f32 / 256.0,
                color.b as f32 / 256.0,
            ));
            for _ in 1..64 {
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

    /// Internal function creating an effect after a block has been placed
    fn fx_block_place(&mut self, pos: IVec3, color: [RGBA8; 2]) {
        self.fx_block_break(pos, color)
    }

    /// The particle message sink, should be called after the main game handler since
    /// here we produce particle effects based on events happening in the game.
    ///
    /// Also updates all the old particles whenever we receive a DrawFrame message.
    pub fn msg_sink(&mut self, msgs: &Vec<Message>, block_types: &Vec<BlockType>) {
        msgs.iter().for_each(|e| match e {
            Message::SyncEvent(SyncEvent::DrawFrame(player_pos, ticks, render_distance)) => {
                self.update(*player_pos, *ticks, *render_distance);
            }
            Message::GameEvent(e) => match e {
                GameEvent::BlockMine(pos, b) => {
                    let color = block_types[*b as usize].colors();
                    self.fx_block_break(*pos, color);
                }
                GameEvent::BlockPlace(pos, b) => {
                    let color = block_types[*b as usize].colors();
                    self.fx_block_place(*pos, color)
                }
                GameEvent::EntityCollision(pos) => {
                    self.fx_explosion(*pos, 9.0);
                }
                _ => (),
            },
            _ => (),
        });
    }

    /// Draw all the particles currently active
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
