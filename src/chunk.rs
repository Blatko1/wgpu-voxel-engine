use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::cube::{Cube, CubeType};
use crate::perlin_noise;
use crate::quad::{self, Quad, Rotation};
use crate::uniform::{SetUniforms, UniformManager};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::Arc;
use wgpu::util::DeviceExt;

const CHUNK_LENGTH: usize = 32;
const CHUNK_WIDTH: usize = 32;
const CHUNK_HEIGHT: usize = 32;

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
        let faces = Chunk::generate_faces(&mut cubes);
        Self {
            position,
            cubes,
            faces,
        }
    }

    fn generate_terrain(cubes: &mut Vec<Cube>, pos: ChunkCoord3D) {
        let noise = perlin_noise::perlin_3d(pos);
        cubes.into_par_iter().enumerate().for_each(|(i, cube)| {
            let x: i32 = cube.position.x;
            let y: i32 = cube.position.y;
            let z: i32 = cube.position.z;
            if noise[i] > 0. {
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

    fn generate_faces(cubes: &Vec<Cube>) -> Vec<Quad> {
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
                                cubes[x + 32 * z + 32 * 32 * y].texture_index[0],
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::LEFT,
                            cubes[x + 32 * z + 32 * 32 * y].texture_index[0],
                        ));
                    }
                    if x < 32 - 1 {
                        if cubes[(x + 1) + 32 * z + 32 * 32 * y].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::RIGHT,
                                cubes[x + 32 * z + 32 * 32 * y].texture_index[1],
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::RIGHT,
                            cubes[x + 32 * z + 32 * 32 * y].texture_index[1],
                        ));
                    }
                    if z > 0 {
                        if cubes[x + 32 * (z - 1) + 32 * 32 * y].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::BACK,
                                cubes[x + 32 * z + 32 * 32 * y].texture_index[2],
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::BACK,
                            cubes[x + 32 * z + 32 * 32 * y].texture_index[2],
                        ));
                    }
                    if z < 32 - 1 {
                        if cubes[x + 32 * (z + 1) + 32 * 32 * y].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::FRONT,
                                cubes[x + 32 * z + 32 * 32 * y].texture_index[3],
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::FRONT,
                            cubes[x + 32 * z + 32 * 32 * y].texture_index[3],
                        ));
                    }
                    if y > 0 {
                        if cubes[x + 32 * z + 32 * 32 * (y - 1)].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::DOWN,
                                cubes[x + 32 * z + 32 * 32 * y].texture_index[5],
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::DOWN,
                            cubes[x + 32 * z + 32 * 32 * y].texture_index[5],
                        ));
                    }
                    if y < 32 - 1 {
                        if cubes[x + 32 * z + 32 * 32 * (y + 1)].is_air == true {
                            quads.push(Quad::new(
                                Coord3D::new(pos_x, pos_y, pos_z),
                                Rotation::UP,
                                cubes[x + 32 * z + 32 * 32 * y].texture_index[4],
                            ));
                        }
                    } else {
                        quads.push(Quad::new(
                            Coord3D::new(pos_x, pos_y, pos_z),
                            Rotation::UP,
                            cubes[x + 32 * z + 32 * 32 * y].texture_index[4],
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
                        CubeType::GRASS,
                    ));
                }
            }
        }
    }

    pub fn create_mesh(&self, device: Arc<wgpu::Device>) -> ChunkMesh {
        ChunkMesh::new(&device, &self.faces)
    }
}

pub struct ChunkMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    indices_len: usize,
    instances_len: usize,
}

impl ChunkMesh {
    pub fn new(device: &wgpu::Device, faces: &Vec<Quad>) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(quad::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(quad::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let instance_data = faces.iter().map(Quad::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(instance_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let indices_len = quad::INDICES.len();
        let instances_len = faces.len();
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
