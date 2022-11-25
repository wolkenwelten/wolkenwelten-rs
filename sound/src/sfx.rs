// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use core::f32::consts::PI;
use glam::Vec3;
use rodio::{
    buffer::SamplesBuffer, source::Buffered, Decoder, OutputStream, OutputStreamHandle, Sink,
    Source, SpatialSink,
};
use std::io::Cursor;
use wolkenwelten_common::{GameEvent, Message};

struct Sfx {
    buf: Buffered<SamplesBuffer<i16>>,
}

impl Sfx {
    pub fn from_bytes(bytes: &'static [u8]) -> Self {
        let dec = Decoder::new(Cursor::new(bytes)).unwrap();
        let sample_rate = dec.sample_rate();
        let channels = dec.channels();
        let buf: Vec<i16> = dec.collect();
        let buf = SamplesBuffer::new(channels, sample_rate, buf).buffered();
        Self { buf }
    }

    pub fn get_buf(&self) -> Buffered<SamplesBuffer<i16>> {
        self.buf.clone()
    }
}

pub struct SfxList {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,

    bomb: Sfx,
    jump: Sfx,
    hook_fire: Sfx,
    ungh: Sfx,
    step: Sfx,
    stomp: Sfx,
    pock: Sfx,
    tock: Sfx,
}

impl Default for SfxList {
    fn default() -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            _stream,
            stream_handle,

            bomb: Sfx::from_bytes(include_bytes!("../../assets/sfx/bomb.ogg")),
            jump: Sfx::from_bytes(include_bytes!("../../assets/sfx/jump.ogg")),
            hook_fire: Sfx::from_bytes(include_bytes!("../../assets/sfx/hookFire.ogg")),
            ungh: Sfx::from_bytes(include_bytes!("../../assets/sfx/ungh.ogg")),
            step: Sfx::from_bytes(include_bytes!("../../assets/sfx/step.ogg")),
            stomp: Sfx::from_bytes(include_bytes!("../../assets/sfx/stomp.ogg")),
            pock: Sfx::from_bytes(include_bytes!("../../assets/sfx/pock.ogg")),
            tock: Sfx::from_bytes(include_bytes!("../../assets/sfx/tock.ogg")),
        }
    }
}

impl SfxList {
    pub fn new() -> Self {
        Default::default()
    }

    fn _play(&self, sfx: &Sfx, volume: f32) {
        let source = sfx.get_buf();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.detach();
    }

    fn planar_direction(deg: f32) -> Vec3 {
        Vec3::new(
            ((deg - 90.0) * PI / 180.0).cos(),
            0.0,
            ((deg - 90.0) * PI / 180.0).sin(),
        )
    }

    fn play_spatial(
        &self,
        sfx: &Sfx,
        volume: f32,
        player_pos: Vec3,
        player_rot: Vec3,
        emitter_pos: Vec3,
    ) {
        let source = sfx.get_buf();
        let player_pos = player_pos / 8.0;
        let emitter_pos = emitter_pos / 8.0;
        let left = player_pos + (Self::planar_direction(player_rot.x - 90.0) * 0.3);
        let right = player_pos + (Self::planar_direction(player_rot.x + 90.0) * 0.3);
        let emitter = emitter_pos.into();
        let sink =
            SpatialSink::try_new(&self.stream_handle, emitter, left.into(), right.into()).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.detach();
    }

    pub fn msg_sink(&self, msg: &Vec<Message>) {
        let mut player_pos = Vec3::ZERO;
        let mut player_rot = Vec3::ZERO;

        msg.iter().for_each(|e| match e {
            Message::GameEvent(m) => match m {
                GameEvent::CharacterPosRotVel(pos, rot, _) => {
                    player_pos = *pos;
                    player_rot = *rot;
                }
                GameEvent::CharacterJump(pos) => {
                    self.play_spatial(&self.jump, 0.1, player_pos, player_rot, *pos)
                }
                GameEvent::CharacterShoot(pos) => {
                    self.play_spatial(&self.hook_fire, 0.4, player_pos, player_rot, *pos)
                }
                GameEvent::CharacterDamage(pos, _) => {
                    self.play_spatial(&self.ungh, 0.3, player_pos, player_rot, *pos)
                }
                GameEvent::CharacterDeath(pos) => {
                    self.play_spatial(&self.ungh, 0.4, player_pos, player_rot, *pos)
                }
                GameEvent::CharacterStep(pos) => {
                    self.play_spatial(&self.step, 0.2, player_pos, player_rot, *pos)
                }
                GameEvent::BlockBreak(pos, _) => {
                    self.play_spatial(&self.tock, 0.3, player_pos, player_rot, pos.as_vec3())
                }
                GameEvent::BlockMine(pos, _) => {
                    self.play_spatial(&self.tock, 0.1, player_pos, player_rot, pos.as_vec3())
                }
                GameEvent::BlockPlace(pos, _) => {
                    self.play_spatial(&self.pock, 0.3, player_pos, player_rot, pos.as_vec3())
                }
                GameEvent::CharacterStomp(pos) => {
                    self.play_spatial(&self.stomp, 0.3, player_pos, player_rot, *pos)
                }
                GameEvent::EntityCollision(pos) => {
                    self.play_spatial(&self.bomb, 1.0, player_pos, player_rot, *pos)
                }
                GameEvent::ItemDropPickup(pos, _) => {
                    self.play_spatial(&self.pock, 0.3, player_pos, player_rot, *pos)
                } //_ => (),
            },
            _ => (),
        });
    }
}
