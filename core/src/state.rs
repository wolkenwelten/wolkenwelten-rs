// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Instant,
};

use crate::{
    Character, Chungus, ChunkRequestQueue, Message, Reactor, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE,
};

const MS_PER_TICK: u64 = 4;

pub struct GameState {
    clock: Rc<RefCell<Instant>>,
    pub ticks_elapsed: u64,
    world: Rc<RefCell<Chungus>>,
    player: Rc<RefCell<Character>>,
    running: Rc<RefCell<bool>>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        let player = Rc::new(RefCell::new(Character::new()));
        player.borrow_mut().init();
        Self {
            clock: Rc::new(RefCell::new(Instant::now())),
            running: Rc::new(RefCell::new(true)),
            player,
            ticks_elapsed: 0,
            world: Rc::new(RefCell::new(Chungus::new())),
        }
    }

    pub fn with_handler(self, reactor: &mut Reactor<Message>) -> Self {
        self.add_handler(reactor);
        self
    }

    pub fn get_millis(&self) -> u64 {
        self.clock
            .borrow()
            .elapsed()
            .as_millis()
            .try_into()
            .unwrap()
    }

    pub fn player(&self) -> Ref<Character> {
        self.player.borrow()
    }

    pub fn score(&self) -> u64 {
        let player = self.player();
        let xp = player.experience();
        xp.xp_total() + xp.xp()
    }

    pub fn player_mut(&self) -> RefMut<Character> {
        self.player.borrow_mut()
    }

    pub fn player_rc(&self) -> Rc<RefCell<Character>> {
        self.player.clone()
    }

    pub fn ticks(&self) -> u64 {
        self.ticks_elapsed
    }

    pub fn running(&self) -> bool {
        *self.running.borrow()
    }

    pub fn world(&self) -> Ref<Chungus> {
        self.world.borrow()
    }

    pub fn world_mut(&self) -> RefMut<Chungus> {
        self.world.borrow_mut()
    }

    pub fn world_rc(&self) -> Rc<RefCell<Chungus>> {
        self.world.clone()
    }

    pub fn clock_rc(&self) -> Rc<RefCell<Instant>> {
        self.clock.clone()
    }

    pub fn add_handler(&self, reactor: &mut Reactor<Message>) {
        Character::add_handler(reactor, self);
        Chungus::add_handler(reactor, self);

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
