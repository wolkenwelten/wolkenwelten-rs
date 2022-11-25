// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Item;
use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Message {
    DrawFrame(Vec3, u64, f32),
    FinishedFrame(Vec3, u64, f32),
    GameTick(u64),
    GameQuit,
    GameInit,

    CharacterPosRotVel(Vec3, Vec3, Vec3),
    CharacterJump(Vec3),
    CharacterStomp(Vec3),
    CharacterShoot(Vec3),
    CharacterDamage(Vec3, i16),
    CharacterDeath(Vec3),
    CharacterStep(Vec3),
    BlockMine(IVec3, u8),
    BlockBreak(IVec3, u8),
    BlockPlace(IVec3, u8),
    EntityCollision(Vec3),
    ItemDropPickup(Vec3, Item),

    PlayerShoot,
    PlayerDropItem,
    PlayerMove(Vec3),
    PlayerFly(Vec3),
    PlayerTurn(Vec3),
    PlayerBlockMine(Option<IVec3>),
    PlayerBlockPlace(IVec3),
    PlayerSwitchSelection(i32),
    PlayerSelect(i32),
    PlayerNoClip(bool),
}

impl Message {
    /// Returns a positions if there is one associated with that message, mainly
    /// used for positioning sound effects.
    pub fn pos(&self) -> Option<Vec3> {
        match self {
            Message::BlockPlace(ipos, _)
            | Message::BlockBreak(ipos, _)
            | Message::BlockMine(ipos, _) => Some(ipos.as_vec3()),
            Message::ItemDropPickup(pos, _)
            | Message::EntityCollision(pos)
            | Message::CharacterStep(pos)
            | Message::CharacterDeath(pos)
            | Message::CharacterDamage(pos, _)
            | Message::CharacterShoot(pos)
            | Message::CharacterStomp(pos)
            | Message::CharacterJump(pos) => Some(*pos),
            _ => None,
        }
    }
}
