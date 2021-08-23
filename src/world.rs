use crate::renderer::{Renderable, Renderer};
use wgpu::RenderPass;
use crate::pipeline::Type;
use std::collections::HashMap;
use crate::graphics::Graphics;
use crate::coordinate::{Coord3D, RegionCoord3D, ChunkCoord3D};
use crate::chunk::Chunk;
use crate::uniform::UniformManager;

pub struct World {
    chunks: HashMap<ChunkCoord3D, Chunk>,
    active_chunks: Vec<ChunkCoord3D>
}

impl Renderable for World {
    fn render<'a>(&'a self, pass: &mut RenderPass<'a>, renderer: &'a Renderer, uniform: &'a UniformManager) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for p in &self.active_chunks {
            self.chunks.get(p).unwrap().render(pass, &uniform);
        }
    }
}

impl World {
    pub fn new(graphics: &Graphics) -> Self {
        let chunks = HashMap::new();
        let active_chunks = Vec::new();
        Self {
            chunks,
            active_chunks
        }
    }

    pub fn add_chunk(&mut self, coord: Coord3D, graphics: &Graphics) {
        self.chunks.insert(coord.to_chunk_coord(), Chunk::new(&graphics, coord));
    }

    pub fn get_chunk(&self, coord: &Coord3D) -> &Chunk {
        &self.chunks.get(&coord.to_chunk_coord()).unwrap()
    }

    pub fn add_quad(&mut self, graphics: &Graphics) {
        let pos = Coord3D::new(0, 0, 0);
        self.add_chunk(pos, &graphics);
        self.active_chunks.push(pos.to_chunk_coord());
    }
}
