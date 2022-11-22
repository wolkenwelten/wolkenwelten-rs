// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Item;
use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum InputEvent {
    PlayerShoot,
    PlayerMove(Vec3),
    PlayerFly(Vec3),
    PlayerTurn(Vec3),
    PlayerBlockMine(IVec3),
    PlayerBlockPlace(IVec3),
    PlayerSwitchSelection(i32),
    PlayerSelect(i32),
    PlayerNoClip(bool),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameEvent {
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
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SyncEvent {
    DrawFrame(Vec3, u64, f32),
    GameTick(u64),
    GameQuit,
    GameInit,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Message {
    InputEvent(InputEvent),
    GameEvent(GameEvent),
    SyncEvent(SyncEvent),
}

impl From<InputEvent> for Message {
    fn from(e: InputEvent) -> Self {
        Self::InputEvent(e)
    }
}
impl From<GameEvent> for Message {
    fn from(e: GameEvent) -> Self {
        Self::GameEvent(e)
    }
}
impl From<SyncEvent> for Message {
    fn from(e: SyncEvent) -> Self {
        Self::SyncEvent(e)
    }
}
