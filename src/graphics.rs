pub struct Graphics {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
}

impl Graphics {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface)
        }).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            features: wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING
                | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
                | wgpu::Features::NON_FILL_POLYGON_MODE
                | wgpu::Features::UNSIZED_BINDING_ARRAY,
            limits: wgpu::Limits::default()
        }, None).await.unwrap();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        Self {
            size,
            surface,
            device,
            queue,
            sc_desc,
            swap_chain
        }
    }
}