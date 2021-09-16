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
    pub chunk_load_queue: Vec<ChunkCoord3D>,

    // Redraw queue
    chunk_redraw_queue: Vec<ChunkCoord3D>,

    // Chunks in loading process
    data_in_process: Vec<ChunkCoord3D>,
}

impl ChunkGenerator {
    pub fn new() -> Self {
        let (data_sender, data_receiver) = flume::unbounded();
        let chunk_load_queue = Vec::new();
        let chunk_redraw_queue = Vec::new();
        let data_in_process = Vec::new();
        Self {
            data_sender,
            data_receiver,
            chunk_load_queue,
            chunk_redraw_queue,
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
        self.load_primary_chunks(&graphics, world, player, &pool);
        // Enqueue chunks in frustum.
        self.enqueue_chunks_in_frustum(world, &player, &frustum);
        // Load chunks in queue.
        self.process_chunk_loading_queue(&graphics, world, &pool);
        // Update the world.
        self.update_world(world);
        // Remove unseen chunks.
        world.filter_unseen_chunks(&player);
        // Clear load queue.
        self.chunk_load_queue.clear();
    }

    fn process_chunk_loading_queue(
        &mut self,
        graphics: &Graphics,
        world: &mut World,
        pool: &uvth::ThreadPool,
    ) {
        if self.chunk_load_queue.len() > 0 {
            let adjacent_chunks =
                ChunkGenerator::check_adjacent_chunks(self.chunk_load_queue[0], &world);
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

    fn try_load_chunk_directly(
        &mut self,
        graphics: &Graphics,
        pos: ChunkCoord3D,
        world: &World,
        pool: &ThreadPool,
    ) {
        if !self.is_chunk_loaded(&world, &pos) {
            let adjacent_chunks = ChunkGenerator::check_adjacent_chunks(pos, &world);
            self.generate_chunk(&graphics, adjacent_chunks, pos, &pool);
            self.data_in_process.push(pos);
        }
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

    fn load_primary_chunks(
        &mut self,
        graphics: &Graphics,
        world: &mut World,
        player: &mut Player,
        pool: &ThreadPool,
    ) {
        if player.is_in_new_chunk_pos() {
            player.update_chunk_pos();
            let player_pos = player.chunk.clone();
            let player_chunk = ChunkCoord3D::new(player_pos.x, 0, player_pos.z);
            // Render the first chunk at players position.
            self.try_load_chunk_directly(&graphics, player_chunk, &world, &pool);
            // Load chunks around the player.
            let radius = 1;
            for z in -radius..radius + 1 {
                let mut pos = ChunkCoord3D::new(player_pos.x + radius, 0, player_pos.z + z);
                self.try_load_chunk_directly(&graphics, pos, &world, &pool);
                pos = ChunkCoord3D::new(player_pos.x - radius, 0, player_pos.z + z);
                self.try_load_chunk_directly(&graphics, pos, &world, &pool);
            }
            for x in (-radius + 1)..radius {
                let mut pos = ChunkCoord3D::new(player_pos.x + x, 0, player_pos.z + radius);
                self.try_load_chunk_directly(&graphics, pos, &world, &pool);
                pos = ChunkCoord3D::new(player_pos.x + x, 0, player_pos.z - radius);
                self.try_load_chunk_directly(&graphics, pos, &world, &pool);
            }
        }
    }

    fn enqueue_chunks_in_frustum(&mut self, world: &mut World, player: &Player, frustum: &Frustum) {
        let player_pos = player.chunk.clone();
        if world::RENDER_DISTANCE > 1 {
            for radius in 2..world::RENDER_DISTANCE {
                for z in -radius..radius + 1 {
                    let mut pos = ChunkCoord3D::new(player_pos.x + radius, 0, player_pos.z + z);
                    self.try_enqueue_data(&world, pos, &frustum);
                    pos = ChunkCoord3D::new(player_pos.x - radius, 0, player_pos.z + z);
                    self.try_enqueue_data(&world, pos, &frustum);
                }
                for x in (-radius + 1)..radius {
                    let mut pos = ChunkCoord3D::new(player_pos.x + x, 0, player_pos.z + radius);
                    self.try_enqueue_data(&world, pos, &frustum);
                    pos = ChunkCoord3D::new(player_pos.x + x, 0, player_pos.z - radius);
                    self.try_enqueue_data(&world, pos, &frustum);
                }
            }
        }
    }

    fn try_enqueue_data(&mut self, world: &World, pos: ChunkCoord3D, frustum: &Frustum) {
        if !self.is_chunk_loaded(world, &pos) {
            if frustum.contains(&pos) {
                self.chunk_load_queue.push(pos);
            }
        }
    }

    fn is_chunk_loaded(&self, world: &World, pos: &ChunkCoord3D) -> bool {
        if !world.chunks.contains_key(&pos) {
            if !self.chunk_load_queue.contains(&pos) {
                if !self.data_in_process.contains(&pos) {
                    return false;
                }
            }
        }
        return true;
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
