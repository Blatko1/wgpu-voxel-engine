use crate::coordinate::ChunkCoord3D;
use bracket_noise::prelude::*;
use simdnoise::NoiseBuilder;

pub struct PerlinGenerator {
    noise: FastNoise,
}

impl PerlinGenerator {
    pub fn new(seed: u64) -> Self {
        let mut noise = FastNoise::new();
        noise.set_seed(seed);
        noise.set_frequency(0.05);
        Self { noise }
    }
    pub fn perlin_3d(&self, x: i32, y: i32, z: i32) -> f32 {
        self.noise.get_noise3d(x as f32, y as f32, z as f32)
    }
}

pub fn perlin_3d(pos: ChunkCoord3D) -> Vec<f32> {
    let generator = NoiseBuilder::fbm_3d_offset(
        (pos.x * 32) as f32,
        32,
        (pos.z * 32) as f32,
        32,
        (pos.y * 32) as f32,
        32,
    )
    .with_seed(1)
    .with_freq(0.05)
    .wrap();
    let (noise, _, _) = unsafe { simdnoise::sse41::get_3d_noise(&generator) };
    noise
}
