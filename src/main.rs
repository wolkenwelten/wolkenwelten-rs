extern crate rostregen_client;
extern crate rostregen_game;

use rostregen_client::{render_init, ClientState};
use rostregen_game::GameState;

mod lib;

pub fn main() {
    let game_state = GameState::new(); // First we initialize the game, this is completely separate from the Client/OGL state so that we can build a headless server

    let (event_loop, windowed_context) = lib::init_glutin(); // This opens a window, and initialized OpenGL
    render_init(); // This is separate because it has no dependency on glutin, just OpenGL
    let render_state = ClientState::new(); // Now that we have setup an OpenGL context, we cam load all meshes/textures/shaders

    // And after having set up everything we can start up the event loop
    lib::run_event_loop(lib::AppState {
        game_state,
        render_state,
        event_loop,
        windowed_context,
    })
}
