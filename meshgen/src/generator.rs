// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use super::BlockVertex;
use wolkenwelten_common::{
    BlockType, ChunkBlockData, ChunkLightData, ChunkPosIter, Side, CHUNK_SIZE,
};

type BlockBuffer = [[[u8; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
type SideBuffer = [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

#[derive(Copy, Clone, Debug, Default)]
struct PlaneEntry {
    pub width: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
    pub height: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
    pub block: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
    pub light: [[u16; CHUNK_SIZE]; CHUNK_SIZE],
}
impl PlaneEntry {
    pub fn new() -> Self {
        Self {
            block: [[0; CHUNK_SIZE]; CHUNK_SIZE],
            width: [[0; CHUNK_SIZE]; CHUNK_SIZE],
            height: [[0; CHUNK_SIZE]; CHUNK_SIZE],
            light: [[0; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
    pub fn optimize(&mut self) {
        for y in (0..CHUNK_SIZE).rev() {
            for x in (0..CHUNK_SIZE).rev() {
                if self.block[x][y] == 0 {
                    continue;
                }
                if (x < CHUNK_SIZE - 2)
                    && (self.block[x][y] == self.block[x + 1][y])
                    && (self.light[x][y] == self.light[x + 1][y])
                    && (self.width[x][y] == self.width[x + 1][y])
                {
                    self.height[x][y] += self.height[x + 1][y];
                    self.block[x + 1][y] = 0;
                }

                if (y < CHUNK_SIZE - 2)
                    && (self.block[x][y] == self.block[x][y + 1])
                    && (self.light[x][y] == self.light[x][y + 1])
                    && (self.height[x][y] == self.height[x][y + 1])
                {
                    self.width[x][y] += self.width[x][y + 1];
                    self.block[x][y + 1] = 0;
                }
            }
        }
    }
}

fn add_face_front(
    vertices: &mut Vec<BlockVertex>,
    (x, y, z): (u8, u8, u8),
    (w, h, d): (u8, u8, u8),
    texture_index: u8,
    light: u16,
) {
    let side: u8 = Side::Front.into();
    let z = z + d;
    vertices.push(BlockVertex::new(
        x,
        y,
        z,
        texture_index,
        side,
        (light & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y,
        z,
        texture_index,
        side,
        ((light >> 4) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y + h,
        z,
        texture_index,
        side,
        ((light >> 8) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y + h,
        z,
        texture_index,
        side,
        ((light >> 12) & 0xF) as u8,
    ));
}

fn add_face_back(
    vertices: &mut Vec<BlockVertex>,
    (x, y, z): (u8, u8, u8),
    (w, h, _): (u8, u8, u8),
    texture_index: u8,
    light: u16,
) {
    let side: u8 = Side::Back.into();
    vertices.push(BlockVertex::new(
        x,
        y,
        z,
        texture_index,
        side,
        (light & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y + h,
        z,
        texture_index,
        side,
        ((light >> 4) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y + h,
        z,
        texture_index,
        side,
        ((light >> 8) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y,
        z,
        texture_index,
        side,
        ((light >> 12) & 0xF) as u8,
    ));
}

fn add_face_top(
    vertices: &mut Vec<BlockVertex>,
    (x, y, z): (u8, u8, u8),
    (w, h, d): (u8, u8, u8),
    texture_index: u8,
    light: u16,
) {
    let side: u8 = Side::Top.into();
    let y = y + h;
    vertices.push(BlockVertex::new(
        x,
        y,
        z,
        texture_index,
        side,
        (light & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y,
        z + d,
        texture_index,
        side,
        ((light >> 4) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y,
        z + d,
        texture_index,
        side,
        ((light >> 8) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y,
        z,
        texture_index,
        side,
        ((light >> 12) & 0xF) as u8,
    ));
}

fn add_face_bottom(
    vertices: &mut Vec<BlockVertex>,
    (x, y, z): (u8, u8, u8),
    (w, _, d): (u8, u8, u8),
    texture_index: u8,
    light: u16,
) {
    let side: u8 = Side::Bottom.into();
    vertices.push(BlockVertex::new(
        x,
        y,
        z,
        texture_index,
        side,
        (light & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y,
        z,
        texture_index,
        side,
        ((light >> 4) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x + w,
        y,
        z + d,
        texture_index,
        side,
        ((light >> 8) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y,
        z + d,
        texture_index,
        side,
        ((light >> 12) & 0xF) as u8,
    ));
}

fn add_face_left(
    vertices: &mut Vec<BlockVertex>,
    (x, y, z): (u8, u8, u8),
    (_, h, d): (u8, u8, u8),
    texture_index: u8,
    light: u16,
) {
    let side: u8 = Side::Left.into();
    vertices.push(BlockVertex::new(
        x,
        y,
        z,
        texture_index,
        side,
        (light & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y,
        z + d,
        texture_index,
        side,
        ((light >> 4) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y + h,
        z + d,
        texture_index,
        side,
        ((light >> 8) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y + h,
        z,
        texture_index,
        side,
        ((light >> 12) & 0xF) as u8,
    ));
}

fn add_face_right(
    vertices: &mut Vec<BlockVertex>,
    (x, y, z): (u8, u8, u8),
    (w, h, d): (u8, u8, u8),
    texture_index: u8,
    light: u16,
) {
    let side: u8 = Side::Right.into();
    let x = x + w;
    vertices.push(BlockVertex::new(
        x,
        y,
        z,
        texture_index,
        side,
        (light & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y + h,
        z,
        texture_index,
        side,
        ((light >> 4) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y + h,
        z + d,
        texture_index,
        side,
        ((light >> 8) & 0xF) as u8,
    ));
    vertices.push(BlockVertex::new(
        x,
        y,
        z + d,
        texture_index,
        side,
        ((light >> 12) & 0xF) as u8,
    ));
}

fn light_left_right(light_data: &BlockBuffer, x: usize, y: usize, z: usize) -> u16 {
    let a = light_data[x][y][z] as u16;
    let b = light_data[x][y + 1][z] as u16;
    let c = light_data[x][y][z + 1] as u16;
    let d = light_data[x][y + 1][z + 1] as u16;
    ((a + b + c + d) / 4).min(15)
}

fn light_top_bottom(light_data: &BlockBuffer, x: usize, y: usize, z: usize) -> u16 {
    let a = light_data[x][y][z] as u16;
    let b = light_data[x][y][z + 1] as u16;
    let c = light_data[x + 1][y][z] as u16;
    let d = light_data[x + 1][y][z + 1] as u16;
    ((a + b + c + d) / 4).min(15)
}

fn light_front_back(light_data: &BlockBuffer, x: usize, y: usize, z: usize) -> u16 {
    let a = light_data[x][y][z] as u16;
    let b = light_data[x][y + 1][z] as u16;
    let c = light_data[x + 1][y][z] as u16;
    let d = light_data[x + 1][y + 1][z] as u16;
    ((a + b + c + d) / 4).min(15)
}

fn gen_front(
    vertices: &mut Vec<BlockVertex>,
    (block_data, light_data, side_cache, block_types): (
        &BlockBuffer,
        &BlockBuffer,
        &SideBuffer,
        &Vec<BlockType>,
    ),
) -> usize {
    let start = vertices.len();
    // First we slice the chunk into many, zero-initialized, planes
    for z in 0..CHUNK_SIZE {
        let mut found = 0;
        let mut plane: PlaneEntry = PlaneEntry::new();
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                // Skip all faces that can't be seen, due to a block
                // being right in front of that particular face.
                if side_cache[x][y][z] & 1 == 0 {
                    continue;
                }
                // Gotta increment our counter so that we don't skip this chunk
                found += 1;
                plane.width[y][x] = 1;
                plane.height[y][x] = 1;
                plane.block[y][x] = block_data[x + 1][y + 1][z + 1];
                plane.light[y][x] = light_front_back(light_data, x, y, z + 2)
                    | (light_front_back(light_data, x + 1, y, z + 2) << 4)
                    | (light_front_back(light_data, x + 1, y + 1, z + 2) << 8)
                    | (light_front_back(light_data, x, y + 1, z + 2) << 12);
            }
        }
        // If not a single face can be seen then we can skip this slice
        if found == 0 {
            continue;
        }
        plane.optimize();
        let cd = 1;
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if plane.block[y][x] == 0 {
                    continue;
                }
                let light = plane.light[y][x];
                let cw = plane.width[y][x];
                let ch = plane.height[y][x];
                let b = block_types.get(plane.block[y][x] as usize);
                let pos = (x as u8, y as u8, z as u8);
                let size = (cw, ch, cd);
                if let Some(b) = b {
                    add_face_front(vertices, pos, size, b.tex_front(), light);
                }
            }
        }
    }
    (vertices.len() - start) / 4
}

fn gen_back(
    vertices: &mut Vec<BlockVertex>,
    (block_data, light_data, side_cache, block_types): (
        &BlockBuffer,
        &BlockBuffer,
        &SideBuffer,
        &Vec<BlockType>,
    ),
) -> usize {
    let start = vertices.len();
    // First we slice the chunk into many, zero-initialized, planes
    for z in 0..CHUNK_SIZE {
        let mut found = 0;
        let mut plane: PlaneEntry = PlaneEntry::new();
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                // Skip all faces that can't be seen, due to a block
                // being right in front of that particular face.
                if side_cache[x][y][z] & 2 == 0 {
                    continue;
                }
                // Gotta increment our counter so that we don't skip this chunk
                found += 1;
                plane.width[y][x] = 1;
                plane.height[y][x] = 1;
                plane.block[y][x] = block_data[x + 1][y + 1][z + 1];
                plane.light[y][x] = light_front_back(light_data, x, y, z)
                    | (light_front_back(light_data, x, y + 1, z) << 4)
                    | (light_front_back(light_data, x + 1, y + 1, z) << 8)
                    | (light_front_back(light_data, x + 1, y, z) << 12);
            }
        }
        // If not a single face can be seen then we can skip this slice
        if found == 0 {
            continue;
        }
        plane.optimize();
        let cd = 1;
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if plane.block[y][x] == 0 {
                    continue;
                }
                let cw = plane.width[y][x];
                let ch = plane.height[y][x];
                let light = plane.light[y][x];
                let b = block_types.get(plane.block[y][x] as usize);
                let pos = (x as u8, y as u8, z as u8);
                let size = (cw, ch, cd);
                if let Some(b) = b {
                    add_face_back(vertices, pos, size, b.tex_back(), light);
                }
            }
        }
    }
    (vertices.len() - start) / 4
}

fn gen_top(
    vertices: &mut Vec<BlockVertex>,
    (block_data, light_data, side_cache, block_types): (
        &BlockBuffer,
        &BlockBuffer,
        &SideBuffer,
        &Vec<BlockType>,
    ),
) -> usize {
    let start = vertices.len();
    // First we slice the chunk into many, zero-initialized, planes
    for y in 0..CHUNK_SIZE {
        let mut found = 0;
        let mut plane: PlaneEntry = PlaneEntry::new();
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                // Skip all faces that can't be seen, due to a block
                // being right in front of that particular face.
                if side_cache[x][y][z] & 4 == 0 {
                    continue;
                }
                // Gotta increment our counter so that we don't skip this chunk
                found += 1;
                plane.width[z][x] = 1;
                plane.height[z][x] = 1;
                plane.block[z][x] = block_data[x + 1][y + 1][z + 1];
                plane.light[z][x] = light_top_bottom(light_data, x, y + 2, z)
                    | (light_top_bottom(light_data, x, y + 2, z + 1) << 4)
                    | (light_top_bottom(light_data, x + 1, y + 2, z + 1) << 8)
                    | (light_top_bottom(light_data, x + 1, y + 2, z) << 12);
            }
        }
        // If not a single face can be seen then we can skip this slice
        if found == 0 {
            continue;
        }
        plane.optimize();
        let ch = 1;
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if plane.block[z][x] == 0 {
                    continue;
                }
                let cw = plane.width[z][x];
                let cd = plane.height[z][x];
                let light = plane.light[z][x];
                let b = block_types.get(plane.block[z][x] as usize);
                let pos = (x as u8, y as u8, z as u8);
                let size = (cw, ch, cd);
                if let Some(b) = b {
                    add_face_top(vertices, pos, size, b.tex_top(), light);
                }
            }
        }
    }
    (vertices.len() - start) / 4
}

fn gen_bottom(
    vertices: &mut Vec<BlockVertex>,
    (block_data, light_data, side_cache, block_types): (
        &BlockBuffer,
        &BlockBuffer,
        &SideBuffer,
        &Vec<BlockType>,
    ),
) -> usize {
    let start = vertices.len();
    // First we slice the chunk into many, zero-initialized, planes
    for y in 0..CHUNK_SIZE {
        let mut found = 0;
        let mut plane: PlaneEntry = PlaneEntry::new();
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                // Skip all faces that can't be seen, due to a block
                // being right in front of that particular face.
                if side_cache[x][y][z] & 8 == 0 {
                    continue;
                }
                // Gotta increment our counter so that we don't skip this chunk
                found += 1;
                plane.width[z][x] = 1;
                plane.height[z][x] = 1;
                plane.block[z][x] = block_data[x + 1][y + 1][z + 1];
                plane.light[z][x] = light_top_bottom(light_data, x, y, z)
                    | (light_top_bottom(light_data, x + 1, y, z) << 4)
                    | (light_top_bottom(light_data, x + 1, y, z + 1) << 8)
                    | (light_top_bottom(light_data, x, y, z + 1) << 12);
            }
        }
        // If not a single face can be seen then we can skip this slice
        if found == 0 {
            continue;
        }
        plane.optimize();
        let ch = 1;
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if plane.block[z][x] == 0 {
                    continue;
                }
                let cw = plane.width[z][x];
                let cd = plane.height[z][x];
                let light = plane.light[z][x];
                let b = block_types.get(plane.block[z][x] as usize);
                let pos = (x as u8, y as u8, z as u8);
                let size = (cw, ch, cd);
                if let Some(b) = b {
                    add_face_bottom(vertices, pos, size, b.tex_bottom(), light);
                }
            }
        }
    }
    (vertices.len() - start) / 4
}

fn gen_left(
    vertices: &mut Vec<BlockVertex>,
    (block_data, light_data, side_cache, block_types): (
        &BlockBuffer,
        &BlockBuffer,
        &SideBuffer,
        &Vec<BlockType>,
    ),
) -> usize {
    let start = vertices.len();
    // First we slice the chunk into many, zero-initialized, planes
    for x in 0..CHUNK_SIZE {
        let mut found = 0;
        let mut plane: PlaneEntry = PlaneEntry::new();
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // Skip all faces that can't be seen, due to a block
                // being right in front of that particular face.
                if side_cache[x][y][z] & 16 == 0 {
                    continue;
                }
                // Gotta increment our counter so that we don't skip this chunk
                found += 1;
                plane.width[y][z] = 1;
                plane.height[y][z] = 1;
                plane.block[y][z] = block_data[x + 1][y + 1][z + 1];
                plane.light[y][z] = light_left_right(light_data, x, y, z)
                    | (light_left_right(light_data, x, y, z + 1) << 4)
                    | (light_left_right(light_data, x, y + 1, z + 1) << 8)
                    | (light_left_right(light_data, x, y + 1, z) << 12);
            }
        }
        // If not a single face can be seen then we can skip this slice
        if found == 0 {
            continue;
        }
        plane.optimize();
        let cw = 1;
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if plane.block[y][z] == 0 {
                    continue;
                }
                let cd = plane.width[y][z];
                let ch = plane.height[y][z];
                let light = plane.light[y][z];
                let b = block_types.get(plane.block[y][z] as usize);
                let pos = (x as u8, y as u8, z as u8);
                let size = (cw, ch, cd);
                if let Some(b) = b {
                    add_face_left(vertices, pos, size, b.tex_left(), light);
                }
            }
        }
    }
    (vertices.len() - start) / 4
}

fn gen_right(
    vertices: &mut Vec<BlockVertex>,
    (block_data, light_data, side_cache, block_types): (
        &BlockBuffer,
        &BlockBuffer,
        &SideBuffer,
        &Vec<BlockType>,
    ),
) -> usize {
    let start = vertices.len();
    // First we slice the chunk into many, zero-initialized, planes
    for x in 0..CHUNK_SIZE {
        let mut found = 0;
        let mut plane: PlaneEntry = PlaneEntry::new();
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // Skip all faces that can't be seen, due to a block
                // being right in front of that particular face.
                if side_cache[x][y][z] & 32 == 0 {
                    continue;
                }
                // Gotta increment our counter so that we don't skip this chunk
                found += 1;
                plane.width[y][z] = 1;
                plane.height[y][z] = 1;
                plane.block[y][z] = block_data[x + 1][y + 1][z + 1];
                plane.light[y][z] = light_left_right(light_data, x + 2, y, z)
                    | (light_left_right(light_data, x + 2, y + 1, z) << 4)
                    | (light_left_right(light_data, x + 2, y + 1, z + 1) << 8)
                    | (light_left_right(light_data, x + 2, y, z + 1) << 12);
            }
        }
        // If not a single face can be seen then we can skip this slice
        if found == 0 {
            continue;
        }
        plane.optimize();
        let cw = 1;
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if plane.block[y][z] == 0 {
                    continue;
                }
                let cd = plane.width[y][z];
                let ch = plane.height[y][z];
                let light = plane.light[y][z];
                let b = block_types.get(plane.block[y][z] as usize);
                let pos = (x as u8, y as u8, z as u8);
                let size = (cw, ch, cd);
                if let Some(b) = b {
                    add_face_right(vertices, pos, size, b.tex_right(), light);
                }
            }
        }
    }
    (vertices.len() - start) / 4
}

fn calc_block_data(block_data: &mut BlockBuffer, chunk: &ChunkBlockData) {
    for (x, y, z) in ChunkPosIter::new() {
        block_data[x + 1][y + 1][z + 1] = chunk.data[x][y][z];
    }
}

fn calc_light_data(block_data: &mut BlockBuffer, light: &ChunkLightData) {
    for (x, y, z) in ChunkPosIter::new() {
        block_data[x + 1][y + 1][z + 1] = light.data[x][y][z];
    }
}

fn calc_sides((x, y, z): (usize, usize, usize), block_data: &BlockBuffer) -> u8 {
    if block_data[x][y][z] == 0 {
        return 0;
    }
    (if block_data[x][y][z + 1] == 0 { 1 } else { 0 })
        | (if block_data[x][y][z - 1] == 0 { 2 } else { 0 })
        | (if block_data[x][y + 1][z] == 0 { 4 } else { 0 })
        | (if block_data[x][y - 1][z] == 0 { 8 } else { 0 })
        | (if block_data[x - 1][y][z] == 0 { 16 } else { 0 })
        | (if block_data[x + 1][y][z] == 0 { 32 } else { 0 })
}

fn calc_side_cache(side_cache: &mut SideBuffer, block_data: &BlockBuffer) {
    for (x, y, z) in ChunkPosIter::new() {
        side_cache[x][y][z] = calc_sides((x + 1, y + 1, z + 1), block_data);
    }
}

pub fn generate(
    chunk: &ChunkBlockData,
    light: &ChunkLightData,
    block_types: &Vec<BlockType>,
) -> (Vec<BlockVertex>, [usize; 6]) {
    let mut vertices: Vec<BlockVertex> = Vec::with_capacity(8192);

    let mut block_data: BlockBuffer = [[[0; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
    let mut light_data: BlockBuffer = [[[15; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
    let mut side_cache: SideBuffer = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

    calc_block_data(&mut block_data, chunk);
    calc_light_data(&mut light_data, light);
    calc_side_cache(&mut side_cache, &block_data);

    let data = (&block_data, &light_data, &side_cache, block_types);
    let mut side_square_count = [0; 6];

    side_square_count[0] = gen_front(&mut vertices, data);
    side_square_count[1] = gen_back(&mut vertices, data);
    side_square_count[2] = gen_top(&mut vertices, data);
    side_square_count[3] = gen_bottom(&mut vertices, data);
    side_square_count[4] = gen_left(&mut vertices, data);
    side_square_count[5] = gen_right(&mut vertices, data);

    (vertices, side_square_count)
}
