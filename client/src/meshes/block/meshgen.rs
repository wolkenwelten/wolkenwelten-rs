use super::{BlockMesh, BlockVertex};
use crate::meshes::util::Vbo;
use gl::types::GLvoid;
use wolkenwelten_game::{ChunkBlockData, GameState, CHUNK_BITS, CHUNK_MASK, CHUNK_SIZE};

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

impl BlockMesh {
    fn update_front(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for (x, y, z) in ChunkPosIter::new() {
            let block = chunk.data[x][y][z];
            if block == 0 {
                continue;
            };
            if (z >= CHUNK_SIZE - 1) || (chunk.data[x][y][z + 1] == 0) {
                let pos = (x as u8, y as u8, z as u8);
                let b = game.world.get_block_type(block);
                BlockVertex::add_front(vertices, pos, size, b.tex_front(), light);
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_back(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for (x, y, z) in ChunkPosIter::new() {
            let block = chunk.data[x][y][z];
            if block == 0 {
                continue;
            };
            if (z == 0) || (chunk.data[x][y][z - 1] == 0) {
                let pos = (x as u8, y as u8, z as u8);
                let b = game.world.get_block_type(block);
                BlockVertex::add_back(vertices, pos, size, b.tex_back(), light);
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_top(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for (x, y, z) in ChunkPosIter::new() {
            let block = chunk.data[x][y][z];
            if block == 0 {
                continue;
            };
            if (y >= CHUNK_SIZE - 1) || (chunk.data[x][y + 1][z] == 0) {
                let pos = (x as u8, y as u8, z as u8);
                let b = game.world.get_block_type(block);
                BlockVertex::add_top(vertices, pos, size, b.tex_top(), light);
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_bottom(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for (x, y, z) in ChunkPosIter::new() {
            let block = chunk.data[x][y][z];
            if block == 0 {
                continue;
            };
            if (y == 0) || (chunk.data[x][y - 1][z] == 0) {
                let pos = (x as u8, y as u8, z as u8);
                let b = game.world.get_block_type(block);
                BlockVertex::add_bottom(vertices, pos, size, b.tex_bottom(), light);
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_left(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for (x, y, z) in ChunkPosIter::new() {
            let block = chunk.data[x][y][z];
            if block == 0 {
                continue;
            };
            if (x == 0) || (chunk.data[x - 1][y][z] == 0) {
                let pos = (x as u8, y as u8, z as u8);
                let b = game.world.get_block_type(block);
                BlockVertex::add_left(vertices, pos, size, b.tex_left(), light);
            }
        }
        (vertices.len() - start) / 4
    }

    fn update_right(
        vertices: &mut Vec<BlockVertex>,
        chunk: &ChunkBlockData,
        game: &GameState,
    ) -> usize {
        let start = vertices.len();
        let size = (1, 1, 1);
        let light = 0x0F;
        for (x, y, z) in ChunkPosIter::new() {
            let block = chunk.data[x][y][z];
            if block == 0 {
                continue;
            };
            if (x >= CHUNK_SIZE - 1) || (chunk.data[x + 1][y][z] == 0) {
                let pos = (x as u8, y as u8, z as u8);
                let b = game.world.get_block_type(block);
                BlockVertex::add_right(vertices, pos, size, b.tex_right(), light);
            }
        }
        (vertices.len() - start) / 4
    }

    pub fn update(&mut self, chunk: &ChunkBlockData, game: &GameState, now: u64) {
        self.last_updated_at = now;
        let mut vertices: Vec<BlockVertex> = Vec::with_capacity(8192);

        self.side_square_count[0] = Self::update_front(&mut vertices, chunk, game);
        self.side_square_count[1] = Self::update_back(&mut vertices, chunk, game);
        self.side_square_count[2] = Self::update_top(&mut vertices, chunk, game);
        self.side_square_count[3] = Self::update_bottom(&mut vertices, chunk, game);
        self.side_square_count[4] = Self::update_left(&mut vertices, chunk, game);
        self.side_square_count[5] = Self::update_right(&mut vertices, chunk, game);
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
