use std::fmt::Display;

// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{WorldBox, WORLDGEN_INTERNER};
use string_interner::DefaultSymbol;

#[derive(Copy, Clone, Debug)]
pub struct WorldGenOutline {
    pub position: WorldBox,
    pub name: DefaultSymbol,
    pub variant: u8,
    pub level: u8,
}

impl Display for WorldGenOutline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        WORLDGEN_INTERNER.with(|interner| {
            let interner = interner.borrow();
            if let Some(name) = interner.resolve(self.name) {
                write!(
                    f,
                    "<{} variant={} level={} />",
                    name, self.variant, self.level
                )
            } else {
                write!(
                    f,
                    "<Unknown variant={} level={} />",
                    self.variant, self.level
                )
            }
        })
    }
}

pub type OutlineGenerator = Box<dyn Fn(WorldBox, &WorldGenOutline, &mut Vec<WorldGenOutline>)>;

pub struct OutlineGeneratorEntry {
    primary: OutlineGenerator,
}

impl OutlineGeneratorEntry {
    pub fn new(primary: OutlineGenerator) -> Self {
        Self { primary }
    }

    pub fn run(
        &self,
        position: WorldBox,
        outline: &WorldGenOutline,
        queue: &mut Vec<WorldGenOutline>,
    ) {
        (self.primary)(position, outline, queue);
    }
}
