[![WolkenWelten CI](https://github.com/wolkenwelten/wolkenwelten/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/wolkenwelten/wolkenwelten/actions/workflows/ci.yml)

# WolkenWelten

![Have a screenshot](https://github.com/wolkenwelten/wolkenwelten-screenshots/raw/main/2022_10_26_ao.jpg)

This is the rust port of the WolkenWelten codebase, everything is still super early so prepare for pretty much nothing
working whatsoever.

Also keep in mind that this is the first Rust I've ever written, so if you spot some ugly bits of code
be sure to open a PR so as to improve the quality of the codebase :)

# How to build/run it
You need a complete rust toolchain installed that supports Rust 2021 (1.56+), for example via `rustup`.
After that executing the `cargo run --release` command should build and run the game.

# Platform support

| μArch  | Operating System | Status                          |
|--------|------------------|---------------------------------|
| x86_64 | Arch Linux       | Regular manual testing          |
| ARM64  | Raspberry Pi OS  | Regular manual testing          |
| x86_64 | MacOS            | Irregular manual testing        |
| x86_64 | Windows 10       | Irregular manual testing        |
| x86_64 | FreeBSD          | Very infrequent manual testing  |
| x86_64 | Chrome OS        | Very infrequent manual testing  |
| WASM   | Browsers         | Not supported                   |

# Changes from the C version (these are 99% certain to happen if not checked already)
- [X] No more WASM/Emscripten build (really liked that, but it was A LOT of work and broke all the time, so in the beginning WW will only support Lin/Mac/Win)
- [X] No SDL2 (it worked quite well, but it simplifies the build process to have as much as possible be written in Rust)
- [X] Bigger world Size (32-bit per axis, instead of 16, allowing for ~4 Billion Blocks per Axis as compared to ~65 thousand before)
- [ ] OpenAL for sound output
- [ ] Single executable for client/server (the client should be a feature that can be disabled though)
- [ ] Meshes are voxel based, just like the world
- [ ] Include V8 as a scripting runtime instead of Nujel

# Graphics ToDo's (current focus)
- [X] Chunk fade
- [X] Remove hidden surfaces from BlockMeshes
- [X] Chunk/BlockMesh GC
- [X] Frustum culling (port from WW)
- [X] Indexed BlockMeshes
- [X] Simple voxel side occlusion-culling (port from WW)
- [X] Proper chunk draw ordering, back to front due to fade-in (port from WW)
- [X] Chunk fade-in after generation
- [X] Sky sphere (port from WW)
- [X] Frame-rate independent physics/gameplay
- [X] Greedy meshing (port from WW)
- [X] Flat Lighting (port from WW)
- [X] Simple Lightmaps (port from WW)
- [X] Smooth Lighting (port from WW)
- [X] Ambient Occlusion (port from WW)
- [ ] Async lightmap calculation / mesh generation
- [ ] Correct Lightmaps
- [ ] Block manipulation (simple removal/placement as well as block selection)
- [ ] Import Models made with Goxel (probably using .vox format https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox.txt)
- [ ] Make entities use voxel meshes

# Scripting ToDo's
- [X] Embed deno_core
- [X] Enable TypeScript
- [ ] Experiment with Tokio
- [ ] Run a proper deno event loop
- [ ] Build a Module Loader
- [ ] Add `world.getBlock(x, y, z)` and `world.setBlock(x, y, z, b)`
- [ ] Enable ES Modules
- [ ] Make worldgen call into deno and receive an entire chunk of data
- [ ] Allow entities to be created from within deno for example `world.createEntity()`
- [ ] Allow entities to be enumerated from within deno and add `entity.getPos(), entity.setPos(x, y, z), entity.hide()` methods
- [ ] Add support for `entity.onCollide`
- [ ] Add support for `game.onKey{Up,Down}` | `game.onMouse{Up,Down}` higher-level abstractions can then be built within JS/TS
- [ ] Hot reloading of modules in dev mode
- [ ] Player event system (onHurt, onJump...), should be super simple to trigger from within JS, and reasonably simple from within Rust

# GUI ToDo's
- [ ] Allow for the querying and creation of text labels from within JS
- [ ] Add image widget type (just show a sprite from the GUI Atlas)
- [ ] Add textarea widget, basically a label with line-wrap
- [ ] Add button widget, with event handler, but no keyboard/gamepad support (yet)
- [ ] Allow reading of (absolute) mouse position from JS

# Gameplay ToDo's
- [X] "Infinite" world
- [X] Simple placeholder worldgen
- [X] Simple player controls (gravity/collision with the world)
- [ ] Play around with Rapier3D
- [ ] Block mining
- [ ] Item drops
- [ ] Inventory (and block pickups)
- [ ] Health(bar)
- [ ] Fall damage
- [ ] Hitting entities
- [ ] Super simple mob (chases player when close enough and does melee attack)

# Sound ToDo's
- [ ] Allow triggering of SFX via JS
- [ ] Play sound when player hurt
- [ ] Play sound when mob hurt
- [ ] Play sound when mob attacks
- [ ] Play sound when player stomps
- [ ] Play sound when mining
- [ ] Play sound when jumping

# Multiplayer ToDo's
- [ ] Super simple multiplayer (connecting with login/logout messages only for now)
- [ ] Synchronize world mutations
- [ ] Synchronize character movement (and render characters)
- [ ] Hitting other players
- [ ] Synchronize entities
- [ ] Allow for a simple JSON-RPC inspired interface for client<->server communication
- [ ] Use lz4_flex to compress network traffic

# Infrastructure ToDo's
- [X] (some) unit tests
- [X] CI running tests/fmt/clippy
- [ ] Build releases in GH CI (Win/Mac/Lin)
- [ ] Allow for automatic releases via GH