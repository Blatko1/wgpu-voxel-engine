use crate::chunk::{Chunk, ChunkMesh};
use crate::chunk_loader::ChunkGenerator;
use crate::coordinate::ChunkCoord3D;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Type;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::uniform::UniformManager;
use hashbrown::HashMap;
use wgpu::RenderPass;

pub struct World {
    pub global_chunks: Vec<ChunkCoord3D>,
    pub chunks: HashMap<ChunkCoord3D, Chunk>,
    pub meshes: HashMap<ChunkCoord3D, ChunkMesh>,

    //Chunk Loading Queue
    pub load_queue: Vec<ChunkCoord3D>,
}

pub const RENDER_DISTANCE: i32 = 6;
const MAX_LOADING_QUEUE_DATA: u32 = 1;

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
    pub fn new() -> Self {
        let global_chunks = Vec::new();
        let chunks = HashMap::new();
        let meshes = HashMap::new();
        let load_queue = Vec::new();
        Self {
            global_chunks,
            chunks,
            meshes,
            load_queue,
        }
    }

    pub fn update(
        &mut self,
        chunk_gen: &ChunkGenerator,
        player: &mut Player,
        pool: &uvth::ThreadPool,
        graphics: &Graphics,
    ) {
        chunk_gen.load_chunk_queue(self, player);
        self.process_loading_queue(&chunk_gen, &graphics, &pool);
        chunk_gen.update_world(self, &player);
    }

    fn process_loading_queue(
        &mut self,
        chunk_gen: &ChunkGenerator,
        graphics: &Graphics,
        pool: &uvth::ThreadPool,
    ) {
        if self.load_queue.len() > 0 {
            chunk_gen.generate_chunk(&graphics, self.load_queue[0], &pool);
            self.load_queue.remove(0);
        }
    }

    pub fn remove_unseen_chunks(&mut self, player: &Player) {
        let meshes = &mut self.meshes;
        let global_chunks = &mut self.global_chunks;
        self.chunks.retain(|p, _| {
            if p.x < RENDER_DISTANCE + player.chunk.x
                && p.z < RENDER_DISTANCE + player.chunk.z
                && p.x > -RENDER_DISTANCE + player.chunk.x
                && p.z > -RENDER_DISTANCE + player.chunk.z
            {
                return true;
            }
            meshes.remove(p);
            global_chunks.retain(|c| *c != *p);
            return false;
        });
    }
}
