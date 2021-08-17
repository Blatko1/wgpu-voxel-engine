use crate::quad::Quad;
use nalgebra::Rotation3;

#[derive(Copy, Clone)]
pub struct Cube {
    pub top_face: bool,
    pub bottom_face: bool,
    pub left_face: bool,
    pub right_face: bool,
    pub back_face: bool,
    pub front_face: bool,
    pub is_active: bool,
}

impl Cube {
    pub fn default() -> Self {
        Self {
            top_face: true,
            bottom_face: true,
            left_face: true,
            right_face: true,
            back_face: true,
            front_face: true,
            is_active: true,
        }
    }

    pub fn new(
        top_face: bool,
        bottom_face: bool,
        left_face: bool,
        right_face: bool,
        back_face: bool,
        front_face: bool,
    ) -> Self {
        Self {
            top_face,
            bottom_face,
            left_face,
            right_face,
            back_face,
            front_face,
            is_active: true,
        }
    }

    pub fn get_faces(&self, position: [f32; 3]) -> Vec<Quad> {
        let mut quads = Vec::new();
        if self.back_face {
            quads.push(Quad::new(
                position,
                Rotation3::new([0., 0., 0.].into()),
            ));
        }
        if self.front_face {
            let pitch: f32 = 180.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(0., pitch.to_radians(), 0.),
            ))
        }
        if self.left_face {
            let pitch: f32 = 90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(0., pitch.to_radians(), 0.),
            ))
        }
        if self.right_face {
            let pitch: f32 = -90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(0., pitch.to_radians(), 0.),
            ))
        }
        if self.top_face {
            let roll: f32 = 90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(roll.to_radians(), 0., 0.),
            ))
        }
        if self.bottom_face {
            let roll: f32 = -90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(roll.to_radians(), 0., 0.),
            ))
        }
        return quads;
    }

}

#[derive(Copy, Clone)]
pub enum CubeType {
    GRASS = 0,
    DIRT = 1,
    STONE = 2,
    WOOD = 3,
}