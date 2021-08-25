use crate::camera::Camera;
use crate::renderer::graphics::Graphics;
use crate::texture::Texture;
use std::num::NonZeroU32;
use wgpu::util::DeviceExt;

pub struct UniformManager {
    global_matrix: GlobalMatrix,
    texture_array: SampledTextureArray
}

impl UniformManager {
    pub fn new(graphics: &Graphics, camera: &Camera) -> Self {
        let global_matrix = GlobalMatrix::new(&graphics, &camera);
        let texture_array =
            SampledTextureArray::new(&graphics, Texture::load_textures(&graphics).unwrap(), Texture::create_sampler(&graphics));

        Self { global_matrix, texture_array }
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
    fn set_bind_groups(&mut self, uniform: &'a UniformManager);
}

impl<'a> SetUniforms<'a> for wgpu::RenderPass<'a> {
    fn set_bind_groups(&mut self, uniform: &'a UniformManager) {
        self.set_bind_group(0, &uniform.global_matrix.bind_group, &[]);
        self.set_bind_group(1, &uniform.texture_array.bind_group, &[]);
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
        let data = camera.create_global_matrix();

        let buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Global Matrix Data Buffer"),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            });
        let bind_group_layout =
            graphics
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Global Matrix bind group layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::all(),
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
        self.data = camera.create_global_matrix();
        graphics.queue.write_buffer(
            &self.buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&[self.data]),
        );
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
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

type SampledTextureArray = Uniform<TextureArray>;

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
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler {
                                filtering: true,
                                comparison: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
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
