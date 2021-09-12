use crate::chunk::{Chunk, ChunkMesh};
use crate::chunk_builder::ChunkGenerator;
use crate::coordinate::ChunkCoord3D;
use crate::frustum_culling::Frustum;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Type;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::uniform::RenderPassData;
use hashbrown::HashMap;
use std::sync::Arc;
use wgpu::RenderPass;

pub struct World {
    pub chunks: HashMap<ChunkCoord3D, Arc<Chunk>>,
    pub meshes: HashMap<ChunkCoord3D, ChunkMesh>,
}

pub const RENDER_DISTANCE: i32 = 4;

pub const CHUNK_USIZE: usize = 32;
pub const CHUNK_I32: i32 = CHUNK_USIZE as i32;

impl Renderable for World {
    fn render<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        renderer: &'a Renderer,
        uniform: &'a RenderPassData,
        frustum: &'a Frustum,
    ) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for (p, c) in self.meshes.iter() {
            if frustum.contains(p) {
                c.render(pass, &uniform);
            }
        }
    }
}

impl World {
    pub fn new() -> Self {
        let chunks = HashMap::new();
        let meshes = HashMap::new();
        Self { chunks, meshes }
    }

    pub fn update(
        &mut self,
        chunk_gen: &mut ChunkGenerator,
        player: &mut Player,
        pool: &uvth::ThreadPool,
        graphics: &Graphics,
        frustum: &Frustum,
    ) {
        chunk_gen.build_chunks(&graphics, player, self, pool, frustum);
    }

    pub fn filter_unseen_chunks(&mut self, player: &Player) {
        let mut meshes = &mut self.meshes;
        self.chunks.retain(|p, _| {
            if p.x <= RENDER_DISTANCE + player.chunk.x
                && p.z <= RENDER_DISTANCE + player.chunk.z
                && p.x >= -RENDER_DISTANCE + player.chunk.x
                && p.z >= -RENDER_DISTANCE + player.chunk.z
                && p.y <= RENDER_DISTANCE + player.chunk.y
                && p.y >= -RENDER_DISTANCE + player.chunk.y {
                return true;
            }
            meshes.remove(p);
            return false;
        });
    }
}
