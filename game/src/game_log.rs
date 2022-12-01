// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct GameLog {
    entries: VecDeque<String>,
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
        self.entries.push_back(s);
        while self.entries.len() > self.max_len as usize {
            self.entries.pop_front();
        }
    }

    #[inline]
    pub fn set_max_len(&mut self, l: u8) {
        self.max_len = l;
    }
    #[inline]
    pub fn max_len(&self) -> u8 {
        self.max_len
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub fn entries(&self) -> &VecDeque<String> {
        &self.entries
    }
    #[inline]
    pub fn entries_mut(&mut self) -> &mut VecDeque<String> {
        &mut self.entries
    }
}
