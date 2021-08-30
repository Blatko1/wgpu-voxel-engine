use crate::camera::Camera;
use crate::coordinate::Coord3D;
use crate::renderer::graphics::Graphics;
use crate::renderer::renderer::Renderer;
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
        let mut world = World::new(&graphics, &camera);
        unsafe { crate::texture::init_index_list() };
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
        self.renderer.resize(&graphics);
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent) {
        self.camera.input(event);
    }

    pub fn new_perlin(&mut self) {
        self.world.update(&self.camera);
    }
}
