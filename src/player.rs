use crate::camera::Camera;
use crate::coordinate::{ChunkCoord3D, Coord3D};

pub struct Player {
    pos: Coord3D,
    pub chunk: ChunkCoord3D,
}

impl Player {
    pub fn new(camera: &Camera) -> Self {
        let pos = Coord3D::new(
            camera.eye.x as i32,
            camera.eye.y as i32,
            camera.eye.z as i32,
        );
        let chunk = pos.to_chunk_coord();
        Self { pos, chunk }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.pos = Coord3D::new(
            camera.eye.x as i32,
            camera.eye.y as i32,
            camera.eye.z as i32,
        );
    }

    pub fn new_chunk_pos(&mut self) -> bool {
        let result = self.chunk != self.pos.to_chunk_coord();

        self.chunk = self.pos.to_chunk_coord();
        result
    }
}
