// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use rodio::{buffer::SamplesBuffer, source::Buffered, Decoder, Source};
use std::io::Cursor;

pub struct Sfx {
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
