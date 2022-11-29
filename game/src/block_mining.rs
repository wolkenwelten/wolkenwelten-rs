// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::GameState;
use glam::IVec3;
use std::collections::HashMap;
use wolkenwelten_common::{Message, Reactor};

#[derive(Clone, Copy, Debug, Default)]
pub struct BlockMining {
    pub block: u8,
    pub damage: u16,
    pub block_health: u16,
}

#[derive(Debug, Default)]
pub struct BlockMiningMap {
    map: HashMap<IVec3, BlockMining>,
}

impl BlockMiningMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.map.retain(|_, p| {
            p.damage -= 1;
            p.damage > 0
        });
    }

    pub fn mine(&mut self, pos: IVec3, block: u8, dmg: u16, block_health: u16) -> bool {
        if let Some(m) = self.map.get_mut(&pos) {
            m.damage += dmg;
            if m.damage > m.block_health {
                m.damage = 1;
                return true;
            }
        } else {
            self.map.insert(
                pos,
                BlockMining {
                    damage: 2,
                    block,
                    block_health,
                },
            );
        }
        false
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<IVec3, BlockMining> {
        self.map.iter()
    }

    pub fn add_handler(reactor: &mut Reactor<Message>, game: &GameState) {
        {
            let mining = game.mining_ref();
            let f = move |_reactor: &Reactor<Message>, _msg: Message| {
                mining.borrow_mut().tick();
            };
            reactor.add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
        }
        {
            let player = game.player_ref();
            let world = game.world_ref();
            let mining = game.mining_ref();
            let drops = game.drops_ref();
            let f = move |reactor: &Reactor<Message>, msg: Message| {
                if let Message::GameTick { ticks } = msg {
                    let player = player.borrow();
                    if let Some((pos, block)) = player.mining() {
                        let mut world = world.borrow_mut();
                        let blocks = world.blocks.clone();
                        let bt = blocks.borrow();
                        if let Some(bt) = bt.get(block as usize) {
                            let mut mining = mining.borrow_mut();
                            if mining.mine(pos, block, 2, bt.block_health()) {
                                drops.borrow_mut().add_from_block_break(pos, block);
                                world.set_block(pos, 0);
                                reactor.defer(Message::BlockBreak { pos, block });
                            }
                        }
                        if (ticks & 0x7F) == 0 {
                            reactor.defer(Message::BlockMine { pos, block });
                        }
                    }
                }
            };
            reactor.add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
        }
    }
}
