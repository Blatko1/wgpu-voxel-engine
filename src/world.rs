use crate::chunk::Chunk;
use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Type;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::uniform::UniformManager;
use std::collections::HashMap;
use wgpu::RenderPass;

pub struct World {
    chunks: HashMap<ChunkCoord3D, Chunk>,
}

impl Renderable for World {
    fn render<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        renderer: &'a Renderer,
        uniform: &'a UniformManager,
    ) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for (_, c) in &self.chunks {
            c.render(pass, &uniform);
        }
    }
}

impl World {
    pub fn new(graphics: &Graphics) -> Self {
        let mut chunks = HashMap::new();
        chunks.insert(Coord3D::new(0, 0, 0), Chunk::new(&graphics, Coord3D::new(0, 0, 0)));
        //chunks.insert(Coord3D::new(1, 0, 0), Chunk::new(&graphics, Coord3D::new(1, 0, 0)));
        Self { chunks }
    }
}
