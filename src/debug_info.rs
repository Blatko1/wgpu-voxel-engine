use crate::camera::Camera;
use crate::chunk_builder::ChunkGenerator;
use crate::coordinate::Coord3DF;
use crate::renderer::graphics::Graphics;
use crate::world::World;
use futures::task::SpawnExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};

pub struct DebugInfoBuilder {
    position: (f32, f32),
    scale: f32,
    render_format: wgpu::TextureFormat,
    screen_bounds: (u32, u32),
}

impl DebugInfoBuilder {
    pub fn new(
        x: f32,
        y: f32,
        scale: f32,
        render_format: wgpu::TextureFormat,
        screen_bounds: (u32, u32),
    ) -> Self {
        Self {
            position: (x, y),
            scale,
            render_format,
            screen_bounds,
        }
    }

    pub fn build(&self, graphics: &Graphics) -> Result<DebugInfo, Box<dyn std::error::Error>> {
        let font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("../res/fonts/FiraCode-Medium.ttf"))?;
        let brush = GlyphBrushBuilder::using_font(font).build(&graphics.device, self.render_format);

        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();

        let info = DebugInfo {
            position: self.position,
            scale: self.scale,
            screen_bounds: self.screen_bounds,
            brush,
            text: vec![
                DebugTools::FPS,
                DebugTools::Position,
                DebugTools::Chunk,
                DebugTools::LoadedChunks,
                DebugTools::ChunksInQueue,
            ],
            fps: 0.,
            staging_belt,
            local_pool,
            local_spawner,
        };
        Ok(info)
    }
}

static mut TIME: Duration = Duration::from_millis(0);
static mut FPS_SHOW_TIME: Duration = Duration::from_millis(0);

pub struct DebugInfo {
    position: (f32, f32),
    scale: f32,
    screen_bounds: (u32, u32),
    brush: wgpu_glyph::GlyphBrush<()>,
    text: Vec<DebugTools>,
    fps: f64,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    local_spawner: futures::executor::LocalSpawner,
}

impl DebugInfo {
    pub fn draw<'a>(
        &mut self,
        graphics: &Graphics,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        camera: &Camera,
        world: &World,
        chunk_gen: &ChunkGenerator,
    ) -> Result<(), String> {
        let fps = String::from(format!("FPS: {:.2}\n", self.fps as u32));
        let pos = String::from(format!(
            "Position: x: {:.2}, y: {:.2}, z: {:.2}\n",
            camera.eye.x, camera.eye.y, camera.eye.z
        ));
        let coords = Coord3DF::new(camera.eye.x, camera.eye.y, camera.eye.z).to_chunk_coord();
        let chunk = String::from(format!(
            "Chunk: x: {}, y: {}, z: {}\n",
            coords.x, coords.y, coords.z
        ));
        let chunks_loaded_num = String::from(format!("Chunks loaded: {}\n", world.chunks.len()));
        let chunk_queue = String::from(format!(
            "Chunks loaded: {}\n",
            chunk_gen.chunk_load_queue.len()
        ));
        let mut debug_text: Vec<Text> = Vec::new();
        for t in self.text.iter() {
            match t {
                DebugTools::FPS => {
                    debug_text.push(
                        Text::new(&fps)
                            .with_color([1., 1., 1., 1.])
                            .with_scale(self.scale),
                    );
                }
                DebugTools::Position => {
                    debug_text.push(
                        Text::new(&pos)
                            .with_color([1., 1., 1., 1.])
                            .with_scale(self.scale),
                    );
                }
                DebugTools::Chunk => {
                    debug_text.push(
                        Text::new(&chunk)
                            .with_color([1., 1., 1., 1.])
                            .with_scale(self.scale),
                    );
                }
                DebugTools::LoadedChunks => {
                    debug_text.push(
                        Text::new(&chunks_loaded_num)
                            .with_color([1., 1., 1., 1.])
                            .with_scale(self.scale),
                    );
                }
                DebugTools::ChunksInQueue => {
                    debug_text.push(
                        Text::new(&chunk_queue)
                            .with_color([1., 1., 1., 1.])
                            .with_scale(self.scale),
                    );
                }
            }
        }
        self.brush.queue(Section {
            screen_position: (self.position.0, self.position.1),
            bounds: (self.screen_bounds.0 as f32, self.screen_bounds.1 as f32),
            layout: Default::default(),
            text: debug_text,
        });
        self.brush.draw_queued(
            &graphics.device,
            &mut self.staging_belt,
            encoder,
            &target,
            self.screen_bounds.0,
            self.screen_bounds.1,
        )
    }

    pub fn finish(&mut self) {
        self.staging_belt.finish();
    }

    pub unsafe fn update_info(&mut self) {
        // Recall unused staging buffers
        self.local_spawner
            .spawn(self.staging_belt.recall())
            .expect("Recall staging belt");
        self.local_pool.run_until_stalled();
        let now = SystemTime::now();
        let time = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        if (time.as_millis() - FPS_SHOW_TIME.as_millis()) > 1000 {
            let delta = time.as_micros() - TIME.as_micros();
            self.fps = 1. / (delta as f64 / 1000000.);
            FPS_SHOW_TIME = time;
        }
        TIME = time;
    }

    pub fn resize(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        self.screen_bounds = (size.width, size.height);
    }
}

pub enum DebugTools {
    FPS,
    Position,
    Chunk,
    LoadedChunks,
    ChunksInQueue,
}
