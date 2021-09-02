use crate::renderer::graphics::Graphics;
use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::num::NonZeroU32;
use std::path::Path;

pub struct Texture {
    texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    mip_level_count: u32,
}

impl Texture {
    pub const TEXTURE_ARRAY_SIZE: u32 = 4;

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture_view(graphics: &Graphics) -> wgpu::TextureView {
        let size = {
            wgpu::Extent3d {
                width: graphics.surface_config.width,
                height: graphics.surface_config.height,
                depth_or_array_layers: 1,
            }
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = graphics.device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        view
    }

    fn from_image(
        graphics: &Graphics,
        label: Option<&str>,
        image: DynamicImage,
    ) -> Result<Texture> {
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let mip_level_count = 1 + ((dimensions.0.max(dimensions.1) as f32).log2().floor() as u32);
        let texture = graphics.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_DST,
        });

        graphics.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: None,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Self {
            texture,
            view,
            mip_level_count,
        })
    }

    fn from_path<P: AsRef<Path>>(graphics: &Graphics, path: P) -> Result<Texture> {
        let buf = path.as_ref().to_path_buf();
        let label = buf.to_str();
        let img = image::open(path).expect("Couldn't find an image from path.");

        Self::from_image(graphics, label, img)
    }

    pub fn load_textures(graphics: &Graphics) -> Result<Vec<Texture>> {
        let path = std::path::Path::new(std::env::current_dir().unwrap().as_os_str()).join("res");

        let blit_shader = graphics
            .device
            .create_shader_module(&wgpu::include_wgsl!("shaders/blit.wgsl"));
        let mipmap_pipeline =
            graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("blit"),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &blit_shader,
                        entry_point: "vs_main",
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &blit_shader,
                        entry_point: "fs_main",
                        targets: &[wgpu::TextureFormat::Rgba8UnormSrgb.into()],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleStrip,
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                });

        let sampler = graphics.device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let mut encoder = graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("mipmap command encoder"),
            });

        let mut textures = Vec::new();
        // Grass
        textures.push(Self::from_path(&graphics, path.join("grass_side.png"))?);
        textures.push(Self::from_path(&graphics, path.join("grass_bottom.png"))?);
        textures.push(Self::from_path(&graphics, path.join("grass_top.png"))?);
        // Wolf
        textures.push(Self::from_path(&graphics, path.join("wolf.jpg"))?);

        // Generate Mipmaps
        for t in textures.iter() {
            t.generate_mipmaps(&graphics, &mipmap_pipeline, &sampler, &mut encoder);
        }
        graphics.queue.submit(Some(encoder.finish()));

        Ok(textures)
    }

    pub fn create_sampler(graphics: &Graphics) -> wgpu::Sampler {
        graphics.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("texture array sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        })
    }

    fn generate_mipmaps(
        &self,
        graphics: &Graphics,
        pipeline: &wgpu::RenderPipeline,
        sampler: &wgpu::Sampler,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let t_views = (0..self.mip_level_count)
            .map(|mip| {
                self.texture.create_view(&wgpu::TextureViewDescriptor {
                    label: Some(&format!("mip level {}", mip)),
                    format: None,
                    dimension: None,
                    aspect: wgpu::TextureAspect::All,
                    base_mip_level: mip,
                    mip_level_count: NonZeroU32::new(1),
                    base_array_layer: 0,
                    array_layer_count: None,
                })
            })
            .collect::<Vec<_>>();

        for target_mip in 1..self.mip_level_count as usize {
            let bind_group = graphics
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&t_views[target_mip - 1]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &t_views[target_mip],
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
    }
}

pub static mut TEXTURE_INDEX_LIST: Vec<[u32; 6]> = Vec::new();

pub unsafe fn init_index_list() {
    TEXTURE_INDEX_LIST.push([0, 0, 0, 0, 2, 1]);
    TEXTURE_INDEX_LIST.push([0, 0, 0, 0, 0, 0]);
}
