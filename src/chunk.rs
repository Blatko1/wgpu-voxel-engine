use crate::coordinate::{ChunkCoord3D, Coord3D};
use crate::cube::Cube;
use crate::quad::{self, Quad};
use crate::renderer::graphics::Graphics;
use crate::renderer::vertex::Vertex;
use crate::uniform::{SetUniforms, UniformManager};
use wgpu::util::DeviceExt;

const CHUNK_LENGTH: usize = 16;
const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 16;

const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT;

pub struct Chunk {
    pos: ChunkCoord3D,
    voxels: [Cube; CHUNK_SIZE],
    chunk_mesh: ChunkMesh,
    faces: Vec<Quad>,
}

impl Chunk {
    pub fn new(graphics: &Graphics, coord: Coord3D) -> Self {
        let default = Cube::default();
        let mut voxels: [Cube; CHUNK_SIZE] = [default; CHUNK_SIZE];

        let faces = Chunk::create_quads(&mut voxels, coord);

        let chunk_mesh = ChunkMesh::new(&graphics, quad::VERTICES, quad::INDICES, &faces);
        Self {
            pos: coord.to_chunk_coord(),
            voxels,
            chunk_mesh,
            faces,
        }
    }

    fn create_quads(voxels: &mut [Cube; CHUNK_SIZE], coord: Coord3D) -> Vec<Quad> {
        let mut faces: Vec<Quad> = Vec::new();
        // Filtering unseen faces.
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

    pub fn get_cube(&self, coord: Coord3D) -> &Cube {
        self.voxels.get(coord.to_cube_index()).unwrap()
    }

    pub fn remove_cube(&mut self, coord: Coord3D) {
        self.voxels[coord.to_cube_index()].is_active = false;

        self.update_nearest_cubes(coord);
    }

    pub fn update(&mut self, graphics: &Graphics) {
        self.chunk_mesh.update(&graphics, &self.faces);
    }

    fn update_nearest_cubes(&mut self, coord: Coord3D) {
        // Remove faces.
        if self.voxels[coord.to_cube_index()].front_face == true {
            self.faces.remove(coord.to_cube_index());
            println!("front");
        }
        if self.voxels[coord.to_cube_index()].back_face == true {
            self.faces.remove(coord.to_cube_index() + 1);
            println!("back");
        }
        if self.voxels[coord.to_cube_index()].left_face == true {
            self.faces.remove(coord.to_cube_index() + 2);
            println!("left");
        }
        if self.voxels[coord.to_cube_index()].right_face == true {
            self.faces.remove(coord.to_cube_index() + 3);
            println!("right");
        }
        if self.voxels[coord.to_cube_index()].top_face == true {
            self.faces.remove(coord.to_cube_index() + 4);
            println!("top");
        }
        if self.voxels[coord.to_cube_index()].bottom_face == true {
            self.faces.remove(coord.to_cube_index() + 5);
            println!("bottom");
        }

        if coord.x > 0 {
            if self.voxels[coord.to_cube_index() - 1].is_active == true {
                self.voxels[coord.to_cube_index() - 1].right_face = true;
            }
        }
        if coord.x < 15 {
            if self.voxels[coord.to_cube_index() + 1].is_active == true {
                self.voxels[coord.to_cube_index() + 1].left_face = true;
            }
        }
        if coord.z > 0 {}
        if coord.z < 15 {}
        if coord.y > 0 {}
        if coord.y < 15 {}
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, uniform: &'a UniformManager) {
        self.chunk_mesh.render(pass, &uniform);
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
    fn new(
        graphics: &Graphics,
        vertices: &[Vertex],
        indices: &[u32],
        instances: &Vec<Quad>,
    ) -> Self {
        let vertex_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
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
