// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
#[derive(Copy, Clone, Debug)]
pub struct Experience {
    xp: u64,
    xp_total: u64,
    next_level: u64,
    level: u8,
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            xp: 0,
            xp_total: 0,
            next_level: 32,
            level: 1,
        }
    }
}

impl Experience {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn xp(&self) -> u64 {
        self.xp
    }
    #[inline]
    pub fn next_level(&self) -> u64 {
        self.next_level
    }
    #[inline]
    pub fn level(&self) -> u8 {
        self.level
    }
    #[inline]
    pub fn gain(&mut self, xp: u64) {
        self.xp += xp;
    }
    pub fn level_up(&mut self) -> bool {
        if self.xp >= self.next_level {
            self.xp_total += self.next_level;
            self.xp -= self.next_level;
            self.next_level *= 3;
            self.level += 1;
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.xp = 0;
        self.xp_total = 0;
        self.next_level = 32;
        self.level = 1;
    }
    #[inline]
    pub fn set_next_level(&mut self, next_level: u64) {
        self.next_level = next_level;
    }

    pub fn percent_till_level_up(&self) -> f32 {
        self.xp as f32 / self.next_level as f32
    }
}
