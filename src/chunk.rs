use crate::graphics::Graphics;
use wgpu::util::DeviceExt;
use crate::vertex::Vertex;
use crate::quad::{self, Quad};
use crate::cube::Cube;

const CHUNK_LENGTH: usize = 16;
const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 16;

const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT;

pub struct Chunk {
    voxels: [Cube; CHUNK_SIZE],
    chunk_mesh: ChunkMesh
}

impl Chunk {
    pub fn new(graphics: &Graphics) -> Self {
        let default = Cube::default();
        let mut voxels: [Cube; CHUNK_SIZE] = [default; CHUNK_SIZE];

        let instances = Chunk::filter_quads(&mut voxels);

        let chunk_mesh = ChunkMesh::new(&graphics, quad::VERTICES, quad::INDICES, instances);
        Self {
            voxels,
            chunk_mesh
        }
    }

    fn filter_quads(voxels: &mut [Cube; CHUNK_SIZE]) -> Vec<Quad> {
        let mut faces: Vec<Quad> = Vec::new();
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    if voxels[x + 16 * z + 16 * 16 * y].is_active == false {
                        continue;
                    }
                    let mut left_face = true;
                    if x > 0 {
                        left_face = !voxels[(x - 1) + 16 * z + 16 * 16 * y].is_active;
                    }
                    let mut right_face = true;
                    if x < CHUNK_WIDTH - 1 {
                        right_face = !voxels[(x + 1) + 16 * z + 16 * 16 * y].is_active;
                    }
                    let mut back_face = true;
                    if z > 0 {
                        back_face = !voxels[x + 16 * (z - 1) + 16 * 16 * y].is_active;
                    }
                    let mut front_face = true;
                    if z < CHUNK_LENGTH - 1 {
                        front_face = !voxels[x + 16 * (z + 1) + 16 * 16 * y].is_active;
                    }
                    let mut bottom_face = true;
                    if y > 0 {
                        bottom_face = !voxels[x + 16 * z + 16 * 16 * (y - 1)].is_active;
                    }
                    let mut top_face = true;
                    if y < CHUNK_HEIGHT - 1 {
                        top_face = !voxels[x + 16 * z + 16 * 16 * (y + 1)].is_active;
                    }
                    voxels[x + 16 * z + 16 * 16 * y] = Cube::new(
                        top_face,
                        bottom_face,
                        left_face,
                        right_face,
                        back_face,
                        front_face,
                    );
                    faces.append(
                        &mut voxels[x + 16 * z + 16 * 16 * y]
                            .get_faces([x as f32, y as f32, z as f32]),
                    );
                }
            }
        }
        return faces;
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        self.chunk_mesh.render(pass);
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
    fn new(graphics: &Graphics, vertices: &[Vertex], indices: &[u32], instances: Vec<Quad>) -> Self {
        let vertex_buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let instance_data = instances.iter().map(Quad::to_raw).collect::<Vec<_>>();
        let instance_buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });
        let indices_len = indices.len();
        let instances_len = instances.len();
        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            indices_len,
            instances_len,
        }
    }

    fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.indices_len as _, 0, 0..1);   //self.instances_len as _
    }
}