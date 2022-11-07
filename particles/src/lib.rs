// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{IVec3, Mat4, Vec3};
use glium::implement_vertex;
use glium::Surface;
use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rgb::RGBA8;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use std::thread;
use wolkenwelten_common::SyncEvent;
use wolkenwelten_common::{GameEvent, Message, ParticleEvent};

#[derive(Copy, Clone, Debug)]
struct ParticleVertex {
    pos: [f32; 4],
    vel: [f32; 4],
    color: [u8; 4],
}
implement_vertex!(ParticleVertex, pos normalize(false), color normalize(true));

#[derive(Clone, Debug)]
pub struct ParticleMesh {
    particles: Arc<RwLock<Vec<ParticleVertex>>>,
}

impl Default for ParticleMesh {
    fn default() -> Self {
        Self {
            particles: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl ParticleMesh {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> Option<usize> {
        if let Ok(part) = self.particles.try_read() {
            Some(part.len())
        } else {
            None
        }
    }

    fn update(
        particles: Arc<RwLock<Vec<ParticleVertex>>>,
        player_pos: glam::Vec3,
        render_distance: f32,
    ) {
        if let Ok(mut part) = particles.write() {
            part.retain_mut(|p| {
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
    }

    fn fx_explosion(rng: &mut ChaCha8Rng, pos: Vec3, power: f32) -> Vec<ParticleVertex> {
        let power = power * 0.66;
        let mut n = Vec::with_capacity(256 + 128 + 64);
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
            n.push(ParticleVertex { pos, vel, color });
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
            n.push(ParticleVertex { pos, vel, color });
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
            n.push(ParticleVertex { pos, vel, color });
        }
        n
    }

    fn fx_block_break(rng: &mut ChaCha8Rng, pos: IVec3, color: [RGBA8; 2]) -> Vec<ParticleVertex> {
        let mut n = Vec::with_capacity(4 * 64);
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
                    ((color.r as f32 / 255.0 * rng.gen_range(0.6..1.1)).clamp(0.0, 1.0) * 255.0)
                        as u8,
                    ((color.g as f32 / 255.0 * rng.gen_range(0.6..1.1)).clamp(0.0, 1.0) * 255.0)
                        as u8,
                    ((color.b as f32 / 255.0 * rng.gen_range(0.6..1.1)).clamp(0.0, 1.0) * 255.0)
                        as u8,
                    0xFF,
                ];
                n.push(ParticleVertex { pos, vel, color });
            }
        }
        n
    }

    fn fx_block_place(rng: &mut ChaCha8Rng, pos: IVec3, color: [RGBA8; 2]) -> Vec<ParticleVertex> {
        Self::fx_block_break(rng, pos, color)
    }

    fn dispatch(
        particles: Arc<RwLock<Vec<ParticleVertex>>>,
        rng: &mut ChaCha8Rng,
        msg: &Message,
    ) -> bool {
        let n = match msg {
            Message::ParticleEvent(e) => match e {
                ParticleEvent::Explosion(pos, power) => Self::fx_explosion(rng, *pos, *power),
                ParticleEvent::BlockBreak(pos, color) => Self::fx_block_break(rng, *pos, *color),
                ParticleEvent::BlockPlace(pos, color) => Self::fx_block_place(rng, *pos, *color),
            },
            Message::GameEvent(GameEvent::EntityCollision(pos)) => {
                Self::fx_explosion(rng, *pos, 4.0)
            }
            Message::SyncEvent(SyncEvent::GameQuit(_)) => return false,
            Message::SyncEvent(SyncEvent::ParticleTick(player_pos, render_distance)) => {
                Self::update(particles, *player_pos, *render_distance);
                return true;
            }
            _ => return true,
        };
        if let Ok(mut part) = particles.write() {
            part.extend(n);
        };
        true
    }

    pub fn fork_sink(&self, rx: Receiver<Vec<Message>>) {
        let part = self.particles.clone();
        thread::spawn(move || {
            let mut rng = ChaCha8Rng::from_entropy();
            loop {
                let msg = rx.recv().expect("Particles: error receiving message");
                for msg in msg {
                    if !Self::dispatch(part.clone(), &mut rng, &msg) {
                        return;
                    }
                }
            }
        });
    }

    pub fn draw(
        &self,
        frame: &mut glium::Frame,
        display: &glium::Display,
        program: &glium::Program,
        mvp: &Mat4,
    ) -> Result<(), glium::DrawError> {
        let buffer: glium::VertexBuffer<ParticleVertex> = {
            if let Ok(data) = self.particles.read() {
                glium::VertexBuffer::dynamic(display, data.as_slice()).unwrap()
            } else {
                return Ok(());
            }
        };

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
