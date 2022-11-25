// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{Mesh, MeshVertex, VoxelMesh};
use anyhow::Result;
use wolkenwelten_game::GameState;

/// This struct is meant to be a simple way to store
/// all static meshes included with WW.
#[derive(Debug)]
pub struct MeshList {
    pub bag: VoxelMesh,
    pub dome: Mesh,
    pub fist: VoxelMesh,
    pub grenade: VoxelMesh,
    pub blocks: Vec<Mesh>,
}

impl MeshList {
    fn gen_block_meshes(display: &glium::Display, game: &GameState) -> Result<Vec<Mesh>> {
        let tile_size = 64.0 / 1600.0;
        Ok(game
            .world()
            .blocks
            .borrow()
            .iter()
            .map(|block| {
                let mut vertices: Vec<MeshVertex> = vec![];
                Mesh::add_block_type(&mut vertices, block, tile_size);
                Mesh::from_vec(display, &vertices).expect("Couldn't create block mesh")
            })
            .collect())
    }

    /// Load all the the models from the build-in raw .obj/.vox bytes.
    pub fn new(display: &glium::Display, game: &GameState) -> Result<Self> {
        Ok(Self {
            blocks: Self::gen_block_meshes(display, game)?,
            bag: VoxelMesh::from_vox_data(
                display,
                include_bytes!("../../../assets/voxel_meshes/bag.vox"),
            )?,
            fist: VoxelMesh::from_vox_data(
                display,
                include_bytes!("../../../assets/voxel_meshes/fist.vox"),
            )?,
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
