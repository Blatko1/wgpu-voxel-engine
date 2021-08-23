/*use crate::chunk::Chunk;
use crate::coordinate::{UCoord3D, Coord3D, RegionCoord3D, ChunkCoord3D};

pub struct Region {
    pos: RegionCoord3D,
    chunks: Vec<Chunk>,
    active_chunks: Vec<ChunkCoord3D>
}

impl Region {
    pub fn new(pos: Coord3D) -> Self {
        let chunks = Vec::new();
        let active_chunks = Vec::new();
        Self {
            pos,
            chunks,
            active_chunks
        }
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        for a in &self.active_chunks {
            self.chunks.get(a.to_index()).unwrap().render(pass);
        }
    }

    pub fn add_chunk(&mut self, coord: Coord3D, chunk: Chunk) {
        let index = coord.to_chunk_coord();
        self.chunks.insert(index.to_index(), chunk);
        self.active_chunks.push(index);
    }

    pub fn get_chunk(&self, coord: UCoord3D) -> &Chunk {
        self.chunks.get(coord.to_chunk_index()).unwrap()
    }

    pub fn remove_active_chunk(&mut self, coord: UCoord3D) {
        self.active_chunks.remove(coord.to_chunk_index());
    }
}*/
