// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::Sfx;
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct SfxList {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,

    pub bomb: Sfx,
    pub jump: Sfx,
    pub hook_fire: Sfx,
    pub stomp: Sfx,
    pub pock: Sfx,
    pub tock: Sfx,
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

    pub fn play(&self, sfx: &Sfx, volume: f32) {
        let source = sfx.get_buf();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.detach();
    }
}
