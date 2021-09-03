use crate::coordinate::ChunkCoord3D;
use simdnoise::NoiseBuilder;

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
    let (noise, _, _) = unsafe { simdnoise::avx2::get_3d_noise(&generator) };
    noise
}
