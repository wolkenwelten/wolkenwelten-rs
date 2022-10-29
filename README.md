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
- [X] Include V8 as a scripting runtime instead of Nujel
- [ ] Single executable for client/server (the client should be a feature that can be disabled though)
- [ ] Meshes are voxel based, just like the world
