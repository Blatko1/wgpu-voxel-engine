use crate::renderer::{Renderable, Renderer};
use wgpu::RenderPass;
use crate::pipeline::Type;
use std::collections::HashMap;
use crate::chunk::{Chunk, ChunkManager};
use crate::graphics::Graphics;

pub struct World {
    regions: HashMap<(i32, i32, i32), Chunk>,
    active_chunks: Vec<u32>
}

impl Renderable for World {
    fn render<'a>(&'a self, pass: &mut RenderPass<'a>, renderer: &'a Renderer) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for p in &self.active_chunks {
            self.chunks.get(to_chunk_pos(p[0], p[1], p[2])).unwrap().render(pass);
        }
    }
}

impl World {
    pub fn new(graphics: &Graphics) -> Self {
        let chunks = ChunkManager::init();
        let active_chunks = Vec::new();
        Self {
            chunks,
            active_chunks
        }
    }

    pub fn add_test(&mut self, x: i32, y: i32, z: i32, graphics: &Graphics) {
        self.chunks.add_chunk(Chunk::new(&graphics, x, y, z));
    }

    pub fn get_chunk(&mut self, x: u32, y: u32, z: u32) -> &Chunk {
        &self.chunks.get(to_chunk_pos(x, y, z)).unwrap()
    }
}

fn to_chunk_pos(x: u32, y: u32, z: u32) -> usize {
    (x + 16 * z + 16 * 16 * y) as usize
}
