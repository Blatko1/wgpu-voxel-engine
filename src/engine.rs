use crate::camera::Camera;
use crate::chunk_loader::ChunkGenerator;
use crate::coordinate::{ChunkCoord3D, Coord3DI};
use crate::debug_info::{DebugInfo, DebugInfoBuilder};
use crate::frustum_culling::Frustum;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::renderer::renderer::Renderer;
use crate::uniform::UniformManager;
use crate::world::World;

pub struct Engine {
    renderer: Renderer,
    world: World,
    chunk_gen: ChunkGenerator,
    uniforms: UniformManager,
    camera: Camera,
    player: Player,
    debug_info: DebugInfo,
    tick_time: u32,
    frustum: Frustum,
}

const TICK: u32 = 3;

impl Engine {
    pub fn new(graphics: &Graphics) -> Self {
        let camera = Camera::new(&graphics);
        let uniforms = UniformManager::new(&graphics, &camera);
        let renderer = Renderer::new(&graphics, &uniforms);
        let world = World::new();
        let chunk_gen = ChunkGenerator::new();
        unsafe { crate::texture::init_index_list() };
        let player = Player::new(&camera);
        let debug_info = DebugInfoBuilder::new(
            10.,
            10.,
            40.,
            graphics.surface_config.format,
            (graphics.size.width, graphics.size.height),
        )
        .build(&graphics)
        .unwrap();
        let mut frustum = Frustum::new(&camera);
        Self {
            renderer,
            world,
            chunk_gen,
            uniforms,
            camera,
            player,
            debug_info,
            tick_time: 0,
            frustum,
        }
    }

    pub fn update(&mut self, graphics: &Graphics, pool: &uvth::ThreadPool) {
        self.camera.update();
        self.uniforms.update(&self.camera, &graphics);
        self.player.update(&self.camera);
        unsafe { self.debug_info.update_info() };

        // Tick system:
        self.tick_time += 1;
        if TICK <= self.tick_time {
            self.world
                .update(&self.chunk_gen, &mut self.player, &pool, &graphics);
            self.tick_time = 0;
        }
        self.frustum.update(&self.camera);
    }

    pub fn update_frustum(&mut self) {
        //self.frustum.update(&self.camera);            // Remove comment sign from this line to test out frustum culling.
                                                        //
    }

    pub fn render(&mut self, graphics: &Graphics) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(
            &graphics,
            &self.world,
            &self.uniforms,
            &mut self.debug_info,
            &self.camera,
            &self.frustum,
        )?;
        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, graphics: &mut Graphics) {
        graphics.resize(new_size);
        self.camera.resize(&graphics);
        self.renderer.resize(&graphics);
        self.debug_info.resize(&new_size);
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent) {
        self.camera.input(event);
    }
}
