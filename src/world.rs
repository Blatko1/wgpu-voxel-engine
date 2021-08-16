use crate::renderer::{Renderable, Renderer};
use wgpu::RenderPass;
use crate::pipeline::Type;
use std::collections::HashMap;
use crate::chunk::Chunk;
use crate::graphics::Graphics;

pub struct World {
    chunks: HashMap<[u32; 3], Chunk>,
    active_chunks: Vec<[u32; 3]>
}

impl Renderable for World {
    fn render<'a>(&self, pass: &mut RenderPass<'a>, renderer: &'a Renderer) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for p in &self.active_chunks {
            self.chunks.get(p).unwrap().render(pass);
        }
    }
}

impl World {
    pub fn new(graphics: &Graphics) -> Self {
        let mut chunks = HashMap::new();
        chunks.insert([0, 0, -2], Chunk::new(&graphics));
        let mut active_chunks = Vec::new();
        active_chunks.push([0, 0, -2]);
        Self {
            chunks,
            active_chunks
        }
    }
}
