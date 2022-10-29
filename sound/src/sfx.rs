/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
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
