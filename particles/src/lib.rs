// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{IVec3, Mat4, Vec3};
use glium::implement_vertex;
use glium::Surface;
use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[derive(Copy, Clone, Debug)]
pub enum ParticleEmission {
    Explosion(Vec3, f32),
    BlockBreak(IVec3, [[u8; 4]; 2]),
    BlockPlace(IVec3, [[u8; 4]; 2]),
}

#[derive(Copy, Clone, Debug)]
struct ParticleVertex {
    pos: [f32; 4],
    vel: [f32; 4],
    color: [u8; 4],
}
implement_vertex!(ParticleVertex, pos normalize(false), color normalize(true));

#[derive(Clone, Debug, Default)]
pub struct ParticleMesh {
    particles: Vec<ParticleVertex>,
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

    fn fx_explosion(&mut self, rng: &mut ChaCha8Rng, pos: Vec3, power: f32) {
        let power = power * 0.66;
        for _ in 1..256 {
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
            self.particles.push(ParticleVertex { pos, vel, color });
        }

        let power = power * 0.66;
        for _ in 1..128 {
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
            self.particles.push(ParticleVertex { pos, vel, color });
        }

        let power = power * 0.66;
        for _ in 1..64 {
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
            self.particles.push(ParticleVertex { pos, vel, color });
        }
    }

    fn fx_block_break(&mut self, rng: &mut ChaCha8Rng, pos: IVec3, color: [[u8; 4]; 2]) {
        for color in color.iter() {
            for _ in 1..64 {
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
                    -23.0,
                ];
                let color = [
                    ((color[0] as f32 / 255.0 * rng.gen_range(0.6..1.1)).clamp(0.0, 1.0) * 255.0)
                        as u8,
                    ((color[1] as f32 / 255.0 * rng.gen_range(0.6..1.1)).clamp(0.0, 1.0) * 255.0)
                        as u8,
                    ((color[2] as f32 / 255.0 * rng.gen_range(0.6..1.1)).clamp(0.0, 1.0) * 255.0)
                        as u8,
                    0xFF,
                ];
                self.particles.push(ParticleVertex { pos, vel, color });
            }
        }
    }

    fn fx_block_place(&mut self, rng: &mut ChaCha8Rng, pos: IVec3, color: [[u8; 4]; 2]) {
        self.fx_block_break(rng, pos, color)
    }

    pub fn reduce_emissions(&mut self, emissions: &Vec<ParticleEmission>, rng_seed: u64) {
        if emissions.is_empty() {
            return;
        }
        let mut rng = ChaCha8Rng::seed_from_u64(rng_seed);
        emissions.iter().for_each(|e| match e {
            ParticleEmission::Explosion(pos, power) => self.fx_explosion(&mut rng, *pos, *power),
            ParticleEmission::BlockBreak(pos, color) => self.fx_block_break(&mut rng, *pos, *color),
            ParticleEmission::BlockPlace(pos, color) => self.fx_block_place(&mut rng, *pos, *color),
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
