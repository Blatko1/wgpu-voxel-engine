use crate::renderer::graphics::Graphics;
use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::num::NonZeroU32;
use std::path::Path;

pub struct Texture {
    texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Texture {
    pub const TEXTURE_ARRAY_SIZE: u32 = 1;

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture_view(graphics: &Graphics) -> wgpu::TextureView {
        let size = {
            wgpu::Extent3d {
                width: graphics.sc_desc.width,
                height: graphics.sc_desc.height,
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
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
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

        let texture = graphics.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::RENDER_ATTACHMENT
                | wgpu::TextureUsage::COPY_DST,
        });

        graphics.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
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

        Ok(Self { texture, view })
    }

    fn from_path<P: AsRef<Path>>(graphics: &Graphics, path: P) -> Result<Texture> {
        let buf = path.as_ref().to_path_buf();
        let label = buf.to_str();
        let img = image::open(path).expect("Couldn't find an image from path.");

        Self::from_image(graphics, label, img)
    }

    pub fn load_textures(graphics: &Graphics) -> Result<Vec<Texture>> {
        let path = std::path::Path::new(std::env::current_dir().unwrap().as_os_str()).join("res");

        let mut textures = Vec::new();
        textures.push(Self::from_path(&graphics, path.join("wolf.jpg"))?);

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
}
