// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Chungus, Entity, GameState};
use glam::{IVec3, Vec3};
use wolkenwelten_common::{BlockItem, Item, Message, Reactor};

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

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn tick_all(&mut self, reactor: &Reactor<Message>, player_pos: Vec3, world: &Chungus) {
        for index in (0..self.drops.len()).rev() {
            self.drops[index].tick(world);
            let dist = self.drops[index].pos() - player_pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;
            if dd > (256.0 * 256.0) {
                self.drops.swap_remove(index); // Remove when far enough away
            } else if dd < ITEM_DROP_PICKUP_RANGE * ITEM_DROP_PICKUP_RANGE {
                reactor.dispatch(Message::ItemDropPickup(
                    self.drops[index].pos(),
                    self.drops[index].item(),
                ));
                self.drops.swap_remove(index);
            }
        }
    }

    pub fn add_handler(reactor: &mut Reactor<Message>, game: &GameState) {
        {
            let player = game.player_ref();
            let drops = game.drops_ref();
            let world = game.world_ref();
            let f = move |reactor: &Reactor<Message>, _msg: Message| {
                let player_pos = player.borrow().pos();
                drops
                    .borrow_mut()
                    .tick_all(reactor, player_pos, &world.borrow());
            };
            reactor.add_sink(Message::GameTick(0), Box::new(f));
        }
    }
}
