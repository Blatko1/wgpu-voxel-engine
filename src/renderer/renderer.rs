use super::graphics::Graphics;
use super::pipeline::{Pipeline, Type};
use crate::camera::Camera;
use crate::debug_info::DebugInfo;
use crate::texture::Texture;
use crate::uniform::UniformManager;
use crate::world::World;
use std::collections::HashMap;

pub struct Renderer {
    pub pipelines: HashMap<Type, Pipeline>,
    depth_texture_view: wgpu::TextureView,
}

impl Renderer {
    pub fn new(graphics: &Graphics, uniforms: &UniformManager) -> Self {
        let mut pipelines = HashMap::new();
        pipelines.insert(Type::Main, Pipeline::main_pipeline(&graphics, uniforms));
        let depth_texture_view = Texture::create_depth_texture_view(&graphics);
        Self {
            pipelines,
            depth_texture_view,
        }
    }

    pub fn render(
        &self,
        graphics: &Graphics,
        world: &World,
        uniform: &UniformManager,
        debug_info: &mut DebugInfo,
        camera: &Camera,
    ) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Command Encoder"),
            });
        let frame = graphics.surface.get_current_frame()?.output;
        {
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let render_pass_builder = RenderPassBuilder::init(&view, &self.depth_texture_view);
            let desc = render_pass_builder.build();
            let mut pass = encoder.begin_render_pass(&desc);
            world.render(&mut pass, &self, &uniform);
        }
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        debug_info
            .draw(&graphics, &mut encoder, &view, &camera)
            .unwrap();
        debug_info.finish();
        graphics.queue.submit(Some(encoder.finish()));

        Ok(())
    }

    pub fn resize(&mut self, graphics: &Graphics) {
        self.depth_texture_view = Texture::create_depth_texture_view(&graphics);
    }
}

pub trait Renderable {
    fn render<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        renderer: &'a Renderer,
        uniform: &'a UniformManager,
    );
}

struct RenderPassBuilder<'a> {
    color_attachments: Vec<wgpu::RenderPassColorAttachment<'a>>,
    depth_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
}

impl<'a> RenderPassBuilder<'a> {
    fn init(frame: &'a wgpu::TextureView, depth_view: &'a wgpu::TextureView) -> Self {
        let color_attachments = vec![wgpu::RenderPassColorAttachment {
            view: frame,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: true,
            },
        }];

        let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        });
        Self {
            color_attachments,
            depth_attachment,
        }
    }
    fn build(&self) -> wgpu::RenderPassDescriptor {
        wgpu::RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &self.color_attachments,
            depth_stencil_attachment: self.depth_attachment.clone(),
        }
    }
}
