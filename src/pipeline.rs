use crate::graphics::Graphics;
use std::fs;
use std::path::Path;
use crate::vertex::Vertex;
use crate::instance::InstanceRaw;
use crate::uniform::UniformManager;

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    fn new<P: AsRef<Path>>(
        graphics: &Graphics,
        label: &str,
        v_shader: P,
        f_shader: P,
        bind_group_layouts: Vec<&wgpu::BindGroupLayout>,
        vertex_layout: Vec<wgpu::VertexBufferLayout>,
        depth_format: Option<wgpu::TextureFormat>,
    ) -> Self {
        let vertex_shader = graphics
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(&format!("{} vertex shader", label)),
                source: wgpu::util::make_spirv(&fs::read(v_shader).unwrap()),
                flags: wgpu::ShaderFlags::all(),
            });
        let fragment_shader = graphics
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(&format!("{} fragment shader", label)),
                source: wgpu::util::make_spirv(&fs::read(f_shader).unwrap()),
                flags: wgpu::ShaderFlags::all(),
            });
        let layout = graphics
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("{} pipeline layout", label)),
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &[],
            });
        let pipeline = graphics
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(&format!("{} render pipeline", label)),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "main",
                    buffers: &vertex_layout,
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    clamp_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: depth_format.map(|f| wgpu::DepthStencilState {
                    format: f,
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
                    module: &fragment_shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: graphics.sc_desc.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
            });
        Self { pipeline }
    }

    pub fn main_pipeline(graphics: &Graphics, uniform: &UniformManager) -> Pipeline {
        let shader_dir = std::path::Path::new(std::env::current_dir().unwrap().as_os_str()).join("src/shaders");
        let bind_group_layouts: Vec<&wgpu::BindGroupLayout> = vec![];
        let vertex_buffer_layouts = vec![Vertex::init_buffer_layout(), InstanceRaw::init_buffer_layout()];
        Pipeline::new(
            &graphics,
            "main",
            shader_dir.join("vertex.vert.spv"),
            shader_dir.join("fragment.frag.spv"),
            bind_group_layouts,
            vertex_buffer_layouts,
            None,
        )
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum Type {
    Main,
    Light,
}
