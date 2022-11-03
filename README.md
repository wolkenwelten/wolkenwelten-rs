# WolkenWelten
![Have a screenshot](https://github.com/wolkenwelten/wolkenwelten-screenshots/raw/main/2022_11_03.jpg)

WolkenWelten aspires to be the **open voxel sandbox** you can modify while it's running.

To accomplish this it combines a **custom graphics engine** written in **Rust** with the **V8 JavaScript engine**.
This should allow for a workflow similar to a browsers DevTools, but with voxels.

Everything is still **very experimental**, and it's more of a proof of concept rather than a finished game.

During development focus has been put into supporting low-end devices like the **Raspberry pi 4(00)**, because of that it should run at ~60FPS in 720p, and ~30-40 in 1080p.

If you want to chat, have a question or would like to contribute then the most direct way would be either Matrix or Discord.

[![AGPL-3.0](https://img.shields.io/github/license/wolkenwelten/wolkenwelten?style=flat-square)](https://www.gnu.org/licenses/agpl-3.0.en.html)
[![CI Main](https://img.shields.io/github/workflow/status/wolkenwelten/wolkenwelten/WolkenWelten%20CI/main?label=CI%20Main&style=flat-square)](https://github.com/wolkenwelten/wolkenwelten/actions/workflows/ci.yml)
[![Matrix](https://img.shields.io/matrix/wolkenwelten:matrix.org?label=Matrix&style=flat-square)](https://matrix.to/#/#wolkenwelten:matrix.org)
[![Discord](https://img.shields.io/discord/750878611795607653?label=Discord&style=flat-square)](https://discord.gg/7rhnYH2)
![Commit activity](https://img.shields.io/github/commit-activity/w/wolkenwelten/wolkenwelten?style=flat-square)

# How to build/run it
You need a complete rust toolchain installed that supports Rust 2021 (1.56+), for example via `rustup`.
After that executing the `cargo run --release` command should build and run the game.

# Platform support

| μArch  | Operating System | Status                         |
|--------|------------------|--------------------------------|
| x86_64 | Arch Linux       | Regular manual testing         |
| ARM64  | Raspberry Pi OS  | Regular manual testing         |
| x86_64 | MacOS            | Irregular manual testing       |
| x86_64 | Windows 10       | Irregular manual testing       |
| x86_64 | Chrome OS        | Very infrequent manual testing |
| x86_64 | FreeBSD          | Not working (for now)          |
| WASM   | Browsers         | Not supported                  |
