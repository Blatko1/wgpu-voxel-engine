use crate::camera::Camera;
use crate::chunk::Chunk;
use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Type;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::uniform::UniformManager;
use rayon::prelude::*;
use std::collections::HashMap;
use wgpu::RenderPass;
use crate::player::Player;

pub struct World {
    chunks: HashMap<ChunkCoord3D, Chunk>,
}

const RENDER_DISTANCE: i32 = 5;

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
    pub fn new(graphics: &Graphics, camera: &Camera) -> Self {
        let mut chunks = HashMap::new();
        Self { chunks }
    }

    pub fn update(&self, camera: &Camera) {
        for x in 0..RENDER_DISTANCE {
            for z in 0..RENDER_DISTANCE {
                if !self.chunks.contains_key(&ChunkCoord3D::new(x, 0, z)) {

                }
            }
        }
    }
}
