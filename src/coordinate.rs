#[derive(Clone, Copy, Debug)]
pub struct Coord3DI {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord3DI {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn to_chunk_coord(&self) -> ChunkCoord3D {
        let x = self.x >> 5;
        let y = self.y >> 5;
        let z = self.z >> 5;
        ChunkCoord3D { x, y, z }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Coord3DF {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coord3DF {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_chunk_coord(&self) -> ChunkCoord3D {
        let x = self.x.floor() as i32 >> 5;
        let y = self.y.floor() as i32 >> 5;
        let z = self.z.floor() as i32 >> 5;
        ChunkCoord3D { x, y, z }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct ChunkCoord3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoord3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn to_world_position_f32(&self) -> Coord3DF {
        let x = self.x * 32;
        let y = self.y * 32;
        let z = self.z * 32;
        Coord3DF::new(x as f32, y as f32, z as f32)
    }

    pub fn to_world_position_i32(&self) -> Coord3DI {
        let x = self.x * 32;
        let y = self.y * 32;
        let z = self.z * 32;
        Coord3DI::new(x, y, z)
    }
}
