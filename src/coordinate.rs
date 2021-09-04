#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Coord3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn to_chunk_coord(&self) -> ChunkCoord3D {
        let x = self.x >> 5;
        let y = self.y >> 5;
        let z = self.z >> 5;
        Coord3D { x, y, z }
    }
}

pub type ChunkCoord3D = Coord3D;

impl ChunkCoord3D {
    pub fn to_world_position(&self) -> Coord3D {
        let x = self.x * 32;
        let y = self.y * 32;
        let z = self.z * 32;
        Coord3D::new(x, y, z)
    }
}

pub struct Coord3DF {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coord3DF {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
