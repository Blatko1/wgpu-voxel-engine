use crate::graphics::Graphics;
use crate::pipeline::{Pipeline, Type};
use std::collections::HashMap;
use crate::uniform::UniformManager;

pub struct Renderer {
    pub pipelines: HashMap<Type, Pipeline>,
}

impl Renderer {
    pub fn new(graphics: &Graphics, uniforms: &UniformManager) -> Self {
        let mut pipelines = HashMap::new();
        pipelines.insert(Type::Main, Pipeline::main_pipeline(&graphics, uniforms));
        Self { pipelines }
    }

    pub fn render<T: Renderable>(
        &self,
        graphics: &Graphics,
        items: &[&T],
    ) -> Result<(), wgpu::SwapChainError> {
        let mut encoder = graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Command Encoder"),
            });
        let view = &graphics.swap_chain.get_current_frame()?.output.view;
        let render_pass_builder = RenderPassBuilder::init(view);
        for i in items {
            let desc = render_pass_builder.build();
            let mut pass = encoder.begin_render_pass(&desc);
            i.render(&mut pass, &self);
        }
        graphics.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}

pub trait Renderable {
    fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, renderer: &'a Renderer);
}

struct RenderPassBuilder<'a> {
    color_attachments: Vec<wgpu::RenderPassColorAttachment<'a>>,
    depth_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
}

impl<'a> RenderPassBuilder<'a> {
    fn init(view: &'a wgpu::TextureView) -> Self {
        let color_attachments = vec![wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0
                }),
                store: true,
            },
        }];
        /*
        Some(wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        })
         */
        let depth_attachment = None;
        Self {
            color_attachments,
            depth_attachment,
        }
    }
    fn build(
        &self,
    ) -> wgpu::RenderPassDescriptor {
        wgpu::RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &self.color_attachments,
            depth_stencil_attachment: self.depth_attachment.clone(),
        }
    }
}
