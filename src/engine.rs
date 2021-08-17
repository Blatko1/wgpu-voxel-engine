use crate::graphics::Graphics;
use crate::renderer::Renderer;
use crate::world::World;

pub struct Engine {
    renderer: Renderer,
    world: World,
}

impl Engine {
    pub fn new(graphics: &Graphics) -> Self {
        let renderer = Renderer::new(&graphics);
        let mut world = World::new(&graphics);
        world.add_test(0, 0, 0, &graphics);
        Self { renderer, world }
    }

    pub fn render(&self, graphics: &Graphics) -> Result<(), wgpu::SwapChainError> {
        self.renderer.render(&graphics, &[&self.world])?;
        Ok(())
    }

    pub fn resize(&self, size: winit::dpi::PhysicalSize<u32>) {}
}
