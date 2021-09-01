use crate::chunk::{Chunk, ChunkMeshData, ChunkMesh};
use crate::coordinate::ChunkCoord3D;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::world::{self, World};
use std::sync::mpsc::{Receiver, RecvError, Sender};

pub struct ChunkGenerator {
    sender: Sender<(ChunkCoord3D, Chunk, ChunkMeshData)>,
    receiver: Receiver<(ChunkCoord3D, Chunk, ChunkMeshData)>,
}

impl ChunkGenerator {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self { sender, receiver }
    }
    pub fn generate(&self, world: &World, player: &mut Player, pool: &rayon::ThreadPool) {
        if player.new_chunk_pos() {
            let keys = world.chunks.keys().map(|k| *k).collect::<Vec<_>>();
            let chunk = player.chunk.clone();
            let sender = self.sender.clone();
            pool.spawn(move || {
                for x in 0..world::RENDER_DISTANCE {
                    for z in 0..world::RENDER_DISTANCE {
                        if !keys.contains(&ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z)) {
                            let c = Chunk::new(ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z));
                            let mesh_data = c.generate_data();
                            let data = (
                                ChunkCoord3D::new(x + chunk.x, 0, z + chunk.z),
                                c,
                                mesh_data
                            );
                            sender.send(data).unwrap();
                        }
                        if !keys.contains(&ChunkCoord3D::new(-x + chunk.x, 0, z + chunk.z)) {
                            let c = Chunk::new(ChunkCoord3D::new(-x + chunk.x, 0, z + chunk.z));
                            let mesh_data = c.generate_data();
                            let data = (
                                ChunkCoord3D::new(-x + chunk.x, 0, z + chunk.z),
                                c,
                                mesh_data
                            );
                            sender.send(data).unwrap();
                        }
                        if !keys.contains(&ChunkCoord3D::new(-x + chunk.x, 0, -z + chunk.z)) {
                            let c = Chunk::new(ChunkCoord3D::new(-x + chunk.x, 0, -z + chunk.z));
                            let mesh_data = c.generate_data();
                            let data = (
                                ChunkCoord3D::new(-x + chunk.x, 0, -z + chunk.z),
                                c,
                                mesh_data
                            );
                            sender.send(data).unwrap();
                        }
                        if !keys.contains(&ChunkCoord3D::new(x + chunk.x, 0, -z + chunk.z)) {
                            let c = Chunk::new(ChunkCoord3D::new(x + chunk.x, 0, -z + chunk.z));
                            let mesh_data = c.generate_data();
                            let data = (
                                ChunkCoord3D::new(x + chunk.x, 0, -z + chunk.z),
                                c,
                                mesh_data
                            );
                            sender.send(data).unwrap();
                        }
                    }
                }
            });
        }
    }
    pub fn update_world(&self, world: &mut World, graphics: &Graphics) {
        match self.receiver.try_recv() {
            Ok(r) => {
                world.chunks.insert(r.0, r.1);
                world.meshes.insert(r.0, ChunkMesh::new(&graphics, r.2));
            }
            Err(_) => {}
        }
    }
}
