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

    pub fn to_cube_index(&self) -> usize {
        (self.x + self.z * 32 + self.y * 32 * 32) as usize
    }

    pub fn to_chunk_coord(&self) -> ChunkCoord3D {
        let x = self.x / 32;
        let y = self.y / 32;
        let z = self.z / 32;
        Coord3D { x, y, z }
    }
}

pub type ChunkCoord3D = Coord3D;
