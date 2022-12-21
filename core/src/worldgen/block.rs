// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{ChunkBlockData, ChunkFluidData, Message, Reactor, WorldGenOutline};
use glam::IVec3;

#[derive(Clone, Default, Debug)]
pub struct BlockGeneratorResult {
    pub block: ChunkBlockData,
    pub fluid: ChunkFluidData,
    pub outlines: Vec<WorldGenOutline>,
}
pub type BlockGenerator =
    Box<dyn Fn(IVec3, &Reactor<Message>, BlockGeneratorResult) -> BlockGeneratorResult>;

pub struct BlockGeneratorEntry {
    primary: BlockGenerator,
}

impl BlockGeneratorEntry {
    pub fn new(primary: BlockGenerator) -> Self {
        Self { primary }
    }

    pub fn run(
        &self,
        pos: IVec3,
        reactor: &Reactor<Message>,
        result: BlockGeneratorResult,
    ) -> BlockGeneratorResult {
        (self.primary)(pos, reactor, result)
    }
}
