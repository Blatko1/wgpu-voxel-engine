use crate::camera::Camera;
use crate::coordinate::{ChunkCoord3D, Coord3DF};

pub struct Player {
    pos: Coord3DF,
    pub chunk: ChunkCoord3D,
}

impl Player {
    pub fn new(camera: &Camera) -> Self {
        let pos = Coord3DF::new(camera.eye.x, camera.eye.y, camera.eye.z);
        let chunk = pos.to_chunk_coord();
        Self { pos, chunk }
    }

    pub fn update_pos(&mut self, camera: &Camera) {
        self.pos = Coord3DF::new(camera.eye.x, camera.eye.y, camera.eye.z);
    }

    pub fn update_chunk_pos(&mut self) {
        self.chunk = self.pos.to_chunk_coord();
    }

    pub fn is_in_new_chunk_pos(&self) -> bool {
        let result = self.chunk != self.pos.to_chunk_coord();
        result
    }
}
