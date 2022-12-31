// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
#![forbid(unsafe_code)]

mod block_type;
mod character;
mod chungus;
mod chunk;
mod entity;
mod experience;
mod game_log;
mod health;
mod item;
mod iter;
mod message;
mod queue;
mod reactor;
mod state;
mod worldgen;

pub use self::character::{Character, CharacterAnimation, RaycastReturn};
pub use self::chungus::{Chungus, BLOCKS, FLUIDS};
pub use self::entity::Entity;
pub use self::game_log::{GameLog, GAME_LOG};
pub use self::health::Health;
pub use self::state::GameState;
pub use block_type::*;
pub use chunk::*;
pub use experience::*;
pub use item::*;
pub use iter::*;
pub use message::*;
pub use queue::*;
pub use reactor::*;
pub use worldgen::*;

pub const CHUNK_BITS: i32 = 5;
pub const CHUNK_SIZE: usize = 1 << CHUNK_BITS;
pub const CHUNK_MASK: i32 = CHUNK_SIZE as i32 - 1;

pub type ChunkData = [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

pub type ChunkBuffer = [[[u8; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub enum Side {
    #[default]
    Front = 0,
    Back,
    Top,
    Bottom,
    Left,
    Right,
}

impl From<Side> for u8 {
    fn from(s: Side) -> Self {
        s as u8
    }
}
impl From<Side> for usize {
    fn from(s: Side) -> Self {
        s as usize
    }
}
