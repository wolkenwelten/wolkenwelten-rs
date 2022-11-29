// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::{Character, Chungus, Grenade};
use crate::{BlockMiningMap, ItemDropList};
use anyhow::Result;
use glam::IVec3;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Instant,
};

use wolkenwelten_common::{
    ChunkRequestQueue, Message, Reactor, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE,
};

const MS_PER_TICK: u64 = 4;

pub struct GameState {
    clock: Rc<RefCell<Instant>>,
    pub ticks_elapsed: u64,
    world: Rc<RefCell<Chungus>>,

    grenades: Rc<RefCell<Vec<Grenade>>>,
    drops: Rc<RefCell<ItemDropList>>,
    mining: Rc<RefCell<BlockMiningMap>>,

    player: Rc<RefCell<Character>>,
    running: Rc<RefCell<bool>>,
}

impl GameState {
    pub fn new() -> Result<Self> {
        let player = Rc::new(RefCell::new(Character::new()));
        player.borrow_mut().rebirth();
        Ok(Self {
            clock: Rc::new(RefCell::new(Instant::now())),
            running: Rc::new(RefCell::new(true)),
            player,
            grenades: Rc::new(RefCell::new(Vec::new())),
            ticks_elapsed: 0,
            drops: Rc::new(RefCell::new(ItemDropList::new())),
            mining: Rc::new(RefCell::new(BlockMiningMap::new())),
            world: Rc::new(RefCell::new(Chungus::new()?)),
        })
    }

    #[inline]
    pub fn get_millis(&self) -> u64 {
        self.clock
            .borrow()
            .elapsed()
            .as_millis()
            .try_into()
            .unwrap()
    }

    #[inline]
    pub fn get_entity_count(&self) -> usize {
        self.grenades.borrow().len()
    }

    #[inline]
    pub fn push_entity(&mut self, e: Grenade) {
        self.grenades.borrow_mut().push(e);
    }

    #[inline]
    pub fn player(&self) -> Ref<Character> {
        self.player.borrow()
    }

    #[inline]
    pub fn player_mut(&self) -> RefMut<Character> {
        self.player.borrow_mut()
    }

    #[inline]
    pub fn player_ref(&self) -> Rc<RefCell<Character>> {
        self.player.clone()
    }

    #[inline]
    pub fn drops(&self) -> Ref<ItemDropList> {
        self.drops.borrow()
    }

    #[inline]
    pub fn drops_mut(&self) -> RefMut<ItemDropList> {
        self.drops.borrow_mut()
    }

    #[inline]
    pub fn drops_ref(&self) -> Rc<RefCell<ItemDropList>> {
        self.drops.clone()
    }

    #[inline]
    pub fn mining(&self) -> Ref<BlockMiningMap> {
        self.mining.borrow()
    }

    #[inline]
    pub fn mining_mut(&self) -> RefMut<BlockMiningMap> {
        self.mining.borrow_mut()
    }

    #[inline]
    pub fn mining_ref(&self) -> Rc<RefCell<BlockMiningMap>> {
        self.mining.clone()
    }

    #[inline]
    pub fn ticks(&self) -> u64 {
        self.ticks_elapsed
    }

    #[inline]
    pub fn running(&self) -> bool {
        *self.running.borrow()
    }

    #[inline]
    pub fn world(&self) -> Ref<Chungus> {
        self.world.borrow()
    }

    #[inline]
    pub fn world_mut(&self) -> RefMut<Chungus> {
        self.world.borrow_mut()
    }

    #[inline]
    pub fn world_ref(&self) -> Rc<RefCell<Chungus>> {
        self.world.clone()
    }

    #[inline]
    pub fn clock_ref(&self) -> Rc<RefCell<Instant>> {
        self.clock.clone()
    }

    #[inline]
    pub fn grenades(&self) -> Ref<Vec<Grenade>> {
        self.grenades.borrow()
    }

    #[inline]
    pub fn grenades_ref(&self) -> Rc<RefCell<Vec<Grenade>>> {
        self.grenades.clone()
    }

    pub fn add_handler(&self, reactor: &mut Reactor<Message>) {
        Grenade::add_handler(reactor, self);
        Character::add_handler(reactor, self);
        BlockMiningMap::add_handler(reactor, self);
        Chungus::add_handler(reactor, self);
        ItemDropList::add_handler(reactor, self);

        {
            let running = self.running.clone();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::GameQuit = msg {
                    running.replace(false);
                }
            };
            reactor.add_sink(Message::GameQuit, Box::new(f));
        }
    }

    pub fn tick(&mut self, reactor: &Reactor<Message>, request: &mut ChunkRequestQueue) {
        let to_run = self.get_millis() / MS_PER_TICK - self.ticks_elapsed;
        for _ in 0..to_run {
            self.ticks_elapsed += 1;
            reactor.dispatch(Message::GameTick {
                ticks: self.ticks_elapsed,
            });
        }
        self.prepare_world(request);
    }

    #[inline]
    pub fn has_chunk(&self, pos: IVec3) -> bool {
        self.world().get(&pos).is_some()
    }

    pub fn get_single_block(&self, (x, y, z): (i32, i32, i32)) -> u8 {
        let pos = IVec3::new(
            x / CHUNK_SIZE as i32,
            y / CHUNK_SIZE as i32,
            z / CHUNK_SIZE as i32,
        );
        let world = self.world_mut();
        if let Some(chnk) = world.get(&pos) {
            chnk.data[(x & CHUNK_MASK) as usize][(y & CHUNK_MASK) as usize]
                [(z & CHUNK_MASK) as usize]
        } else {
            0
        }
    }

    pub fn prepare_world(&mut self, request: &mut ChunkRequestQueue) {
        let pos = self.player.borrow().pos();
        let px = (pos.x as i32) >> CHUNK_BITS;
        let py = (pos.y as i32) >> CHUNK_BITS;
        let pz = (pos.z as i32) >> CHUNK_BITS;

        for cx in -1..=1 {
            for cy in -1..=1 {
                for cz in -1..=1 {
                    let pos = IVec3::new(cx + px, cy + py, cz + pz);
                    if self.world().get(&pos).is_none() {
                        request.block(pos);
                    }
                }
            }
        }
    }
}
