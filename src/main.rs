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

use wolkenwelten_client::{render_init, ClientState};
use wolkenwelten_game::GameState;
use wolkenwelten_scripting::Runtime;

mod lib;

pub fn main() {
    let (event_loop, windowed_context) = lib::init_glutin(); // This opens a window, and initialized OpenGL
    render_init(); // This is separate because it has no dependency on glutin, just OpenGL
    let render_state = ClientState::new(); // Now that we have setup an OpenGL context, we cam load all meshes/textures/shaders
    let game_state = GameState::new();
    let runtime = Runtime::new();

    // And after having set up everything we can start up the event loop
    lib::run_event_loop(lib::AppState {
        game_state,
        render_state,
        event_loop,
        runtime,
        windowed_context,
    })
}
