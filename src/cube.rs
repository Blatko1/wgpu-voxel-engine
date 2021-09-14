#[derive(Debug)]
pub struct Cube {
    pub cube_type: CubeType,
}

impl Cube {
    pub fn new(cube_type: CubeType) -> Self {
        Self { cube_type }
    }

    pub fn set_type(&mut self, cube_type: CubeType) {
        self.cube_type = cube_type;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CubeType {
    GRASS = 0,
    DIRT = 1,
    AIR,
}
