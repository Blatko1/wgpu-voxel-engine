use crate::renderer::{Renderable, Renderer};
use wgpu::RenderPass;
use crate::pipeline::Type;
use std::collections::HashMap;
use crate::chunk::Chunk;
use crate::graphics::Graphics;

pub struct World {
    chunks: Vec<Chunk>,
    active_chunks: Vec<[u32; 3]>
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
        let chunks = Vec::new();
        let active_chunks = Vec::new();
        Self {
            chunks,
            active_chunks
        }
    }

    pub fn add_test(&mut self, x: u32, y: u32, z: u32, graphics: &Graphics) {
        self.chunks.insert(to_chunk_pos(x, y, z), Chunk::new(&graphics));
        self.active_chunks.push([x, y, z]);
    }

    pub fn get_chunk(&mut self, x: u32, y: u32, z: u32) -> &Chunk {
        &self.chunks.get(to_chunk_pos(x, y, z)).unwrap()
    }
}

fn to_chunk_pos(x: u32, y: u32, z: u32) -> usize {
    (x + 16 * z + 16 * 16 * y) as usize
}
