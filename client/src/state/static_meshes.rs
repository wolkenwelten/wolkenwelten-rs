// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{VoxelMesh, Mesh};

pub struct MeshList {
    pub grenade: VoxelMesh,
    pub dome: Mesh,
}

impl MeshList {
    pub fn new(display: &glium::Display) -> Self {
        Self {
            grenade: VoxelMesh::from_vox_data(display, include_bytes!("../../../assets/voxel_meshes/grenade.vox"))
                .unwrap(),
            dome: Mesh::from_obj_string(display, include_str!("../../../assets/meshes/skydome.obj"))
                .unwrap(),
        }
    }
}
