// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Message, Reactor, CHUNK_SIZE};
use glam::IVec3;
use std::{cell::RefCell, collections::HashMap};
use string_interner::{DefaultSymbol, StringInterner};

mod block;
mod outline;
mod worldbox;
pub use block::*;
pub use outline::*;
pub use worldbox::*;

thread_local! {
    pub static WORLDGEN_INTERNER: RefCell<StringInterner> = RefCell::new(StringInterner::new());
}

pub fn worldgen_intern(s: String) -> DefaultSymbol {
    WORLDGEN_INTERNER.with(|interner| interner.borrow_mut().get_or_intern(s))
}

pub struct WorldGenerator {
    outline: HashMap<DefaultSymbol, OutlineGeneratorEntry>,
    block: HashMap<DefaultSymbol, BlockGeneratorEntry>,

    symbol_root: DefaultSymbol,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        let symbol_root = worldgen_intern("Root".to_string());
        Self {
            symbol_root,
            block: HashMap::new(),
            outline: HashMap::new(),
        }
    }
}

impl WorldGenerator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn outline_insert_primary(&mut self, k: DefaultSymbol, λ: OutlineGenerator) {
        self.outline.insert(k, OutlineGeneratorEntry::new(λ));
    }

    pub fn block_insert_primary(&mut self, k: DefaultSymbol, λ: BlockGenerator) {
        self.block.insert(k, BlockGeneratorEntry::new(λ));
    }

    fn outline_resolve(
        &mut self,
        position: WorldBox,
        outline: &WorldGenOutline,
        queue: &mut Vec<WorldGenOutline>,
    ) {
        if let Some(h) = self.outline.get(&outline.name) {
            h.run(position, outline, queue)
        }
    }

    pub fn outline_generate(&mut self, position: WorldBox) -> Vec<WorldGenOutline> {
        let mut ret = vec![];
        let mut tmp = vec![];
        let mut queue = vec![WorldGenOutline {
            position,
            name: self.symbol_root,
            variant: 0,
            level: 0,
        }];
        loop {
            for outline in queue.iter() {
                self.outline_resolve(position, outline, &mut tmp);
                if outline.position.intersects(&position) {
                    ret.push(*outline);
                }
            }
            {
                if tmp.is_empty() {
                    return ret;
                }
                std::mem::swap(&mut queue, &mut tmp);
                tmp.clear()
            }
        }
    }

    pub fn generate(&mut self, pos: IVec3, reactor: &Reactor<Message>) -> BlockGeneratorResult {
        let pos = pos * CHUNK_SIZE as i32;
        let position = WorldBox::new(
            pos,
            pos + IVec3::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32, CHUNK_SIZE as i32),
        );
        let outlines = self.outline_generate(position);
        let mut ret = outlines
            .iter()
            .fold(BlockGeneratorResult::default(), |result, outline| {
                if let Some(gen) = self.block.get(&outline.name) {
                    gen.run(pos, reactor, result)
                } else {
                    result
                }
            });
        ret.outlines = outlines;
        ret
    }
}
