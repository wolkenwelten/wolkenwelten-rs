// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{IVec3, Mat4, Vec3};
use glium::implement_vertex;
use glium::Surface;
use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rgb::RGBA8;
use wolkenwelten_common::{Message, ParticleEvent};

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
    rng: ChaCha8Rng,
}

impl Default for ParticleMesh {
    fn default() -> Self {
        Self {
            particles: vec![],
            rng: ChaCha8Rng::from_entropy(),
        }
    }
}

impl ParticleMesh {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.particles.is_empty()
    }

    pub fn len(&self) -> usize {
        self.particles.len()
    }

    pub fn update(&mut self, player_pos: glam::Vec3, render_distance: f32) {
        self.particles.retain_mut(|p| {
            p.vel[1] -= 0.001;
            p.pos[0] += p.vel[0];
            p.pos[1] += p.vel[1];
            p.pos[2] += p.vel[2];
            p.pos[3] += p.vel[3];

            let dx = (player_pos.x - p.pos[0]).abs();
            let dy = (player_pos.y - p.pos[1]).abs();
            let dz = (player_pos.z - p.pos[2]).abs();
            let d = dx + dy + dz;

            (p.pos[3] > 1.0) && (d < (render_distance * 2.0))
        })
    }

    fn fx_explosion(&mut self, pos: Vec3, power: f32) {
        let power = power * 0.66;
        for _ in 1..256 {
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
        for _ in 1..128 {
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
        for _ in 1..64 {
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

    fn fx_block_break(&mut self, pos: IVec3, color: [RGBA8; 2]) {
        for color in color.iter() {
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
                    -23.0,
                ];
                let color = [
                    ((color.r as f32 / 255.0 * self.rng.gen_range(0.6..1.1)).clamp(0.0, 1.0)
                        * 255.0) as u8,
                    ((color.g as f32 / 255.0 * self.rng.gen_range(0.6..1.1)).clamp(0.0, 1.0)
                        * 255.0) as u8,
                    ((color.b as f32 / 255.0 * self.rng.gen_range(0.6..1.1)).clamp(0.0, 1.0)
                        * 255.0) as u8,
                    0xFF,
                ];
                self.particles.push(ParticleVertex { pos, vel, color });
            }
        }
    }

    fn fx_block_place(&mut self, pos: IVec3, color: [RGBA8; 2]) {
        self.fx_block_break(pos, color)
    }

    pub fn msg_sink(&mut self, msgs: &Vec<Message>) {
        msgs.iter().for_each(|e| match e {
            Message::ParticleEvent(e) => match e {
                ParticleEvent::Explosion(pos, power) => self.fx_explosion(*pos, *power),
                ParticleEvent::BlockBreak(pos, color) => self.fx_block_break(*pos, *color),
                ParticleEvent::BlockPlace(pos, color) => self.fx_block_place(*pos, *color),
            },
            _ => (),
        });
    }

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
