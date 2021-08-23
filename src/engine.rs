use crate::camera::Camera;
use crate::coordinate::Coord3D;
use crate::graphics::Graphics;
use crate::renderer::Renderer;
use crate::uniform::UniformManager;
use crate::world::World;

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
        let mut world = World::new(&graphics);
        world.add_quad(&graphics);
        Self {
            renderer,
            world,
            uniforms,
            camera,
        }
    }

    pub fn update(&mut self) {
        self.uniforms.update(&self.camera);
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
