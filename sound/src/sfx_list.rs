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

            bomb: Sfx::from_bytes(include_bytes!("../assets/bomb.ogg")),
            jump: Sfx::from_bytes(include_bytes!("../assets/jump.ogg")),
            hook_fire: Sfx::from_bytes(include_bytes!("../assets/hookFire.ogg")),
            stomp: Sfx::from_bytes(include_bytes!("../assets/stomp.ogg")),
            pock: Sfx::from_bytes(include_bytes!("../assets/pock.ogg")),
            tock: Sfx::from_bytes(include_bytes!("../assets/tock.ogg")),
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
