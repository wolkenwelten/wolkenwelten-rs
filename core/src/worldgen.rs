// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{ChunkBlockData, ChunkFluidData, Message, Reactor};
use glam::IVec3;
use std::collections::HashMap;
use string_interner::{DefaultSymbol, StringInterner};

pub struct WorldGenOutline {
    pub position: IVec3,
    pub size: IVec3,
}

#[derive(Clone, Default, Debug)]
pub struct ChunkGeneratorResult {
    pub block: ChunkBlockData,
    pub fluid: ChunkFluidData,
}
pub type ChunkGenerator = fn(
    pos: IVec3,
    reactor: &Reactor<Message>,
    result: ChunkGeneratorResult,
) -> ChunkGeneratorResult;

#[derive(Clone)]
pub struct ChunkGeneratorEntry {
    primary: ChunkGenerator,
}

impl ChunkGeneratorEntry {
    pub fn new(primary: ChunkGenerator) -> Self {
        Self { primary }
    }
}

impl ChunkGeneratorEntry {
    pub fn run(
        &self,
        pos: IVec3,
        reactor: &Reactor<Message>,
        result: ChunkGeneratorResult,
    ) -> ChunkGeneratorResult {
        (self.primary)(pos, reactor, result)
    }
}

#[derive(Clone)]
pub struct WorldGenerator {
    interner: StringInterner,
    chunk: HashMap<DefaultSymbol, ChunkGeneratorEntry>,

    symbol_root: DefaultSymbol,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        let mut interner = StringInterner::new();
        let symbol_root = interner.get_or_intern_static("Root");
        Self {
            interner,
            symbol_root,
            chunk: HashMap::new(),
        }
    }
}

impl WorldGenerator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn chunk_insert_primary(&mut self, k: String, λ: ChunkGenerator) {
        let sym = self.interner.get_or_intern(k);
        self.chunk.insert(sym, ChunkGeneratorEntry::new(λ));
    }

    pub fn chunk_generate(
        &mut self,
        pos: IVec3,
        reactor: &Reactor<Message>,
    ) -> ChunkGeneratorResult {
        let result = ChunkGeneratorResult::default();

        if let Some(e) = self.chunk.get(&self.symbol_root) {
            e.run(pos, reactor, result)
        } else {
            result
        }
    }
}
