use std::collections::HashSet;

// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;

#[derive(Clone, Debug, Default)]
pub struct ChunkRequestQueue {
    mesh: HashSet<IVec3>,
    block: HashSet<IVec3>,
    simple_light: HashSet<IVec3>,
    _complex_light: HashSet<IVec3>,
    _meta: HashSet<IVec3>,
}

impl ChunkRequestQueue {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn block(&mut self, pos: IVec3) {
        self.block.insert(pos);
    }

    #[inline]
    pub fn get_block(&self) -> &HashSet<IVec3> {
        &self.block
    }

    #[inline]
    pub fn get_block_mut(&mut self) -> &mut HashSet<IVec3> {
        &mut self.block
    }

    #[inline]
    pub fn block_len(&self) -> usize {
        self.block.len()
    }

    #[inline]
    pub fn simple_light(&mut self, pos: IVec3) {
        self.simple_light.insert(pos);
    }

    #[inline]
    pub fn get_simple_light(&self) -> &HashSet<IVec3> {
        &self.simple_light
    }

    #[inline]
    pub fn get_simple_light_mut(&mut self) -> &mut HashSet<IVec3> {
        &mut self.simple_light
    }

    #[inline]
    pub fn simple_light_len(&self) -> usize {
        self.simple_light.len()
    }

    #[inline]
    pub fn mesh(&mut self, pos: IVec3) {
        self.mesh.insert(pos);
    }

    #[inline]
    pub fn get_mesh(&self) -> &HashSet<IVec3> {
        &self.mesh
    }

    #[inline]
    pub fn get_mesh_mut(&mut self) -> &mut HashSet<IVec3> {
        &mut self.mesh
    }

    #[inline]
    pub fn mesh_len(&self) -> usize {
        self.mesh.len()
    }
}
