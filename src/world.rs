use crate::camera::Camera;
use crate::chunk::Chunk;
use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Type;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::uniform::UniformManager;
use rayon::prelude::*;
use std::collections::HashMap;
use wgpu::RenderPass;

pub struct World {
    chunks: HashMap<ChunkCoord3D, Chunk>,
}

const RENDER_DISTANCE: i32 = 5;

impl Renderable for World {
    fn render<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        renderer: &'a Renderer,
        uniform: &'a UniformManager,
    ) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for (_, c) in &self.chunks {
            c.render(pass, &uniform);
        }
    }
}

impl World {
    pub fn new(graphics: &Graphics, camera: &Camera) -> Self {
        let mut chunks = HashMap::new();
        Self { chunks }
    }

    pub fn update(&mut self, graphics: &Graphics, player: &mut Player) {
        if player.new_chunk_pos() {
            for x in 0..RENDER_DISTANCE {
                for z in 0..RENDER_DISTANCE {
                    if !self.chunks.contains_key(&ChunkCoord3D::new(
                        x + player.chunk.x,
                        0,
                        z + player.chunk.z,
                    )) {
                        self.chunks.insert(
                            ChunkCoord3D::new(x + player.chunk.x, 0, z + player.chunk.z),
                            Chunk::new(
                                &graphics,
                                ChunkCoord3D::new(x + player.chunk.x, 0, z + player.chunk.z),
                            ),
                        );
                    }
                    if !self.chunks.contains_key(&ChunkCoord3D::new(
                        -x + player.chunk.x,
                        0,
                        z + player.chunk.z,
                    )) {
                        self.chunks.insert(
                            ChunkCoord3D::new(-x + player.chunk.x, 0, z + player.chunk.z),
                            Chunk::new(
                                &graphics,
                                ChunkCoord3D::new(-x + player.chunk.x, 0, z + player.chunk.z),
                            ),
                        );
                    }
                    if !self.chunks.contains_key(&ChunkCoord3D::new(
                        -x + player.chunk.x,
                        0,
                        -z + player.chunk.z,
                    )) {
                        self.chunks.insert(
                            ChunkCoord3D::new(-x + player.chunk.x, 0, -z + player.chunk.z),
                            Chunk::new(
                                &graphics,
                                ChunkCoord3D::new(-x + player.chunk.x, 0, -z + player.chunk.z),
                            ),
                        );
                    }
                    if !self.chunks.contains_key(&ChunkCoord3D::new(
                        x + player.chunk.x,
                        0,
                        -z + player.chunk.z,
                    )) {
                        self.chunks.insert(
                            ChunkCoord3D::new(x + player.chunk.x, 0, -z + player.chunk.z),
                            Chunk::new(
                                &graphics,
                                ChunkCoord3D::new(x + player.chunk.x, 0, -z + player.chunk.z),
                            ),
                        );
                    }
                }
            }
            self.remove_unseen_chunks(&player);
        }
    }

    fn remove_unseen_chunks(&mut self, player: &Player) {
        self.chunks.retain(|v, _| {
            v.x < 5 + player.chunk.x && v.z < 5 + player.chunk.z && v.x > -5 + player.chunk.x && v.z > -5 + player.chunk.z
        });

    }
}
