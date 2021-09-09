use crate::chunk::{Chunk, ChunkMesh};
use crate::coordinate::ChunkCoord3D;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::world::{self, World};
use flume::{Receiver, Sender};
use std::sync::Arc;
use uvth::ThreadPool;
use crate::frustum_culling::Frustum;

pub struct ChunkGenerator {
    data_sender: Sender<(ChunkCoord3D, Arc<Chunk>)>,
    data_receiver: Receiver<(ChunkCoord3D, Arc<Chunk>)>,
    mesh_sender: Sender<(ChunkCoord3D, ChunkMesh)>,
    mesh_receiver: Receiver<(ChunkCoord3D, ChunkMesh)>,

    //Chunk Loading Queue
    data_load_queue: Vec<ChunkCoord3D>,
    mesh_load_queue: Vec<ChunkCoord3D>,

    // Chunks in loading process
    data_in_process: Vec<ChunkCoord3D>,
}

const MAX_LOADING_QUEUE_DATA: u32 = 2;

impl ChunkGenerator {
    pub fn new() -> Self {
        let (data_sender, data_receiver) = flume::unbounded();
        let (mesh_sender, mesh_receiver) = flume::unbounded();
        let data_load_queue = Vec::new();
        let mesh_load_queue = Vec::new();
        let data_in_process = Vec::new();
        Self {
            data_sender,
            data_receiver,
            mesh_sender,
            mesh_receiver,
            data_load_queue,
            mesh_load_queue,
            data_in_process,
        }
    }

    pub fn build_chunks(
        &mut self,
        graphics: &Graphics,
        player: &mut Player,
        world: &mut World,
        pool: &ThreadPool,
        frustum: &Frustum
    ) {
        // Enqueue chunks within render distance.
        self.load_chunk_queue(world, player);
        // Load chunk data in queue.
        self.process_data_loading_queue(&pool);
        // Load chunk meshes in queue.
        self.process_mesh_loading_queue(&graphics, world, &pool, &frustum);
        // Update the world.
        self.update_world(world);
        // Remove unseen chunks.
        world.filter_unseen_chunks(&player);
    }

    fn process_data_loading_queue(&mut self, pool: &uvth::ThreadPool) {
        for _ in 0..MAX_LOADING_QUEUE_DATA {
            if self.data_load_queue.len() > 0 {
                self.generate_chunk_data(self.data_load_queue[0], &pool);
                self.data_in_process.push(self.data_load_queue[0]);
                self.data_load_queue.remove(0);
            }
        }
    }

    fn process_mesh_loading_queue(
        &mut self,
        graphics: &Graphics,
        world: &mut World,
        pool: &uvth::ThreadPool,
        frustum: &Frustum
    ) {
        if self.mesh_load_queue.len() > 0 {
            for (i, m) in self.mesh_load_queue.iter().enumerate() {
                if frustum.contains(m) {
                    let adjacent_chunks =
                        ChunkGenerator::check_adjacent_chunks(m.clone(), &world);
                    self.generate_chunk_mesh(
                        &graphics,
                        &world,
                        adjacent_chunks,
                        m.clone(),
                        &pool,
                    );
                    self.mesh_load_queue.remove(i);
                    return;
                }
            }
        }
    }

    fn generate_chunk_data(&mut self, pos: ChunkCoord3D, pool: &uvth::ThreadPool) {
        let sender = self.data_sender.clone();
        pool.execute(move || {
            let chunk = Arc::new(Chunk::new(pos));
            let data = (pos, chunk);
            sender.send(data).unwrap();
        });
    }

    fn generate_chunk_mesh(
        &mut self,
        graphics: &Graphics,
        world: &World,
        adjacent_chunks: Vec<Option<Arc<Chunk>>>,
        pos: ChunkCoord3D,
        pool: &uvth::ThreadPool,
    ) {
        if let Some(chunk) = world.chunks.get(&pos) {
            let c = chunk.clone();
            let sender = self.mesh_sender.clone();
            let device = Arc::clone(&graphics.device);
            pool.execute(move || {
                let mesh = c.create_mesh(device.clone(), adjacent_chunks);
                let data = (pos, mesh);
                sender.send(data).unwrap();
            });
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

    fn load_chunk_queue(&mut self, world: &mut World, player: &mut Player) {
        if player.is_in_new_chunk_pos() {
            player.update_chunk_pos();
            self.data_load_queue.clear();
            let chunk = player.chunk.clone();
            for x in -world::RENDER_DISTANCE..world::RENDER_DISTANCE + 1 {
                for z in -world::RENDER_DISTANCE..world::RENDER_DISTANCE + 1 {
                    self.enqueue_chunk_data(world, x, 0, z, chunk);
                    if player.is_in_new_chunk_pos() {
                        return;
                    }
                }
            }
        }
    }

    fn enqueue_chunk_data(&mut self, world: &World, x: i32, y: i32, z: i32, chunk: ChunkCoord3D) {
        let pos = &ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z);
        if !world
            .chunks
            .contains_key(pos)
            && !self
                .data_in_process
                .contains(pos)
        {
                self.data_load_queue
                    .push(pos.clone());

        }
    }

    fn update_world(&mut self, world: &mut World) {
        match self.data_receiver.try_recv() {
            Ok((pos, data)) => {
                println!(
                    "Loaded chunk data at: x: {}, y: {}, z: {}",
                    pos.x, pos.y, pos.z
                );
                world.chunks.insert(pos, data);
                self.mesh_load_queue.push(pos);
                self.data_in_process.retain(|&p| p != pos);
            }
            Err(_) => {}
        }
        match self.mesh_receiver.try_recv() {
            Ok((pos, mesh)) => {
                println!(
                    "Loaded chunk mesh at: x: {}, y: {}, z: {}",
                    pos.x, pos.y, pos.z
                );
                world.meshes.insert(pos, mesh);
            }
            Err(_) => {}
        }
    }
}
