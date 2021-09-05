use crate::texture;

pub struct Cube {
    pub is_air: bool,
    pub texture_index: [u32; 6],
}

impl Cube {
    pub fn new(is_air: bool, cube_type: CubeType) -> Self {
        let texture_index = unsafe { texture::TEXTURE_INDEX_LIST[cube_type as usize] };
        Self {
            is_air,
            texture_index,
        }
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
