use crate::camera::Camera;
use crate::coordinate::{ChunkCoord3D, Coord3D, Coord3DF};
use crate::renderer::graphics::Graphics;
use crate::renderer::pipeline::Pipeline;
use crate::renderer::renderer::{Renderable, Renderer};
use crate::renderer::vertex::Vertex;
use crate::texture::Texture;
use crate::uniform::UniformManager;
use nalgebra::{Matrix4, Rotation3, Translation3, Vector3, Vector4};
use wgpu::util::DeviceExt;
use wgpu::RenderPass;

pub struct Frustum {
    v_fov: f32,
    planes: Vec<Plane>,
    pub object: FrustumObject,
}

impl Frustum {
    pub fn new(
        graphics: &Graphics,
        uniform: &UniformManager,
        camera: &Camera,
    ) -> Self {
        let v_fov: f32 = 2. * ((camera.fov.to_radians() / 2.).tan() * camera.aspect).atan();
        let object = FrustumObject::new(&graphics, uniform);
        let mat = camera.global_matrix;
        let planes = Frustum::matrix_to_planes(mat);
        Self {
            v_fov,
            planes,
            object,
        }
    }

    pub fn check(&mut self, pos: Coord3D, camera: &Camera) -> bool {
        //self.normalize();
        for i in 0..6 {
            let dist = self.planes[i].a * pos.x as f32 + self.planes[i].b * pos.y as f32 + self.planes[i].c * pos.z as f32 + self.planes[i].d;
                if dist < 0. {
                println!("Out");
                return false;
            }
        }
        println!("In!");
        true
    }

    pub fn update(&mut self, camera: &Camera) {
        let mat = camera.global_matrix;
        self.planes = Frustum::matrix_to_planes(mat);
    }

    fn matrix_to_planes(matrix: Matrix4<f32>) -> Vec<Plane> {
        let mut left = Plane::new();
        let mut right = Plane::new();
        let mut top = Plane::new();
        let mut bottom = Plane::new();
        let mut near = Plane::new();
        let mut far = Plane::new();
        let data = matrix.data.0;
        left.a = data[0][3] + data[0][0];
        left.b = data[1][3] + data[1][0];
        left.c = data[2][3] + data[2][0];
        left.d = data[3][3] + data[3][0];

        right.a = data[0][3] - data[0][0];
        right.b = data[1][3] - data[1][0];
        right.c = data[2][3] - data[2][0];
        right.d = data[3][3] - data[3][0];

        top.a = data[0][3] - data[0][1];
        top.b = data[1][3] - data[1][1];
        top.c = data[2][3] - data[2][1];
        top.d = data[3][3] - data[3][1];

        bottom.a = data[0][3] + data[0][1];
        bottom.b = data[1][3] + data[1][1];
        bottom.c = data[2][3] + data[2][1];
        bottom.d = data[3][3] + data[3][1];

        near.a = data[0][3] + data[0][2];
        near.b = data[1][3] + data[1][2];
        near.c = data[2][3] + data[2][2];
        near.d = data[3][3] + data[3][2];

        far.a = data[0][3] - data[0][2];
        far.b = data[1][3] - data[1][2];
        far.c = data[2][3] - data[2][2];
        far.d = data[3][3] - data[3][2];
        vec![near, far, left, right, top, bottom]
    }
}

#[derive(Debug)]
struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
        }
    }
}

pub struct FrustumObject {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices_len: u32,
}

impl FrustumObject {
    pub fn new(graphics: &Graphics, uniform: &UniformManager) -> Self {
        let v_shader = Pipeline::load_shader(&graphics, "frustum.vert.spv");
        let f_shader = Pipeline::load_shader(&graphics, "frustum.frag.spv");
        let layout = graphics
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Frustum pipeline layout"),
                bind_group_layouts: &[uniform.bind_group_layouts()[0]],
                push_constant_ranges: &[],
            });
        let pipeline = graphics
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Frustum pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &v_shader,
                    entry_point: "main",
                    buffers: &[Vertex::init_buffer_layout()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    clamp_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &f_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: graphics.surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
            });
        let vertex_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("frustum vertex buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("frustum index buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
        let indices_len = INDICES.len() as u32;

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            indices_len,
        }
    }
}

impl Renderable for FrustumObject {
    fn render<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        renderer: &'a Renderer,
        uniform: &'a UniformManager,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_bind_group(0, &uniform.global_matrix.bind_group, &[]);

        pass.draw_indexed(0..self.indices_len, 0, 0..1);
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1., -1., 0.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [1., -1., 0.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [-1., 1., 0.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [1., 1., 0.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [-50., -40., -50.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [50., -40., -50.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [-50., 40., -50.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [50., 40., -50.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [0., 0., -30.],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [0., 0., -35.],
        tex_coords: [0., 0.],
    },
];

const INDICES: &[u32] = &[
    0, 1, 1, 3, 3, 2, 2, 0, 0, 4, 1, 5, 2, 6, 3, 7, 4, 5, 5, 7, 7, 6, 6, 4, 8, 9
];