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
}

impl ChunkGenerator {
    pub fn new() -> Self {
        let (data_sender, data_receiver) = flume::unbounded();
        let (mesh_sender, mesh_receiver) = flume::unbounded();
        Self {
            data_sender,
            data_receiver,
            mesh_sender,
            mesh_receiver,
        }
    }

    pub fn generate_mesh(
        &self,
        graphics: &Graphics,
        world: &mut World,
        pos: ChunkCoord3D,
        pool: &uvth::ThreadPool,
    ) {
        let is_between_chunks = ChunkGenerator::check_adjacent_chunks(pos, &world);
        if is_between_chunks {
            let c = world.chunks.get(&pos).unwrap().clone();
            let sender = self.mesh_sender.clone();
            let device = Arc::clone(&graphics.device);
            let mut adjacent_chunks = Vec::new();
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
            pool.execute(move || {
                let mesh = c.create_mesh(device.clone(), adjacent_chunks);
                let data = (pos, mesh);
                sender.send(data).unwrap();
            });
            world.mesh_load_queue.remove(0);
        }
    }

    pub fn generate_chunk_data(&self, pos: ChunkCoord3D, pool: &uvth::ThreadPool) {
        let sender = self.data_sender.clone();
        pool.execute(move || {
            let chunk = Arc::new(Chunk::new(pos));
            let data = (pos, chunk);
            sender.send(data).unwrap();
        });
    }

    fn check_adjacent_chunks(pos: ChunkCoord3D, world: &World) -> bool {
        if !world.chunks.contains_key(&pos) {
            return false;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x - 1, pos.y, pos.z))
        {
            return false;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x + 1, pos.y, pos.z))
        {
            return false;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x, pos.y, pos.z - 1))
        {
            return false;
        }
        if !world
            .chunks
            .contains_key(&ChunkCoord3D::new(pos.x, pos.y, pos.z + 1))
        {
            return false;
        }
        true
    }

    pub fn load_chunk_queue(&self, world: &mut World, player: &mut Player) {
        if player.new_chunk_pos() {
            let chunk = player.chunk.clone();
            'outer: for x in 0..world::RENDER_DISTANCE {
                for z in 0..world::RENDER_DISTANCE {
                    if !world.global_chunks.contains(&ChunkCoord3D::new(
                        x + chunk.x,
                        0,
                        z + chunk.z,
                    )) {
                        world
                            .global_chunks
                            .push(ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z));
                        world
                            .data_load_queue
                            .push(ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z));
                        world
                            .mesh_load_queue
                            .push(ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z));
                    }
                    if !world.global_chunks.contains(&ChunkCoord3D::new(
                        -x + chunk.x,
                        0,
                        z + chunk.z,
                    )) {
                        world
                            .global_chunks
                            .push(ChunkCoord3D::new(-x + chunk.x, 0, z + chunk.z));
                        world
                            .data_load_queue
                            .push(ChunkCoord3D::new(-x + chunk.x, 0, z + chunk.z));
                        world
                            .mesh_load_queue
                            .push(ChunkCoord3D::new(-x + chunk.x, 0, z + chunk.z));
                    }
                    if !world.global_chunks.contains(&ChunkCoord3D::new(
                        -x + chunk.x,
                        0,
                        -z + chunk.z,
                    )) {
                        world
                            .global_chunks
                            .push(ChunkCoord3D::new(-x + chunk.x, 0, -z + chunk.z));
                        world.data_load_queue.push(ChunkCoord3D::new(
                            -x + chunk.x,
                            0,
                            -z + chunk.z,
                        ));
                        world.mesh_load_queue.push(ChunkCoord3D::new(
                            -x + chunk.x,
                            0,
                            -z + chunk.z,
                        ));
                    }
                    if !world.global_chunks.contains(&ChunkCoord3D::new(
                        x + chunk.x,
                        0,
                        -z + chunk.z,
                    )) {
                        world
                            .global_chunks
                            .push(ChunkCoord3D::new(x + chunk.x, 0, -z + chunk.z));
                        world
                            .data_load_queue
                            .push(ChunkCoord3D::new(x + chunk.x, 0, -z + chunk.z));
                        world
                            .mesh_load_queue
                            .push(ChunkCoord3D::new(x + chunk.x, 0, -z + chunk.z));
                    }
                    if player.new_chunk_pos() {
                        world.data_load_queue.clear();
                        break 'outer;
                    }
                }
            }
        }
        //world.remove_unseen_chunks(&player);
    }

    pub fn update_world(&self, world: &mut World, player: &Player) {
        match self.data_receiver.try_recv() {
            Ok(r) => {
                println!(
                    "Loaded chunk data at: x: {}, y: {}, z: {}",
                    r.0.x, r.0.y, r.0.z
                );
                world.chunks.insert(r.0, r.1);
                //world.remove_all_unseen_chunks(&player);
            }
            Err(_) => {}
        }
        match self.mesh_receiver.try_recv() {
            Ok(r) => {
                println!(
                    "Loaded chunk mesh at: x: {}, y: {}, z: {}",
                    r.0.x, r.0.y, r.0.z
                );
                world.meshes.insert(r.0, r.1);
                //world.remove_all_unseen_chunks(&player);
            }
            Err(_) => {}
        }
    }
}
