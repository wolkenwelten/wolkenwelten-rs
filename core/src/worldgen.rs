// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{ChunkBlockData, ChunkFluidData, Message, Reactor};
use glam::IVec3;
use std::collections::HashMap;

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

#[derive(Clone, Default)]
pub struct WorldGenerator {
    chunk: HashMap<String, ChunkGeneratorEntry>,
}

impl WorldGenerator {
    pub fn new() -> Self {
        Self {
            chunk: HashMap::new(),
        }
    }

    pub fn chunk_insert_primary(&mut self, k: String, λ: ChunkGenerator) {
        self.chunk.insert(k, ChunkGeneratorEntry::new(λ));
    }

    pub fn chunk_generate(&self, pos: IVec3, reactor: &Reactor<Message>) -> ChunkGeneratorResult {
        let result = ChunkGeneratorResult::default();
        if let Some(e) = self.chunk.get("ROOT") {
            e.run(pos, reactor, result)
        } else {
            result
        }
    }
}
