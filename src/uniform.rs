use crate::graphics::Graphics;
use crate::camera::Camera;
use wgpu::util::DeviceExt;
use std::collections::HashMap;

pub struct UniformManager {
    global_matrix: GlobalMatrix,
}

impl UniformManager {
    pub fn new(graphics: &Graphics, camera: &Camera) -> Self {
        let global_matrix = GlobalMatrix::new(&graphics, &camera);

        Self {
            global_matrix,
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.global_matrix.update(&camera);
    }

    pub fn bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout> {
        let mut layouts = Vec::new();
        layouts.push(&self.global_matrix.bind_group_layout);
        layouts
    }

}

pub trait SetUniforms<'a> {
    fn set_bind_groups(&mut self, uniform: &'a UniformManager);
}

impl<'a> SetUniforms<'a> for wgpu::RenderPass<'a> {
    fn set_bind_groups(&mut self, uniform: &'a UniformManager) {
        self.set_bind_group(0, &uniform.global_matrix.bind_group, &[]);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixData {
    pub proj_view_model_matrix: [[f32; 4]; 4]
}

struct Uniform<T> {
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    buffer: wgpu::Buffer,
    data: T
}

type GlobalMatrix = Uniform<MatrixData>;

impl GlobalMatrix {
    pub fn new(graphics: &Graphics, camera: &Camera) -> Self {
        let data = camera.create_global_matrix();

        let buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Global Matrix Data Buffer"),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM
        });
        let bind_group_layout = graphics.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Global Matrix bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::all(),
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ]
        });

        let bind_group = graphics.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Global Matrix bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });
        Self {
            bind_group,
            bind_group_layout,
            buffer,
            data
        }
    }

    fn update(&mut self, camera: &Camera) {
        self.data = camera.create_global_matrix();
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum LayoutType {
    GlobalMatrix
}