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
use super::Sfx;
use rodio::{Decoder, Sink};
use rodio::{OutputStream, OutputStreamHandle};
use std::io::Cursor;

pub struct SfxList {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,

    pub bomb: Sfx,
    pub jump: Sfx,
    pub hook_fire: Sfx,
    pub stomp: Sfx,
}

impl Default for SfxList {
    fn default() -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            sink,
            _stream,
            _stream_handle: stream_handle,

            bomb: Sfx::from_bytes(include_bytes!("../assets/bomb.ogg")),
            jump: Sfx::from_bytes(include_bytes!("../assets/jump.ogg")),
            hook_fire: Sfx::from_bytes(include_bytes!("../assets/hookFire.ogg")),
            stomp: Sfx::from_bytes(include_bytes!("../assets/stomp.ogg")),
        }
    }
}

impl SfxList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn play(&self, sfx: &Sfx, volume: f32) {
        let source = Decoder::new(Cursor::new(sfx.get_bytes())).unwrap();

        self.sink.set_volume(volume);
        self.sink.append(source);
    }
}
