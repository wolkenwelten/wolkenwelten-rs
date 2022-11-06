// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use rodio::{
    buffer::SamplesBuffer, source::Buffered, Decoder, OutputStream, OutputStreamHandle, Sink,
    Source,
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

    fn play(&self, sfx: &Sfx, volume: f32) {
        let source = sfx.get_buf();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.detach();
    }

    pub fn msg_sink(&self, msg: &Vec<Message>) {
        msg.iter().for_each(|e| match e {
            Message::GameEvent(m) => match m {
                GameEvent::CharacterJump(_) => self.play(&self.jump, 0.2),
                GameEvent::CharacterShoot(_) => self.play(&self.hook_fire, 0.4),
                GameEvent::CharacterDamage(_, _) => self.play(&self.ungh, 0.3),
                GameEvent::CharacterDeath(_) => self.play(&self.ungh, 0.4),
                GameEvent::BlockMine(_, _) => self.play(&self.tock, 0.3),
                GameEvent::BlockPlace(_, _) => self.play(&self.pock, 0.3),
                GameEvent::CharacterStomp(_) => self.play(&self.stomp, 0.3),
                GameEvent::EntityCollision(_) => self.play(&self.bomb, 0.3),
                _ => (),
            },
            _ => (),
        });
    }
}
