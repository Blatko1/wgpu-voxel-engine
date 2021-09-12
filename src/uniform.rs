use crate::camera::Camera;
use crate::renderer::graphics::Graphics;
use crate::texture::Texture;
use std::num::NonZeroU32;
use wgpu::util::DeviceExt;
use crate::quad;

pub struct RenderPassData {
    pub face_vertex_buffer: wgpu::Buffer,
    pub face_index_buffer: wgpu::Buffer,
    pub indices_len: u32,

    global_matrix: GlobalMatrix,
    texture_array: SampledTextureArray,
}

impl RenderPassData {
    pub fn new(graphics: &Graphics, camera: &Camera) -> Self {
        let face_vertex_buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("face vertex buffer"),
            contents: bytemuck::cast_slice(quad::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let face_index_buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("face index buffer"),
            contents: bytemuck::cast_slice(quad::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let indices_len = quad::INDICES.len() as u32;
        let global_matrix = GlobalMatrix::new(&graphics, &camera);
        let texture_array = SampledTextureArray::new(
            &graphics,
            Texture::load_textures(&graphics).unwrap(),
            Texture::create_sampler(&graphics),
        );

        Self {
            face_vertex_buffer,
            face_index_buffer,
            indices_len,
            global_matrix,
            texture_array,
        }
    }

    pub fn update(&mut self, camera: &Camera, graphics: &Graphics) {
        self.global_matrix.update(&camera, &graphics);
    }

    pub fn bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout> {
        let mut layouts = Vec::new();
        layouts.push(&self.global_matrix.bind_group_layout);
        layouts.push(&self.texture_array.bind_group_layout);
        layouts
    }
}

pub trait SetUniforms<'a> {
    fn set_bind_groups(&mut self, uniform: &'a RenderPassData);
}

impl<'a> SetUniforms<'a> for wgpu::RenderPass<'a> {
    fn set_bind_groups(&mut self, render_data: &'a RenderPassData) {
        self.set_bind_group(0, &render_data.global_matrix.bind_group, &[]);
        self.set_bind_group(1, &render_data.texture_array.bind_group, &[]);
    }
}

struct Uniform<T> {
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    buffer: Option<wgpu::Buffer>,
    data: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixData {
    pub proj_view_model_matrix: [[f32; 4]; 4],
}

type GlobalMatrix = Uniform<MatrixData>;

impl GlobalMatrix {
    pub fn new(graphics: &Graphics, camera: &Camera) -> Self {
        let proj_view_model_matrix: [[f32; 4]; 4] = camera.global_matrix.into();
        let data = MatrixData {
            proj_view_model_matrix,
        };

        let buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Global Matrix Data Buffer"),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            });
        let bind_group_layout =
            graphics
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Global Matrix bind group layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::all(),
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let bind_group = graphics
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Global Matrix bind group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });
        Self {
            bind_group,
            bind_group_layout,
            buffer: Some(buffer),
            data,
        }
    }

    fn update(&mut self, camera: &Camera, graphics: &Graphics) {
        let proj_view_model_matrix: [[f32; 4]; 4] = camera.global_matrix.into();
        self.data = MatrixData {
            proj_view_model_matrix,
        };
        graphics.queue.write_buffer(
            &self.buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&[self.data]),
        );
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum LayoutType {
    GlobalMatrix,
}

pub struct TextureArray {
    textures: Vec<Texture>,
    sampler: wgpu::Sampler,
}

impl TextureArray {
    fn get_ref(&self) -> Vec<&wgpu::TextureView> {
        let array = self.textures.iter().map(|t| &t.view).collect::<Vec<_>>();
        array
    }
}

pub type SampledTextureArray = Uniform<TextureArray>;

impl SampledTextureArray {
    pub fn new(graphics: &Graphics, textures: Vec<Texture>, sampler: wgpu::Sampler) -> Self {
        let data = TextureArray { textures, sampler };
        let bind_group_layout =
            graphics
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Sampled texture array bind group layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler {
                                filtering: true,
                                comparison: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: NonZeroU32::new(Texture::TEXTURE_ARRAY_SIZE),
                        },
                    ],
                });

        let bind_group = graphics
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Sampled texture array bind group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&data.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureViewArray(&data.get_ref()),
                    },
                ],
            });

        Self {
            bind_group_layout,
            bind_group,
            buffer: None,
            data,
        }
    }
}
