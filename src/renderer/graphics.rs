use futures::executor::block_on;
use std::sync::Arc;

pub struct Graphics {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl Graphics {
    pub fn new(window: &winit::window::Window) -> Self {
        let backend = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY);
        let instance = wgpu::Instance::new(backend);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(window);
            (size, surface)
        };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let adapter_info = adapter.get_info();
        println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                features:
                    wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                        | wgpu::Features::TEXTURE_BINDING_ARRAY
                        | wgpu::Features::NON_FILL_POLYGON_MODE
                        | wgpu::Features::UNSIZED_BINDING_ARRAY
                        | wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
                limits: adapter.limits(),
            },
            None,
        ))
        .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);
        let device_ref = Arc::new(device);
        Self {
            size,
            surface,
            device: device_ref,
            queue,
            surface_config,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.surface_config.width = self.size.width.max(1);
        self.surface_config.height = self.size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
}
