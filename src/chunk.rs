use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::cube::Cube;
use crate::quad;
use crate::quad::{Quad, Rotation};
use crate::renderer::graphics::Graphics;
use crate::uniform::{SetUniforms, UniformManager};
use simdnoise::*;
use wgpu::util::DeviceExt;
use rand::prelude::*;

const CHUNK_LENGTH: usize = 16;
const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 16;

const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT;

pub struct Chunk {
    position: ChunkCoord3D,
    cubes: Vec<Cube>,
    faces: Vec<Quad>,
    mesh: ChunkMesh,
}

impl Chunk {
    pub fn new(graphics: &Graphics, position: ChunkCoord3D) -> Self {
        let mut cubes = Vec::new();

        Chunk::build(&mut cubes, position);
        let faces = Chunk::generate_faces(&mut cubes);

        let mesh = ChunkMesh::new(&graphics, &faces);
        Self {
            position,
            cubes,
            faces,
            mesh,
        }
    }

    fn build(cubes: &mut Vec<Cube>, pos: ChunkCoord3D) {
        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(0..10000);
        let perlin = underground_perlin_3d(seed);
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_LENGTH {
                    if perlin[x + 16 * z + 16 * 16 * y] > 0.5 {
                        cubes.insert((x + 16 * z + 16 * 16 * y),Cube::new(Coord3D::new(x as i32 + pos.x * 16, y as i32 + pos.y * 16, z as i32 + pos.z * 16), false));
                    } else {
                        cubes.insert((x + 16 * z + 16 * 16 * y),Cube::new(Coord3D::new(x as i32 + pos.x * 16, y as i32 + pos.y * 16, z as i32 + pos.z * 16), true));
                    }
                }
            }
        }
    }

    fn generate_faces(cubes: &Vec<Cube>) -> Vec<Quad> {
        let mut quads = Vec::new();
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_LENGTH {
                    if cubes[x + 16 * z + 16 * 16 * y].is_air == true {
                        continue;
                    }
                    let pos_x = cubes[x + 16 * z + 16 * 16 * y].position.x;
                    let pos_y = cubes[x + 16 * z + 16 * 16 * y].position.y;
                    let pos_z = cubes[x + 16 * z + 16 * 16 * y].position.z;
                    if x > 0 {
                        if cubes[(x - 1) + 16 * z + 16 * 16 * y].is_air == true {
                            quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::LEFT, 2));
                        }
                    } else {
                        quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::LEFT, 2));
                    }
                    if x < 16 - 1 {
                        if cubes[(x + 1) + 16 * z + 16 * 16 * y].is_air == true {
                            quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::RIGHT, 2));
                        }
                    } else {
                        quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::RIGHT, 2));
                    }
                    if z > 0 {
                        if cubes[x + 16 * (z - 1) + 16 * 16 * y].is_air == true {
                            quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::BACK, 2));
                        }
                    } else {
                        quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::BACK, 2));
                    }
                    if z < 16 - 1 {
                        if cubes[x + 16 * (z + 1) + 16 * 16 * y].is_air == true {
                            quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::FRONT, 2));
                        }
                    } else {
                        quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::FRONT, 2));
                    }
                    if y > 0 {
                        if cubes[x + 16 * z + 16 * 16 * (y - 1)].is_air == true {
                            quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::DOWN, 2));
                        }
                    } else {
                        quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::DOWN, 2));
                    }
                    if y < 16 - 1 {
                        if cubes[x + 16 * z + 16 * 16 * (y + 1)].is_air == true {
                            quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::UP, 2));
                        }
                    } else {
                        quads.push(Quad::new(Coord3D::new(pos_x, pos_y, pos_z), Rotation::UP, 2));
                    }
                }
            }
        }
        quads
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, uniform: &'a UniformManager) {
        self.mesh.render(pass, uniform);
    }
}

fn underground_perlin_3d(seed: i32) -> Vec<f32> {
    /*NoiseBuilder::fbm_3d(CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_LENGTH)
        .with_seed(seed).with_freq(0.1)
        .generate_scaled(0., 1.)*/
    //NoiseBuilder::gradient_3d(CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_LENGTH).with_seed(seed).generate_scaled(0., 1.)
    NoiseBuilder::gradient_3d(CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_LENGTH).with_seed(seed).generate_scaled(0., 1.)
}

pub struct ChunkMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    indices_len: usize,
    instances_len: usize,
}

impl ChunkMesh {
    fn new(graphics: &Graphics, instances: &Vec<Quad>) -> Self {
        let vertex_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(quad::VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(quad::INDICES),
                usage: wgpu::BufferUsage::INDEX,
            });
        let instance_data = instances.iter().map(Quad::to_raw).collect::<Vec<_>>();
        let instance_buffer =
            graphics
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
                });
        let indices_len = quad::INDICES.len();
        let instances_len = instances.len();
        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            indices_len,
            instances_len,
        }
    }

    fn update(&mut self, graphics: &Graphics, faces: &Vec<Quad>) {
        let instance_data = faces.iter().map(Quad::to_raw).collect::<Vec<_>>();
        self.instance_buffer =
            graphics
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
                });
        self.instances_len = faces.len();
    }

    fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, uniform: &'a UniformManager) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_bind_groups(&uniform);
        pass.draw_indexed(0..self.indices_len as _, 0, 0..self.instances_len as _);
    }
}
