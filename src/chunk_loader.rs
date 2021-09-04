use crate::chunk::{Chunk, ChunkMesh};
use crate::coordinate::ChunkCoord3D;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::world::{self, World};
use flume::{Receiver, Sender};
use std::sync::Arc;

pub struct ChunkGenerator {
    sender: Sender<(ChunkCoord3D, Chunk, ChunkMesh)>,
    receiver: Receiver<(ChunkCoord3D, Chunk, ChunkMesh)>,
}

impl ChunkGenerator {
    pub fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self { sender, receiver }
    }

    pub fn generate_chunk(&self, graphics: &Graphics, pos: ChunkCoord3D, pool: &uvth::ThreadPool) {
        let sender = self.sender.clone();
        let device = Arc::clone(&graphics.device);
        pool.execute(move || {
            let c = Chunk::new(pos);
            let mesh = c.create_mesh(device.clone());
            let data = (pos, c, mesh);
            sender.send(data).unwrap();
        });
    }

    pub fn load_chunk_queue(&self, world: &mut World, player: &mut Player) {
        if player.new_chunk_pos() {
            let chunk = player.chunk.clone();
            'outer: for x in 0..world::RENDER_DISTANCE {
                for z in 0..world::RENDER_DISTANCE {
                    for y in 0..world::RENDER_DISTANCE {
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            x + chunk.x,
                            y + chunk.y,
                            z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                x + chunk.x,
                                y + chunk.y,
                                z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                x + chunk.x,
                                y + chunk.y,
                                z + chunk.z,
                            ));
                        }
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            -x + chunk.x,
                            y + chunk.y,
                            z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                y + chunk.y,
                                z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                y + chunk.y,
                                z + chunk.z,
                            ));
                        }
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            -x + chunk.x,
                            y + chunk.y,
                            -z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                y + chunk.y,
                                -z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                y + chunk.y,
                                -z + chunk.z,
                            ));
                        }
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            x + chunk.x,
                            y + chunk.y,
                            -z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                x + chunk.x,
                                y + chunk.y,
                                -z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                x + chunk.x,
                                y + chunk.y,
                                -z + chunk.z,
                            ));
                        }
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            x + chunk.x,
                            -y + chunk.y,
                            z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                x + chunk.x,
                                -y + chunk.y,
                                z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                x + chunk.x,
                                -y + chunk.y,
                                z + chunk.z,
                            ));
                        }
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            -x + chunk.x,
                            -y + chunk.y,
                            z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                -y + chunk.y,
                                z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                -y + chunk.y,
                                z + chunk.z,
                            ));
                        }
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            -x + chunk.x,
                            -y + chunk.y,
                            -z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                -y + chunk.y,
                                -z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                -x + chunk.x,
                                -y + chunk.y,
                                -z + chunk.z,
                            ));
                        }
                        if !world.global_chunks.contains(&ChunkCoord3D::new(
                            x + chunk.x,
                            -y + chunk.y,
                            -z + chunk.z,
                        )) {
                            world.global_chunks.push(ChunkCoord3D::new(
                                x + chunk.x,
                                -y + chunk.y,
                                -z + chunk.z,
                            ));
                            world.load_queue.push(ChunkCoord3D::new(
                                x + chunk.x,
                                -y + chunk.y,
                                -z + chunk.z,
                            ));
                        }
                        if player.new_chunk_pos() {
                            if world.load_queue.len() > 15 {
                                world.load_queue.clear();
                            }
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    pub fn update_world(&self, world: &mut World, player: &Player) {
        match self.receiver.try_recv() {
            Ok(r) => {
                world.chunks.insert(r.0, r.1);
                world.meshes.insert(r.0, r.2);
                world.remove_unseen_chunks(&player);
            }
            Err(_) => {}
        }
    }
}
