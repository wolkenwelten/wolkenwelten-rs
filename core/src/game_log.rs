// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use std::{cell::RefCell, collections::VecDeque, time::Instant};

thread_local! {
    pub static GAME_LOG:RefCell<GameLog> = RefCell::new(GameLog::new())
}

#[derive(Clone, Debug)]
pub struct GameLog {
    entries: VecDeque<(String, Instant)>,
    max_len: u8,
}

impl Default for GameLog {
    fn default() -> Self {
        Self {
            entries: VecDeque::new(),
            max_len: 10,
        }
    }
}

impl GameLog {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, s: String) {
        self.entries.push_back((s, Instant::now()));
        while self.entries.len() > self.max_len as usize {
            self.entries.pop_front();
        }
    }

    pub fn set_max_len(&mut self, l: u8) {
        self.max_len = l;
    }

    pub fn max_len(&self) -> u8 {
        self.max_len
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn entries(&self) -> &VecDeque<(String, Instant)> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut VecDeque<(String, Instant)> {
        &mut self.entries
    }
}
