// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{IVec3, Vec3};
use rgb::RGBA8;
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
    PlayerNoClip(bool),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameEvent {
    CharacterJump(Vec3),
    CharacterStomp(Vec3),
    CharacterShoot(Vec3),
    CharacterDamage(Vec3, i16),
    CharacterDeath(Vec3),
    BlockMine(IVec3, u8),
    BlockPlace(IVec3, u8),
    EntityCollision(Vec3),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SyncEvent {
    DrawFrame(u64),
    GameTick(u64),
    GameQuit(u64),
    ParticleTick(Vec3, f32),
    GameInit,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ParticleEvent {
    Explosion(Vec3, f32),
    BlockBreak(IVec3, [RGBA8; 2]),
    BlockPlace(IVec3, [RGBA8; 2]),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Message {
    InputEvent(InputEvent),
    GameEvent(GameEvent),
    SyncEvent(SyncEvent),
    ParticleEvent(ParticleEvent),
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
impl From<ParticleEvent> for Message {
    fn from(e: ParticleEvent) -> Self {
        Self::ParticleEvent(e)
    }
}
