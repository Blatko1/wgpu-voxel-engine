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
