use crate::coordinate::Coord3D;

pub struct Cube {
    pub position: Coord3D,
    pub is_air: bool,
}

impl Cube {
    pub fn new(position: Coord3D, is_air: bool) -> Self {
        Self { position, is_air }
    }

    pub fn set_air(&mut self, air: bool) {
        self.is_air = air;
    }
}

#[derive(Copy, Clone)]
pub enum CubeType {
    GRASS = 0,
    DIRT = 1,
}
