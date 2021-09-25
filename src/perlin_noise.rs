use crate::coordinate::ChunkCoord3D;
use crate::world::{CHUNK_I32, CHUNK_USIZE};
use simdnoise::NoiseBuilder;

pub fn perlin_3d_block_avx2(pos: ChunkCoord3D) -> Vec<f32> {
    let generator = NoiseBuilder::fbm_3d_offset(
        (pos.x * CHUNK_I32) as f32,
        CHUNK_USIZE,
        (pos.z * CHUNK_I32) as f32,
        CHUNK_USIZE,
        (pos.y * CHUNK_I32) as f32,
        CHUNK_USIZE,
    )
    .with_seed(1)
    .with_freq(0.09)
    .with_gain(2.5)
    .with_lacunarity(0.5)
    .with_octaves(3)
    .wrap();
    let (noise, _, _) = unsafe { simdnoise::avx2::get_3d_noise(&generator) };
    noise
}

pub fn perlin_3d_block_sse41(pos: ChunkCoord3D) -> Vec<f32> {
    let generator = NoiseBuilder::fbm_3d_offset(
        (pos.x * CHUNK_I32) as f32,
        CHUNK_USIZE,
        (pos.z * CHUNK_I32) as f32,
        CHUNK_USIZE,
        (pos.y * CHUNK_I32) as f32,
        CHUNK_USIZE,
    )
    .with_seed(1)
    .with_freq(0.09)
    .with_gain(2.5)
    .with_lacunarity(0.5)
    .with_octaves(3)
    .wrap();
    let (noise, _, _) = unsafe { simdnoise::sse41::get_3d_noise(&generator) };
    noise
}

pub fn perlin_2d(pos: ChunkCoord3D) -> Vec<f32> {
    let generator = NoiseBuilder::fbm_2d_offset(
        (pos.x * CHUNK_I32) as f32,
        CHUNK_USIZE,
        (pos.z * CHUNK_I32) as f32,
        CHUNK_USIZE,
    )
    .with_seed(1)
    .with_freq(0.09)
    .with_gain(2.5)
    .with_lacunarity(0.5)
    .with_octaves(3)
    .wrap();
    let (noise, _, _) = unsafe { simdnoise::avx2::get_3d_noise(&generator) };
    noise
}
