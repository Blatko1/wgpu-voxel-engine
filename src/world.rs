use crate::chunk::{Chunk, ChunkMesh};
use crate::chunk_loader::ChunkGenerator;
use crate::coordinate::ChunkCoord3D;
use crate::frustum_culling::Frustum;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Type;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::uniform::UniformManager;
use hashbrown::HashMap;
use std::sync::Arc;
use wgpu::RenderPass;

pub struct World {
    pub global_chunks: Vec<ChunkCoord3D>,
    pub chunks: HashMap<ChunkCoord3D, Arc<Chunk>>,
    pub meshes: HashMap<ChunkCoord3D, ChunkMesh>,

    //Chunk Loading Queue
    pub data_load_queue: Vec<ChunkCoord3D>,
    pub mesh_load_queue: Vec<ChunkCoord3D>,
}

pub const RENDER_DISTANCE: i32 = 5;
const MAX_LOADING_QUEUE_DATA: u32 = 2;

pub const CHUNK_USIZE: usize = 32;
pub const CHUNK_I32: i32 = 32;

impl Renderable for World {
    fn render<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        renderer: &'a Renderer,
        uniform: &'a UniformManager,
        frustum: &'a Frustum,
    ) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for (p, c) in self.meshes.iter() {
            if frustum.check(p) {
                c.render(pass, &uniform);
            }
        }
    }
}

impl World {
    pub fn new() -> Self {
        let global_chunks = Vec::new();
        let chunks = HashMap::new();
        let meshes = HashMap::new();
        let data_load_queue = Vec::new();
        let mesh_load_queue = Vec::new();
        Self {
            global_chunks,
            chunks,
            meshes,
            data_load_queue,
            mesh_load_queue,
        }
    }

    pub fn update(
        &mut self,
        chunk_gen: &ChunkGenerator,
        player: &mut Player,
        pool: &uvth::ThreadPool,
        graphics: &Graphics,
    ) {
        // Check if chunks within render distance need loading
        chunk_gen.load_chunk_queue(self, player);
        // Load chunks in queue.
        self.process_loading_queue(&chunk_gen, &pool);
        // Check if any chunks need mesh loading
        self.generate_meshes(&graphics, &chunk_gen, &pool);
        chunk_gen.update_world(self, &player);
    }

    fn process_loading_queue(&mut self, chunk_gen: &ChunkGenerator, pool: &uvth::ThreadPool) {
        for _ in 0..MAX_LOADING_QUEUE_DATA {
            if self.data_load_queue.len() > 0 {
                chunk_gen.generate_chunk_data(self.data_load_queue[0], &pool);
                self.data_load_queue.remove(0);
            }
        }
    }

    fn generate_meshes(
        &mut self,
        graphics: &Graphics,
        chunk_gen: &ChunkGenerator,
        pool: &uvth::ThreadPool,
    ) {
        if self.mesh_load_queue.len() > 0 {
            chunk_gen.generate_mesh(&graphics, self, self.mesh_load_queue[0], &pool);
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
                && p.y < RENDER_DISTANCE + player.chunk.y
                && p.y > -RENDER_DISTANCE + player.chunk.y
            {
                return true;
            }
            meshes.remove(p);
            return false;
        });
    }

    pub fn remove_all_unseen_chunks(&mut self, player: &Player) {
        let meshes = &mut self.meshes;
        let global_chunks = &mut self.global_chunks;
        self.chunks.retain(|p, _| {
            if p.x < RENDER_DISTANCE + player.chunk.x
                && p.z < RENDER_DISTANCE + player.chunk.z
                && p.x > -RENDER_DISTANCE + player.chunk.x
                && p.z > -RENDER_DISTANCE + player.chunk.z
                && p.y < RENDER_DISTANCE + player.chunk.y
                && p.y > -RENDER_DISTANCE + player.chunk.y
            {
                return true;
            }
            meshes.remove(p);
            global_chunks.retain(|c| *c != *p);
            return false;
        });
    }
}
