// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
extern crate glam;
extern crate wolkenwelten_game;

pub use self::frustum::Frustum;
pub use self::meshes::{BlockMesh, Mesh, MeshVertex, VoxelMesh};
pub use self::queue::QueueEntry;
pub use self::render::{prepare_frame, render_frame, FADE_DISTANCE, RENDER_DISTANCE};
pub use self::render_reactor::{RenderInit, RenderInitArgs, RenderPassArgs, RenderReactor};
pub use self::state::{ClientState, ShaderList};
pub use self::texture::{Texture, TextureArray};
pub use self::winit::start_client;

mod frustum;
mod meshes;
mod queue;
mod render;
mod render_reactor;
mod state;
mod texture;
mod winit;

pub mod input;
pub mod ui;
