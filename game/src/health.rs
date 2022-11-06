// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Health {
    hp: i16,
    max_hp: i16,
}

impl Default for Health {
    fn default() -> Self {
        Self { hp: 16, max_hp: 16 }
    }
}

impl Health {
    #[inline]
    pub fn new(max_hp: i16) -> Self {
        Self { max_hp, hp: max_hp }
    }

    #[inline]
    pub fn health(&self) -> i16 {
        self.hp
    }

    #[inline]
    pub fn max_health(&self) -> i16 {
        self.max_hp
    }

    #[inline]
    pub fn is_dead(self) -> bool {
        self.hp <= 0
    }

    #[inline]
    pub fn damage(&mut self, amount: i16) {
        self.hp = (self.hp - amount).clamp(0, self.max_hp);
    }

    #[inline]
    pub fn heal(&mut self, amount: i16) {
        self.hp = (self.hp + amount).clamp(0, self.max_hp);
    }
}

impl ops::Add<i16> for Health {
    type Output = Health;

    fn add(self, rhs: i16) -> Health {
        Self {
            hp: (self.hp - rhs).clamp(0, self.max_hp),
            max_hp: self.max_hp,
        }
    }
}

impl ops::Sub<i16> for Health {
    type Output = Health;

    fn sub(self, rhs: i16) -> Health {
        self + (-rhs)
    }
}
