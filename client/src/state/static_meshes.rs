// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{meshes::mesh::MeshCreationError, Mesh, VoxelMesh, VoxelMeshCreationError};

#[derive(Debug)]
pub struct MeshList {
    pub grenade: VoxelMesh,
    pub dome: Mesh,
}

#[derive(Debug)]
pub enum MeshListCreationError {
    VoxelMeshCreationError(VoxelMeshCreationError),
    MeshCreationError(MeshCreationError),
}

impl From<VoxelMeshCreationError> for MeshListCreationError {
    fn from(err: VoxelMeshCreationError) -> Self {
        Self::VoxelMeshCreationError(err)
    }
}
impl From<MeshCreationError> for MeshListCreationError {
    fn from(err: MeshCreationError) -> Self {
        Self::MeshCreationError(err)
    }
}

impl MeshList {
    pub fn new(display: &glium::Display) -> Result<Self, MeshListCreationError> {
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
