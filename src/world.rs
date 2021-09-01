use crate::camera::Camera;
use crate::chunk::{Chunk, ChunkMesh, ChunkMeshData};
use crate::chunk_generator::ChunkGenerator;
use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Type;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::uniform::UniformManager;
use rayon::prelude::*;
use rayon::ThreadPool;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::RenderPass;

pub struct World {
    pub chunks: HashMap<ChunkCoord3D, Chunk>,
    pub meshes: HashMap<ChunkCoord3D, ChunkMesh>,
}

pub const RENDER_DISTANCE: i32 = 5;

impl Renderable for World {
    fn render<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        renderer: &'a Renderer,
        uniform: &'a UniformManager,
    ) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for (_, c) in self.meshes.iter() {
            c.render(pass, &uniform);
        }
    }
}

impl World {
    pub fn new(graphics: &Graphics) -> Self {
        let mut chunks = HashMap::new();
        let mut meshes = HashMap::new();
        Self { chunks, meshes }
    }

    pub fn update(
        &mut self,
        chunk_gen: &ChunkGenerator,
        player: &mut Player,
        pool: &ThreadPool,
        graphics: &Graphics,
    ) {
        chunk_gen.generate(&self, player, &pool);
        chunk_gen.update_world(self, &graphics);
        self.remove_unseen_chunks(&player);
    }

    fn remove_unseen_chunks(&mut self, player: &Player) {
        self.chunks.retain(|v, _| {
            v.x < RENDER_DISTANCE + player.chunk.x
                && v.z < RENDER_DISTANCE + player.chunk.z
                && v.x > -RENDER_DISTANCE + player.chunk.x
                && v.z > -RENDER_DISTANCE + player.chunk.z
        });
        self.meshes.retain(|v, _| {
            v.x < RENDER_DISTANCE + player.chunk.x
                && v.z < RENDER_DISTANCE + player.chunk.z
                && v.x > -RENDER_DISTANCE + player.chunk.x
                && v.z > -RENDER_DISTANCE + player.chunk.z
        });
    }
}
