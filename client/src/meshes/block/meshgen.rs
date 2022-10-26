use super::{BlockMesh, BlockVertex};
use crate::meshes::util::Vbo;
use gl::types::GLvoid;
use wolkenwelten_game::{
    ChunkBlockData, ChunkLightData, GameState, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE,
};

type BlockBuffer = [[[u8; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
type SideBuffer = [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

struct ChunkPosIter {
    i: usize,
}
impl ChunkPosIter {
    pub fn new() -> Self {
        Self { i: 0 }
    }
}

impl Iterator for ChunkPosIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.i >> (CHUNK_BITS * 2);
        if x >= CHUNK_SIZE {
            return None;
        }
        let y = (self.i >> CHUNK_BITS) & CHUNK_MASK as usize;
        let z = self.i & CHUNK_MASK as usize;
        self.i += 1;
        Some((x as usize, y as usize, z as usize))
    }
}

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

impl BlockMesh {
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

    fn update_front(
        vertices: &mut Vec<BlockVertex>,
        game: &GameState,
        (block_data, light_data, side_cache): (&BlockBuffer, &BlockBuffer, &SideBuffer),
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
                    plane.light[y][x] = Self::light_front_back(light_data, x, y, z + 2)
                        | (Self::light_front_back(light_data, x + 1, y, z + 2) << 4)
                        | (Self::light_front_back(light_data, x + 1, y + 1, z + 2) << 8)
                        | (Self::light_front_back(light_data, x, y + 1, z + 2) << 12);
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
                    let b = game.world.get_block_type(plane.block[y][x]);
                    let pos = (x as u8, y as u8, z as u8);
                    let size = (cw, ch, cd);
                    BlockVertex::add_front(vertices, pos, size, b.tex_front(), light);
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_back(
        vertices: &mut Vec<BlockVertex>,
        game: &GameState,
        (block_data, light_data, side_cache): (&BlockBuffer, &BlockBuffer, &SideBuffer),
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
                    plane.light[y][x] = Self::light_front_back(light_data, x, y, z)
                        | (Self::light_front_back(light_data, x, y + 1, z) << 4)
                        | (Self::light_front_back(light_data, x + 1, y + 1, z) << 8)
                        | (Self::light_front_back(light_data, x + 1, y, z) << 12);
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
                    let b = game.world.get_block_type(plane.block[y][x]);
                    let pos = (x as u8, y as u8, z as u8);
                    let size = (cw, ch, cd);
                    BlockVertex::add_back(vertices, pos, size, b.tex_back(), light);
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_top(
        vertices: &mut Vec<BlockVertex>,
        game: &GameState,
        (block_data, light_data, side_cache): (&BlockBuffer, &BlockBuffer, &SideBuffer),
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
                    plane.light[z][x] = Self::light_top_bottom(light_data, x, y + 2, z)
                        | (Self::light_top_bottom(light_data, x, y + 2, z + 1) << 4)
                        | (Self::light_top_bottom(light_data, x + 1, y + 2, z + 1) << 8)
                        | (Self::light_top_bottom(light_data, x + 1, y + 2, z) << 12);
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
                    let b = game.world.get_block_type(plane.block[z][x]);
                    let pos = (x as u8, y as u8, z as u8);
                    let size = (cw, ch, cd);
                    BlockVertex::add_top(vertices, pos, size, b.tex_top(), light);
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_bottom(
        vertices: &mut Vec<BlockVertex>,
        game: &GameState,
        (block_data, light_data, side_cache): (&BlockBuffer, &BlockBuffer, &SideBuffer),
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
                    plane.light[z][x] = Self::light_top_bottom(light_data, x, y, z)
                        | (Self::light_top_bottom(light_data, x + 1, y, z) << 4)
                        | (Self::light_top_bottom(light_data, x + 1, y, z + 1) << 8)
                        | (Self::light_top_bottom(light_data, x, y, z + 1) << 12);
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
                    let b = game.world.get_block_type(plane.block[z][x]);
                    let pos = (x as u8, y as u8, z as u8);
                    let size = (cw, ch, cd);
                    BlockVertex::add_bottom(vertices, pos, size, b.tex_bottom(), light);
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_left(
        vertices: &mut Vec<BlockVertex>,
        game: &GameState,
        (block_data, light_data, side_cache): (&BlockBuffer, &BlockBuffer, &SideBuffer),
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
                    plane.light[y][z] = Self::light_left_right(light_data, x, y, z)
                        | (Self::light_left_right(light_data, x, y, z + 1) << 4)
                        | (Self::light_left_right(light_data, x, y + 1, z + 1) << 8)
                        | (Self::light_left_right(light_data, x, y + 1, z) << 12);
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
                    let b = game.world.get_block_type(plane.block[y][z]);
                    let pos = (x as u8, y as u8, z as u8);
                    let size = (cw, ch, cd);
                    BlockVertex::add_left(vertices, pos, size, b.tex_left(), light);
                }
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_right(
        vertices: &mut Vec<BlockVertex>,
        game: &GameState,
        (block_data, light_data, side_cache): (&BlockBuffer, &BlockBuffer, &SideBuffer),
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
                    plane.light[y][z] = Self::light_left_right(light_data, x + 2, y, z)
                        | (Self::light_left_right(light_data, x + 2, y + 1, z) << 4)
                        | (Self::light_left_right(light_data, x + 2, y + 1, z + 1) << 8)
                        | (Self::light_left_right(light_data, x + 2, y, z + 1) << 12);
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
                    let b = game.world.get_block_type(plane.block[y][z]);
                    let pos = (x as u8, y as u8, z as u8);
                    let size = (cw, ch, cd);
                    BlockVertex::add_right(vertices, pos, size, b.tex_right(), light);
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
            side_cache[x][y][z] = Self::calc_sides((x + 1, y + 1, z + 1), block_data);
        }
    }

    pub fn update(
        &mut self,
        chunk: &ChunkBlockData,
        light: &ChunkLightData,
        game: &GameState,
        now: u64,
    ) {
        self.last_updated_at = now;
        let mut vertices: Vec<BlockVertex> = Vec::with_capacity(8192);

        let mut block_data: BlockBuffer = [[[0; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
        let mut light_data: BlockBuffer = [[[0; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
        let mut side_cache: SideBuffer = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        Self::calc_block_data(&mut block_data, chunk);
        Self::calc_light_data(&mut light_data, light);
        Self::calc_side_cache(&mut side_cache, &block_data);

        let data = (&block_data, &light_data, &side_cache);

        self.side_square_count[0] = Self::update_front(&mut vertices, game, data);
        self.side_square_count[1] = Self::update_back(&mut vertices, game, data);
        self.side_square_count[2] = Self::update_top(&mut vertices, game, data);
        self.side_square_count[3] = Self::update_bottom(&mut vertices, game, data);
        self.side_square_count[4] = Self::update_left(&mut vertices, game, data);
        self.side_square_count[5] = Self::update_right(&mut vertices, game, data);

        self.side_start[0] = 0;
        for i in 1..6 {
            self.side_start[i] = self.side_start[i - 1] + self.side_square_count[i - 1];
        }

        self.vao.bind();
        let vbo_size: usize = vertices.len() * std::mem::size_of::<BlockVertex>();
        Vbo::buffer_data(vertices.as_ptr() as *const GLvoid, vbo_size as u32);
        BlockVertex::vertex_attrib_pointers();
    }
}
