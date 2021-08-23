use crate::graphics::Graphics;

pub struct Texture {

}

impl Texture {

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture_view(graphics: &Graphics) -> wgpu::TextureView {
        let size = {
            wgpu::Extent3d {
                width: graphics.sc_desc.width,
                height: graphics.sc_desc.height,
                depth_or_array_layers: 1
            }
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED
        };
        let texture = graphics.device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        view
    }
}