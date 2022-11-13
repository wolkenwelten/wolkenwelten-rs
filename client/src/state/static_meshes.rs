// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Mesh, VoxelMesh};
use anyhow::Result;

/// This struct is meant to be a simple way to store
/// all static meshes included with WW.
#[derive(Debug)]
pub struct MeshList {
    pub grenade: VoxelMesh,
    pub dome: Mesh,
}

impl MeshList {
    /// Load all the the models from the build-in raw .obj/.vox bytes.
    pub fn new(display: &glium::Display) -> Result<Self> {
        Ok(Self {
            grenade: VoxelMesh::from_vox_data(
                display,
                include_bytes!("../../../assets/voxel_meshes/grenade.vox"),
            )?,
            dome: Mesh::from_obj_string(
                display,
                include_str!("../../../assets/meshes/skydome.obj"),
            )?,
        })
    }
}
