# Wgpu Voxel Engine

A voxel engine made in Rust with [wgpu-rs](https://github.com/gfx-rs/wgpu) framework.

Loads chunks super fast with almost no FPS drops! 
> After further inspection I think the multithreading is wrong which could boost the performance.

Primary goal of this project is to make *procedural terrain generation* similar to Minecraft's terrain generation.

**TODOs**:
>~~- Add infinite chunk generation with multithreading optimisations~~
> 
>~~- Add noise terrain generation~~
> 
>~~- Add frustum culling~~
>
>~~- Add debug menu~~
- Add lightning
- Divide chunks into bigger regions
- Add some physics

Old Project: **[wgpu-beginner-project](https://github.com/Blatko1/wgpu-beginner-project)**
