use crate::chunk::{Chunk, ChunkMesh};
use crate::coordinate::ChunkCoord3D;
use crate::frustum_culling::Frustum;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::world::{self, World};
use flume::{Receiver, Sender};
use std::sync::Arc;
use uvth::ThreadPool;

pub struct ChunkGenerator {
    data_sender: Sender<(Arc<Chunk>, ChunkMesh)>,
    data_receiver: Receiver<(Arc<Chunk>, ChunkMesh)>,

    // Chunk Loading Queue
    chunk_load_queue: Vec<ChunkCoord3D>,

    // Redraw queue

    // Chunks in loading process
    data_in_process: Vec<ChunkCoord3D>,
}

const MAX_LOADING_QUEUE_DATA: u32 = 2;

impl ChunkGenerator {
    pub fn new() -> Self {
        let (data_sender, data_receiver) = flume::unbounded();
        let chunk_load_queue = Vec::new();
        let data_in_process = Vec::new();
        Self {
            data_sender,
            data_receiver,
            chunk_load_queue,
            data_in_process,
        }
    }

    pub fn build_chunks(
        &mut self,
        graphics: &Graphics,
        player: &mut Player,
        world: &mut World,
        pool: &ThreadPool,
        frustum: &Frustum,
    ) {
        // Enqueue chunks within render distance.
        self.load_chunk_queue(world, player);
        // Load chunks in queue.
        self.process_chunk_loading_queue(&graphics, world, &pool);
        // Update the world.
        self.update_world(world);
        // Remove unseen chunks.
        world.filter_unseen_chunks(&player);
    }

    fn process_chunk_loading_queue(
        &mut self,
        graphics: &Graphics,
        world: &mut World,
        pool: &uvth::ThreadPool,
    ) {
            if self.chunk_load_queue.len() > 0 {
                let adjacent_chunks = ChunkGenerator::check_adjacent_chunks(self.chunk_load_queue[0], &world);
                self.generate_chunk(&graphics, adjacent_chunks, self.chunk_load_queue[0], &pool);
                self.data_in_process.push(self.chunk_load_queue[0]);
                self.chunk_load_queue.remove(0);
            }

    }

    fn generate_chunk(
        &mut self,
        graphics: &Graphics,
        adjacent_chunks: Vec<Option<Arc<Chunk>>>,
        pos: ChunkCoord3D,
        pool: &uvth::ThreadPool,
    ) {
        let sender = self.data_sender.clone();
        let device = Arc::clone(&graphics.device);
        pool.execute(move || {
            let data = Arc::new(Chunk::new(pos));
            let mesh = data.create_mesh(device.clone(), adjacent_chunks);
            sender.send((data, mesh)).unwrap();
        });
    }

    fn check_adjacent_chunks(pos: ChunkCoord3D, world: &World) -> Vec<Option<Arc<Chunk>>> {
        let mut adjacent_chunks = Vec::new();
        if let Some(c) = world
            .chunks
            .get(&ChunkCoord3D::new(pos.x - 1, pos.y, pos.z))
        {
            adjacent_chunks.push(Some(c.clone()));
        } else {
            adjacent_chunks.push(None);
        }
        if let Some(c) = world
            .chunks
            .get(&ChunkCoord3D::new(pos.x + 1, pos.y, pos.z))
        {
            adjacent_chunks.push(Some(c.clone()));
        } else {
            adjacent_chunks.push(None);
        }
        if let Some(c) = world
            .chunks
            .get(&ChunkCoord3D::new(pos.x, pos.y, pos.z - 1))
        {
            adjacent_chunks.push(Some(c.clone()));
        } else {
            adjacent_chunks.push(None);
        }
        if let Some(c) = world
            .chunks
            .get(&ChunkCoord3D::new(pos.x, pos.y, pos.z + 1))
        {
            adjacent_chunks.push(Some(c.clone()));
        } else {
            adjacent_chunks.push(None);
        }
        adjacent_chunks
    }

    fn load_chunk_queue(&mut self, world: &mut World, player: &mut Player) {
        if player.is_in_new_chunk_pos() {
            player.update_chunk_pos();
            self.chunk_load_queue.clear();
            let player_pos = player.chunk.clone();
            for x in -world::RENDER_DISTANCE..world::RENDER_DISTANCE + 1 {
                for z in -world::RENDER_DISTANCE..world::RENDER_DISTANCE + 1 {
                    self.enqueue_chunk_data(world, x, 0, z, player_pos);
                    if player.is_in_new_chunk_pos() {
                        return;
                    }
                }
            }
        }
    }

    fn enqueue_chunk_data(&mut self, world: &World, x: i32, y: i32, z: i32, chunk: ChunkCoord3D) {
        let pos = &ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z);
        if !world.chunks.contains_key(pos) && !self.data_in_process.contains(pos) {
            self.chunk_load_queue.push(pos.clone());
        }
    }

    fn update_world(&mut self, world: &mut World) {
        match self.data_receiver.try_recv() {
            Ok((data, mesh)) => {
                let pos = data.position.clone();
                println!(
                    "Loaded chunk at: x: {}, y: {}, z: {}",
                    data.position.x, data.position.y, data.position.z
                );
                world.chunks.insert(pos, data);
                world.meshes.insert(pos, mesh);
                self.data_in_process.retain(|&p| p != pos);
            }
            Err(_) => {}
        }
    }
}
