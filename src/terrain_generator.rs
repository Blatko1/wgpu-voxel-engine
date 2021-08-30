use bracket_noise::prelude::*;
use core::arch::x86_64::_mm256_set1_ps;
use simdnoise::*;

pub struct TerrainGenerator {
    noise: FastNoise,
    seed: u64,
}

impl TerrainGenerator {
    pub fn new(seed: u64) -> Self {
        let mut noise = FastNoise::new();
        noise.set_seed(seed);
        noise.set_frequency(0.05);
        Self { noise, seed }
    }
    pub fn perlin_3d(&self, x: i32, y: i32, z: i32) -> f32 {
        self.noise.get_noise3d(x as f32, y as f32, z as f32)
    }
}

/*pub struct TerrainGeneratorSimd {
    seed: u64,
}

impl TerrainGeneratorSimd {
    pub fn new(seed: u64) -> Self {
        Self {
            seed
        }
    }
    pub unsafe fn perlin_3d(&self, x: f32, y: f32, z: f32) -> f32 {
        let x = _mm256_set1_ps(x);
        let y = _mm256_set1_ps(y);
        let z = _mm256_set1_ps(z);
        let freq = _mm256_set1_ps(1.0);
        let lacunarity = _mm256_set1_ps(0.5);
        let gain = _mm256_set1_ps(2.0);
        let octaves = 3;
        simdnoise::avx2::fbm_3d(x, y, z, lacunarity, gain, octaves, 1) as f32
    }
}*/

/*fn perlin_3d(offset: f32, x, y, z:) -> Vec<f32> {
    let generator = NoiseBuilder::fbm_3d_offset(
        (pos.x * 16) as f32,
        CHUNK_WIDTH,
        (pos.z * 16) as f32,
        CHUNK_HEIGHT,
        (pos.y * 16) as f32 + offset,
        CHUNK_LENGTH,
    )
    .with_seed(1).with_freq(0.05).wrap();
    let (noise, _, _) = unsafe { simdnoise::sse41::get_3d_noise(&generator) };
    noise
}*/
