// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Character, Chungus, Entity};
use glam::{IVec3, Vec3};
use wolkenwelten_common::{BlockItem, GameEvent, Item, Message};

const ITEM_DROP_PICKUP_RANGE: f32 = 1.5;

#[derive(Clone, Default, Debug)]
pub struct ItemDrop {
    item: Item,
    ent: Entity,
}

impl ItemDrop {
    pub fn new(pos: Vec3, item: Item) -> Self {
        let mut ent = Entity::new();
        ent.set_pos(pos);
        Self { item, ent }
    }
    #[inline]
    pub fn pos(&self) -> Vec3 {
        self.ent.pos()
    }
    #[inline]
    pub fn rot(&self) -> Vec3 {
        self.ent.rot()
    }
    #[inline]
    pub fn vel(&self) -> Vec3 {
        self.ent.vel()
    }
    #[inline]
    pub fn item(&self) -> Item {
        self.item
    }

    #[inline]
    pub fn set_vel(&mut self, vel: Vec3) {
        self.ent.set_vel(vel);
    }
    #[inline]
    pub fn set_rot(&mut self, rot: Vec3) {
        self.ent.set_rot(rot);
    }
    #[inline]
    pub fn set_pos(&mut self, pos: Vec3) {
        self.ent.set_pos(pos);
    }

    #[inline]
    pub fn is_colliding(&self, world: &Chungus) -> bool {
        self.ent.is_colliding(world)
    }

    #[inline]
    pub fn tick(&mut self, world: &Chungus) -> bool {
        self.ent.tick(world)
    }
}

#[derive(Clone, Default, Debug)]
pub struct ItemDropList {
    drops: Vec<ItemDrop>,
}

impl ItemDropList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_from_block_break(&mut self, pos: IVec3, block: u8) {
        let pos = pos.as_vec3() + Vec3::new(0.5, 0.5, 0.5);
        let item = BlockItem::new(block, 1).into();
        self.drops.push(ItemDrop::new(pos, item));
    }

    pub fn add(&mut self, pos: Vec3, vel: Vec3, item: Item) {
        let mut drop = ItemDrop::new(pos, item);
        drop.set_vel(vel);
        self.drops.push(drop);
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<ItemDrop> {
        self.drops.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.drops.len()
    }

    pub fn tick_all(&mut self, player: &Character, world: &Chungus) -> Vec<Message> {
        let player_pos = player.pos();
        let mut events = vec![];
        for index in (0..self.drops.len()).rev() {
            self.drops[index].tick(world);
            let dist = self.drops[index].pos() - player_pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;
            if dd > (256.0 * 256.0) {
                self.drops.swap_remove(index); // Remove when far enough away
            } else if dd < ITEM_DROP_PICKUP_RANGE * ITEM_DROP_PICKUP_RANGE {
                events.push(
                    GameEvent::ItemDropPickup(self.drops[index].pos(), self.drops[index].item())
                        .into(),
                );
                self.drops.swap_remove(index);
            }
        }
        events
    }
}
