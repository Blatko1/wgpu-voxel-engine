use crate::chunk::{ChunkManager, Chunk};

const REGION_LENGTH: usize = 16;
const REGION_WIDTH: usize = 16;
const REGION_HEIGHT: usize = 16;

const REGION_SIZE: usize = REGION_WIDTH * REGION_LENGTH * REGION_HEIGHT;

pub struct Region {
    chunks: Vec<Chunk>,
    active_chunks: Vec<usize>
}

impl Region {
    pub fn new() -> Self {
        let chunks = Vec::new();
        let active_chunks = Vec::new();
        Self {
            chunks,
            active_chunks
        }
    }

    pub fn add_chunk(&mut self, x: u32, y: u32, z: u32, chunk: Chunk) {
        let raw = coords_to_raw(x, y, z);
        self.chunks.insert(raw, chunk);
        self.active_chunks.push(raw);
    }

    pub fn get_chunk(&self, x: u32, y: u32, z: u32) -> &Chunk {
        self.chunks.get(coords_to_raw(x, y, z)).unwrap()
    }

    pub fn remove_active_chunk(&mut self, x: u32, y: u32, z: u32) {
        self.active_chunks.remove(coords_to_raw(x, y, z));
    }
}

fn coords_to_raw(x: u32, y: u32, z: u32) -> usize {
    (x + z * REGION_WIDTH + y * REGION_LENGTH * REGION_HEIGHT) as usize
}