use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::cube::Cube;
use crate::quad::{self, Quad, Rotation};
use crate::renderer::graphics::Graphics;
use crate::terrain_generator::TerrainGenerator;
use crate::uniform::{SetUniforms, UniformManager};
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::time::{SystemTime, UNIX_EPOCH};
use wgpu::util::DeviceExt;

const CHUNK_LENGTH: usize = 32;
const CHUNK_WIDTH: usize = 32;
const CHUNK_HEIGHT: usize = 32;

const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT;

pub struct Chunk {
    position: ChunkCoord3D,
    cubes: Vec<Cube>,
    faces: Vec<Quad>,
}

impl Chunk {
    pub fn new(position: ChunkCoord3D) -> Self {
        let mut cubes = Vec::new();
        Chunk::empty_chunk(&mut cubes, position);
        Chunk::generate_terrain(&mut cubes, position);
        let faces = Chunk::generate_faces(&mut cubes, position);
        Self {
            position,
            cubes,
            faces,
        }
    }

    fn generate_terrain(cubes: &mut Vec<Cube>, pos: ChunkCoord3D) {
        let noise = TerrainGenerator::new(1);
        cubes.into_par_iter().enumerate().for_each(|(i, cube)| {
            let x: i32 = cube.position.x;
            let y: i32 = cube.position.y;
            let z: i32 = cube.position.z;
            let perlin = noise.perlin_3d(x, y, z);
            if perlin > 0. {
                cube.set_air(false);
            } else {
                cube.set_air(true);
            }
        });
    }

    fn local_coords(index: usize) -> (i32, i32, i32) {
        let y = index / 1024;
        let z = (index % 1024) / 32;
        let x = (index % 1024) % 32;
        (x as i32, z as i32, y as i32)
    }

    fn generate_faces(cubes: &Vec<Cube>, pos: ChunkCoord3D) -> Vec<Quad> {
        let mut quads = Vec::new();
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_LENGTH {
                    if cubes[x + 32 * z + 32 * 32 * y].is_air == true {
                        continue;
                    }
                    let pos_x = cubes[x + 32 * z + 32 * 32 * y].position.x;
                    let pos_y = cubes[x + 32 * z + 32 * 32 * y].position.y;
                    let pos_z = cubes[x + 32 * z + 32 * 32 * y].position.z;
                    if x > 0 {
                        if cubes[(x - 1) + 32 * z + 32 * 32 * y].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::LEFT,
                                2,
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::LEFT,
                            2,
                        ));
                    }
                    if x < 32 - 1 {
                        if cubes[(x + 1) + 32 * z + 32 * 32 * y].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::RIGHT,
                                2,
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::RIGHT,
                            2,
                        ));
                    }
                    if z > 0 {
                        if cubes[x + 32 * (z - 1) + 32 * 32 * y].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::BACK,
                                2,
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::BACK,
                            2,
                        ));
                    }
                    if z < 32 - 1 {
                        if cubes[x + 32 * (z + 1) + 32 * 32 * y].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::FRONT,
                                2,
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::FRONT,
                            2,
                        ));
                    }
                    if y > 0 {
                        if cubes[x + 32 * z + 32 * 32 * (y - 1)].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::DOWN,
                                2,
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::DOWN,
                            2,
                        ));
                    }
                    if y < 32 - 1 {
                        if cubes[x + 32 * z + 32 * 32 * (y + 1)].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::UP,
                                2,
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::UP,
                            2,
                        ));
                    }
                }
            }
        }
        quads
    }

    fn empty_chunk(cubes: &mut Vec<Cube>, pos: ChunkCoord3D) {
        for y in 0..32 {
            for z in 0..32 {
                for x in 0..32 {
                    cubes.push(Cube::new(
                        Coord3D::new(x + pos.x * 32, y + pos.y * 32, z + pos.z * 32),
                        true,
                    ));
                }
            }
        }
    }

    pub fn generate_data(&self) -> ChunkMeshData {
        let vertex_data = bytemuck::cast_slice(quad::VERTICES).to_vec();
        let index_data = bytemuck::cast_slice(quad::INDICES).to_vec();
        let instance_raw = self.faces.iter().map(Quad::to_raw).collect::<Vec<_>>();
        let instance_data = bytemuck::cast_slice(&instance_raw).to_vec();
        let instance_len = self.faces.len();
        ChunkMeshData {
            vertex_data,
            index_data,
            instance_data,
            instance_len
        }
    }
}

pub struct ChunkMeshData {
    vertex_data: Vec<u8>,
    index_data: Vec<u8>,
    instance_data: Vec<u8>,
    instance_len: usize
}

pub struct ChunkMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    indices_len: usize,
    instances_len: usize,
}

impl ChunkMesh {
    pub fn new(graphics: &Graphics, data: ChunkMeshData) -> Self {
        /*let now = SystemTime::now();
        let time_then = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");*/
        let vertex_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data.vertex_data.as_slice(),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data.index_data.as_slice(),
                usage: wgpu::BufferUsage::INDEX,
            });
        let instance_buffer =
            graphics
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: data.instance_data.as_slice(),
                    usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
                });
        let indices_len = quad::INDICES.len();
        let instances_len = data.instance_len;
        /*let now = SystemTime::now();
        let time_now = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        let delta = time_now - time_then;
        println!("Delta: {}", delta.as_millis());*/
        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            indices_len,
            instances_len,
        }
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, uniform: &'a UniformManager) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_bind_groups(&uniform);
        pass.draw_indexed(0..self.indices_len as _, 0, 0..self.instances_len as _);
    }
}
