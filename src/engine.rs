use crate::graphics::Graphics;
use crate::renderer::Renderer;
use crate::world::World;
use crate::coordinate::Coord3D;
use crate::uniform::UniformManager;
use crate::camera::Camera;

pub struct Engine {
    renderer: Renderer,
    world: World,
    uniforms: UniformManager
}

impl Engine {
    pub fn new(graphics: &Graphics) -> Self {
        let camera = Camera::new(&graphics);
        let uniforms = UniformManager::new(&graphics, &camera);
        let renderer = Renderer::new(&graphics, &uniforms);
        let mut world = World::new(&graphics);
        world.add_region(Coord3D::new(0, 2,0));
        Self { renderer, world, uniforms }
    }

    pub fn update(&self) {

    }

    pub fn render(&self, graphics: &Graphics) -> Result<(), wgpu::SwapChainError> {
        self.renderer.render(&graphics, &[&self.world])?;
        Ok(())
    }

    pub fn resize(&self, size: winit::dpi::PhysicalSize<u32>) {}
}
