use crate::camera::Camera;
use crate::chunk_builder::ChunkGenerator;
use crate::coordinate::ChunkCoord3D;
use crate::debug_info::{DebugInfo, DebugInfoBuilder};
use crate::frustum_culling::Frustum;
use crate::player::Player;
use crate::renderer::graphics::Graphics;
use crate::renderer::renderer::Renderer;
use crate::uniform::RenderPassData;
use crate::world::World;

pub struct Engine {
    renderer: Renderer,
    world: World,
    chunk_gen: ChunkGenerator,
    uniforms: RenderPassData,
    camera: Camera,
    player: Player,
    debug_info: DebugInfo,
    tick_time: u32,
    frustum: Frustum,
}

const TICK: u32 = 4;

impl Engine {
    pub fn new(graphics: &Graphics) -> Self {
        let camera = Camera::new(&graphics);
        let uniforms = RenderPassData::new(&graphics, &camera);
        let renderer = Renderer::new(&graphics, &uniforms);
        let world = World::new();
        let chunk_gen = ChunkGenerator::new();
        unsafe { crate::texture::init_index_list() };
        let player = Player::new(&camera);
        let debug_info = DebugInfoBuilder::new(
            10.,
            10.,
            30.,
            graphics.surface_config.format,
            (graphics.size.width, graphics.size.height),
        )
        .build(&graphics)
        .unwrap();
        let frustum = Frustum::new(&camera);
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
        self.player.update_pos(&self.camera);
        unsafe { self.debug_info.update_info() };
        self.frustum.update(&self.camera);

        // Tick system:
        self.tick_time += 1;
        if TICK <= self.tick_time {
            self.world.update(
                &mut self.chunk_gen,
                &mut self.player,
                &pool,
                &graphics,
                &self.frustum,
            );
            self.tick_time = 0;
        }
    }

    pub fn render(&mut self, graphics: &Graphics) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(
            &graphics,
            &self.world,
            &self.uniforms,
            &mut self.debug_info,
            &self.camera,
            &self.frustum,
            &self.chunk_gen,
        )?;
        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, graphics: &mut Graphics) {
        graphics.resize(new_size);
        self.camera.resize(&graphics);
        self.renderer.resize(&graphics);
        self.debug_info.resize(&new_size);
        self.frustum.update(&self.camera);
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent) {
        self.camera.input(event);
    }
}
