// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{IVec3, Vec3};

pub enum InputEvent {
    PlayerShoot(),
    PlayerMove(Vec3),
    PlayerFly(Vec3),
    PlayerBlockMine(IVec3),
    PlayerBlockPlace(IVec3),
}

pub enum GameEvent {
    CharacterJump(Vec3),
    CharacterStomp(Vec3),
    CharacterShoot(Vec3),
    BlockMine(IVec3),
    BlockPlace(IVec3),
    EntityCollision(Vec3),
}
