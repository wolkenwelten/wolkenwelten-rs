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
extern crate wolkenwelten_client;
extern crate wolkenwelten_game;
extern crate wolkenwelten_scripting;
extern crate wolkenwelten_sound;

use wolkenwelten_client::ClientState;
use wolkenwelten_game::GameState;
use wolkenwelten_scripting::Runtime;
use wolkenwelten_sound::SfxList;

mod lib;

pub fn main() {
    let (event_loop, display) = lib::init(); // This opens a window, and initialized OpenGL
    let render_state = ClientState::new(display); // Now that we have setup an OpenGL context, we cam load all meshes/textures/shaders

    // And after having set up everything we can start up the event loop
    lib::run_event_loop(lib::AppState {
        game_state: GameState::new(),
        render_state,
        event_loop,
        runtime: Runtime::new(),
        sfx: SfxList::new(),
    })
}
