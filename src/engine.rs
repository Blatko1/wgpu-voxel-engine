use crate::camera::Camera;
use crate::uniform::UniformManager;
use crate::world::World;
use crate::renderer::renderer::Renderer;
use crate::renderer::graphics::Graphics;

pub struct Engine {
    renderer: Renderer,
    world: World,
    uniforms: UniformManager,
    camera: Camera,
}

impl Engine {
    pub fn new(graphics: &Graphics) -> Self {
        let camera = Camera::new(&graphics);
        let uniforms = UniformManager::new(&graphics, &camera);
        let renderer = Renderer::new(&graphics, &uniforms);
        let mut world = World::new();
        world.add_quad(&graphics);
        Self {
            renderer,
            world,
            uniforms,
            camera,
        }
    }

    pub fn update(&mut self, graphics: &Graphics) {
        self.camera.update();
        self.uniforms.update(&self.camera, &graphics);
    }

    pub fn render(&self, graphics: &Graphics) -> Result<(), wgpu::SwapChainError> {
        self.renderer
            .render(&graphics, &self.world, &self.uniforms)?;
        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, graphics: &mut Graphics) {
        graphics.resize(new_size);
        self.camera.resize(&graphics);
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent) {
        self.camera.input(event);
    }
}
