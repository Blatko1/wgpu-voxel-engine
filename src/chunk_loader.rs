use crate::chunk::{Chunk, ChunkMesh};
use crate::coordinate::ChunkCoord3D;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::world::{self, World};
use flume::{Receiver, Sender};
use std::sync::Arc;

pub struct ChunkGenerator {
    data_sender: Sender<(ChunkCoord3D, Arc<Chunk>)>,
    data_receiver: Receiver<(ChunkCoord3D, Arc<Chunk>)>,
    mesh_sender: Sender<(ChunkCoord3D, ChunkMesh)>,
    mesh_receiver: Receiver<(ChunkCoord3D, ChunkMesh)>,

    data_in_process: Vec<ChunkCoord3D>,
    mesh_in_process: Vec<ChunkCoord3D>,
}

impl ChunkGenerator {
    pub fn new() -> Self {
        let (data_sender, data_receiver) = flume::unbounded();
        let (mesh_sender, mesh_receiver) = flume::unbounded();
        let data_in_process = Vec::new();
        let mesh_in_process = Vec::new();
        Self {
            data_sender,
            data_receiver,
            mesh_sender,
            mesh_receiver,
            data_in_process,
            mesh_in_process,
        }
    }

    pub fn generate_mesh(
        &mut self,
        graphics: &Graphics,
        world: &mut World,
        pos: ChunkCoord3D,
        pool: &uvth::ThreadPool,
    ) {
        let is_between_chunks = ChunkGenerator::check_adjacent_chunks(pos, &world);
        if let Some(chunks) = is_between_chunks {
            let c = world.chunks.get(&pos).unwrap().clone();
            let sender = self.mesh_sender.clone();
            let device = Arc::clone(&graphics.device);
            world.mesh_load_queue.remove(0);
            self.mesh_in_process.push(pos);
            pool.execute(move || {
                let mesh = c.create_mesh(device.clone(), chunks);
                let data = (pos, mesh);
                sender.send(data).unwrap();
            });
        } else {
            let e = world.mesh_load_queue.remove(0);
            world.mesh_load_queue.push(e);
        }
    }

    pub fn generate_chunk_data(&mut self, pos: ChunkCoord3D, pool: &uvth::ThreadPool) {
        let sender = self.data_sender.clone();
        self.data_in_process.push(pos);
        pool.execute(move || {
            let chunk = Arc::new(Chunk::new(pos));
            let data = (pos, chunk);
            sender.send(data).unwrap();
        });
    }

    fn check_adjacent_chunks(pos: ChunkCoord3D, world: &World) -> Option<Vec<Arc<Chunk>>> {
        let mut adjacent_chunks = Vec::new();
        if !world.chunks.contains_key(&pos) {
            return None;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x - 1, pos.y, pos.z))
        {
            return None;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x + 1, pos.y, pos.z))
        {
            return None;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x, pos.y, pos.z - 1))
        {
            return None;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x, pos.y, pos.z + 1))
        {
            return None;
        }
        adjacent_chunks.push(
            world
                .chunks
                .get(&ChunkCoord3D::new(pos.x - 1, pos.y, pos.z))
                .unwrap()
                .clone(),
        );
        adjacent_chunks.push(
            world
                .chunks
                .get(&ChunkCoord3D::new(pos.x + 1, pos.y, pos.z))
                .unwrap()
                .clone(),
        );
        adjacent_chunks.push(
            world
                .chunks
                .get(&ChunkCoord3D::new(pos.x, pos.y, pos.z - 1))
                .unwrap()
                .clone(),
        );
        adjacent_chunks.push(
            world
                .chunks
                .get(&ChunkCoord3D::new(pos.x, pos.y, pos.z + 1))
                .unwrap()
                .clone(),
        );
        Some(adjacent_chunks)
    }

    pub fn load_chunk_queue(&self, world: &mut World, player: &mut Player) {
        if player.is_in_new_chunk_pos() {
            player.update_chunk_pos();
            world.data_load_queue.clear();
            world.mesh_load_queue.clear();
            let chunk = player.chunk.clone();
            for x in -5..world::RENDER_DISTANCE + 1 {
                for z in -5..world::RENDER_DISTANCE + 1 {
                    self.enqueue_chunk_data(world, x, 0, z, chunk);
                    self.enqueue_chunk_mesh(world, x, 0, z, chunk);
                    if player.is_in_new_chunk_pos() {
                        return;
                    }
                }
                self.enqueue_chunk_data(world, x, 0, world::RENDER_DISTANCE + 1, chunk);
                self.enqueue_chunk_data(world, x, 0, -world::RENDER_DISTANCE - 1, chunk);
            }
            for z in -5..world::RENDER_DISTANCE + 1 {
                self.enqueue_chunk_data(world, world::RENDER_DISTANCE + 1, 0, z, chunk);
                self.enqueue_chunk_data(world, -world::RENDER_DISTANCE - 1, 0, z, chunk);
            }
        }
    }

    fn enqueue_chunk_data(&self, world: &mut World, x: i32, y: i32, z: i32, chunk: ChunkCoord3D) {
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z))
            && !self
                .data_in_process
                .contains(&ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z))
        {
            world
                .data_load_queue
                .push(ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z));
        }
    }

    fn enqueue_chunk_mesh(&self, world: &mut World, x: i32, y: i32, z: i32, chunk: ChunkCoord3D) {
        if !world
            .meshes
            .contains_key(&ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z))
            && !self
                .mesh_in_process
                .contains(&ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z))
        {
            world
                .mesh_load_queue
                .push(ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z));
        }
    }

    pub fn update_world(&mut self, world: &mut World) {
        match self.data_receiver.try_recv() {
            Ok((pos, data)) => {
                println!(
                    "Loaded chunk data at: x: {}, y: {}, z: {}",
                    pos.x, pos.y, pos.z
                );
                world.chunks.insert(pos, data);
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
                self.mesh_in_process.retain(|&p| p != pos);
            }
            Err(_) => {}
        }
    }
}
