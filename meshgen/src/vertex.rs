// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glium::implement_vertex;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct BlockVertex {
    pos: [u8; 3],
    texture_index: u8, // Right now we don't really use 256 distinct block faces, ~32 should suffice for a long time
    side_and_light: u8, // And another one here as well
}
implement_vertex!(BlockVertex, pos, texture_index, side_and_light);

impl BlockVertex {
    pub fn new(x: u8, y: u8, z: u8, texture_index: u8, side: u8, light: u8) -> Self {
        let side_and_light = side | (light << 4);
        Self {
            pos: [x, y, z],
            texture_index,
            side_and_light,
        }
    }
}
