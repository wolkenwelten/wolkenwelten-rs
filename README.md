# WolkenWelten
![Have a screenshot](https://github.com/wolkenwelten/wolkenwelten-screenshots/raw/main/2022_11_12.jpg)

WolkenWelten aspires to be the **open voxel sandbox** you can modify while it's running.

To accomplish this it combines a **custom graphics engine** written in **Rust** with the **V8 JavaScript engine**.
This should allow for a workflow similar to a browsers DevTools, but with voxels.

Everything is still **very experimental**, and it's more of a proof of concept rather than a finished game.

During development focus has been put into supporting low-end devices like the **Raspberry pi 4(00)**, because of that it should run at ~60FPS in 720p, and ~30-40 in 1080p.

If you want to chat, have a question or would like to contribute then the most direct way would be either Matrix or Discord.

[![AGPL-3.0](https://img.shields.io/github/license/wolkenwelten/wolkenwelten?style=flat-square)](https://www.gnu.org/licenses/agpl-3.0.en.html)
[![CI Main](https://img.shields.io/github/workflow/status/wolkenwelten/wolkenwelten/WolkenWelten%20CI/main?label=CI%20Main&style=flat-square)](https://github.com/wolkenwelten/wolkenwelten/actions/workflows/ci.yml)
[![CI Develop](https://img.shields.io/github/workflow/status/wolkenwelten/wolkenwelten/WolkenWelten%20CI/develop?label=CI%20Develop&style=flat-square)](https://github.com/wolkenwelten/wolkenwelten/actions/workflows/ci.yml)
[![Matrix](https://img.shields.io/matrix/wolkenwelten:matrix.org?label=Matrix&style=flat-square)](https://matrix.to/#/#wolkenwelten:matrix.org)
[![Discord](https://img.shields.io/discord/750878611795607653?label=Discord&style=flat-square)](https://discord.gg/7rhnYH2)
![Commit activity](https://img.shields.io/github/commit-activity/w/wolkenwelten/wolkenwelten?style=flat-square)

# How to build it
You need a complete rust toolchain installed that supports Rust 2021 (1.56+), for example via `rustup`.
After that executing the `cargo run --release` command should build and run the game.

## Linux
In addition to a Rust toolchain, you need to install a couple of system header files using your distributions package manager:

### Arch Linux
```sh
pacman -S base-devel alsa-lib fontconfig freetype2
```

### Debian / Ubuntu / Raspberry Pi OS
```sh
apt-get install build-essential libasound2-dev libfontconfig1-dev libfreetype6-dev
```

# Platform support

| μArch  | Operating System | Status                         |
|--------|------------------|--------------------------------|
| x86_64 | Arch Linux       | Regular manual testing         |
| ARM64  | Raspberry Pi OS  | Regular manual testing         |
| x86_64 | MacOS            | Regular manual testing         |
| x86_64 | Windows 10       | Regular manual testing         |
| x86_64 | Chrome OS        | Irregular manual testing       |
| ARM64  | Windows 11       | Not working (yet)              |
| x86_64 | FreeBSD          | Not working (yet)              |
| x86_64 | NetBSD           | Not working (yet)              |
| x86_64 | OpenBSD          | Not working (yet)              |
| WASM   | Browsers         | Not supported                  |
