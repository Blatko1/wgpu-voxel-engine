use crate::coordinate::{ChunkCoord3D, Coord3DI};
use crate::cube::{Cube, CubeType};
use crate::perlin_noise;
use crate::quad::{Quad, Rotation};
use crate::texture;
use crate::uniform::{RenderPassData, SetUniforms};
use crate::world::CHUNK_USIZE;
use std::sync::Arc;
use wgpu::util::DeviceExt;

const CHUNK_LENGTH: usize = CHUNK_USIZE;
const CHUNK_WIDTH: usize = CHUNK_USIZE;
const CHUNK_HEIGHT: usize = CHUNK_USIZE;

#[derive(Debug)]
pub struct Chunk {
    pub position: ChunkCoord3D,
    cubes: Vec<Cube>,
}

impl Chunk {
    pub fn new(position: ChunkCoord3D) -> Self {
        let mut cubes = Vec::with_capacity(CHUNK_LENGTH * CHUNK_WIDTH * CHUNK_HEIGHT);
        Chunk::generate_terrain(&mut cubes, position);
        Self { position, cubes }
    }

    fn generate_terrain(cubes: &mut Vec<Cube>, pos: ChunkCoord3D) {
        let noise: Vec<f32>;
        if !std::is_x86_feature_detected!("avx2") {
            noise = perlin_noise::perlin_3d_block_sse41(pos);
        } else {
            noise = perlin_noise::perlin_3d_block_avx(pos);
        }

        for y in 0..CHUNK_USIZE {
            for z in 0..CHUNK_USIZE {
                for x in 0..CHUNK_USIZE {
                    if noise[x + CHUNK_USIZE * z + CHUNK_USIZE * CHUNK_USIZE * y] < 0. {
                        cubes.push(Cube::new(CubeType::GRASS));
                    } else {
                        cubes.push(Cube::new(CubeType::AIR));
                    }
                }
            }
        }
    }

    pub fn create_mesh(
        &self,
        device: Arc<wgpu::Device>,
        adjacent_chunks: Vec<Option<Arc<Chunk>>>,
    ) -> ChunkMesh {
        let world_pos = self.position.to_world_position_i32();
        let mut faces = self.cull_unseen_triangles(world_pos, adjacent_chunks);

        ChunkMesh::new(&device, faces)
    }

    fn cull_unseen_triangles(
        &self,
        world_pos: Coord3DI,
        adjacent_chunks: Vec<Option<Arc<Chunk>>>,
    ) -> Vec<Quad> {
        let mut faces = Vec::new();
        for y in 0..CHUNK_HEIGHT {
            let pos_y = y as i32 + world_pos.y;
            for z in 0..CHUNK_WIDTH {
                let pos_z = z as i32 + world_pos.z;
                for x in 0..CHUNK_LENGTH {
                    let cube_ref = &self.cubes[x + CHUNK_USIZE * z + CHUNK_USIZE * CHUNK_USIZE * y];
                    if cube_ref.cube_type == CubeType::AIR {
                        continue;
                    }
                    let texture_index =
                        unsafe { texture::TEXTURE_INDEX_LIST[cube_ref.cube_type as usize] };
                    let pos_x = x as i32 + world_pos.x;
                    if x > 0 {
                        if self.cubes[(x - 1) + CHUNK_USIZE * z + CHUNK_USIZE * CHUNK_USIZE * y]
                            .cube_type
                            == CubeType::AIR
                        {
                            faces.push(Quad::new(
                                Coord3DI::new(pos_x, pos_y, pos_z),
                                Rotation::LEFT,
                                texture_index[0],
                            ));
                        }
                    } else {
                        if let Some(c) = &adjacent_chunks[0] {
                            if c.cubes[(CHUNK_USIZE - 1)
                                + CHUNK_USIZE * z
                                + CHUNK_USIZE * CHUNK_USIZE * y]
                                .cube_type
                                == CubeType::AIR
                            {
                                faces.push(Quad::new(
                                    Coord3DI::new(pos_x, pos_y, pos_z),
                                    Rotation::LEFT,
                                    texture_index[0],
                                ));
                            }
                        }
                    }
                    if x < CHUNK_USIZE - 1 {
                        if self.cubes[(x + 1) + CHUNK_USIZE * z + CHUNK_USIZE * CHUNK_USIZE * y]
                            .cube_type
                            == CubeType::AIR
                        {
                            faces.push(Quad::new(
                                Coord3DI::new(pos_x, pos_y, pos_z),
                                Rotation::RIGHT,
                                texture_index[1],
                            ));
                        }
                    } else {
                        if let Some(c) = &adjacent_chunks[1] {
                            if c.cubes[0 + CHUNK_USIZE * z + CHUNK_USIZE * CHUNK_USIZE * y]
                                .cube_type
                                == CubeType::AIR
                            {
                                faces.push(Quad::new(
                                    Coord3DI::new(pos_x, pos_y, pos_z),
                                    Rotation::RIGHT,
                                    texture_index[1],
                                ));
                            }
                        }
                    }
                    if z > 0 {
                        if self.cubes[x + CHUNK_USIZE * (z - 1) + CHUNK_USIZE * CHUNK_USIZE * y]
                            .cube_type
                            == CubeType::AIR
                        {
                            faces.push(Quad::new(
                                Coord3DI::new(pos_x, pos_y, pos_z),
                                Rotation::BACK,
                                texture_index[2],
                            ));
                        }
                    } else {
                        if let Some(c) = &adjacent_chunks[2] {
                            if c.cubes[x
                                + CHUNK_USIZE * (CHUNK_USIZE - 1)
                                + CHUNK_USIZE * CHUNK_USIZE * y]
                                .cube_type
                                == CubeType::AIR
                            {
                                faces.push(Quad::new(
                                    Coord3DI::new(pos_x, pos_y, pos_z),
                                    Rotation::BACK,
                                    texture_index[2],
                                ));
                            }
                        }
                    }
                    if z < CHUNK_USIZE - 1 {
                        if self.cubes[x + CHUNK_USIZE * (z + 1) + CHUNK_USIZE * CHUNK_USIZE * y]
                            .cube_type
                            == CubeType::AIR
                        {
                            faces.push(Quad::new(
                                Coord3DI::new(pos_x, pos_y, pos_z),
                                Rotation::FRONT,
                                texture_index[3],
                            ));
                        }
                    } else {
                        if let Some(c) = &adjacent_chunks[3] {
                            if c.cubes[x + CHUNK_USIZE * 0 + CHUNK_USIZE * CHUNK_USIZE * y]
                                .cube_type
                                == CubeType::AIR
                            {
                                faces.push(Quad::new(
                                    Coord3DI::new(pos_x, pos_y, pos_z),
                                    Rotation::FRONT,
                                    texture_index[3],
                                ));
                            }
                        }
                    }
                    if y > 0 {
                        if self.cubes[x + CHUNK_USIZE * z + CHUNK_USIZE * CHUNK_USIZE * (y - 1)]
                            .cube_type
                            == CubeType::AIR
                        {
                            faces.push(Quad::new(
                                Coord3DI::new(pos_x, pos_y, pos_z),
                                Rotation::DOWN,
                                texture_index[5],
                            ));
                        }
                    } else {
                        faces.push(Quad::new(
                            Coord3DI::new(pos_x, pos_y, pos_z),
                            Rotation::DOWN,
                            texture_index[5],
                        ));
                    }
                    if y < CHUNK_USIZE - 1 {
                        if self.cubes[x + CHUNK_USIZE * z + CHUNK_USIZE * CHUNK_USIZE * (y + 1)]
                            .cube_type
                            == CubeType::AIR
                        {
                            faces.push(Quad::new(
                                Coord3DI::new(pos_x, pos_y, pos_z),
                                Rotation::UP,
                                texture_index[4],
                            ));
                        }
                    } else {
                        faces.push(Quad::new(
                            Coord3DI::new(pos_x, pos_y, pos_z),
                            Rotation::UP,
                            texture_index[4],
                        ));
                    }
                }
            }
        }
        faces
    }
}

pub struct ChunkMesh {
    instance_buffer: wgpu::Buffer,
    instances_len: usize,
}

impl ChunkMesh {
    pub fn new(device: &wgpu::Device, faces: Vec<Quad>) -> Self {
        let instance_data = faces.iter().map(Quad::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(instance_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let instances_len = faces.len();
        Self {
            instance_buffer,
            instances_len,
        }
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, render_data: &'a RenderPassData) {
        pass.set_vertex_buffer(0, render_data.face_vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        pass.set_index_buffer(
            render_data.face_index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        pass.set_bind_groups(&render_data);
        pass.draw_indexed(0..render_data.indices_len, 0, 0..self.instances_len as _);
    }
}
