// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::Mesh;

pub struct MeshList {
    pub pear: Mesh,
    pub dome: Mesh,
}

impl MeshList {
    pub fn new(display: &glium::Display) -> Self {
        Self {
            dome: Mesh::from_obj_string(display, include_str!("../assets/meshes/skydome.obj"))
                .unwrap(),
            pear: Mesh::from_obj_string(display, include_str!("../assets/meshes/pear.obj"))
                .unwrap(),
        }
    }
}
