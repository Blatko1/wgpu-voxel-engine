use crate::coordinate::Coord3D;
use crate::renderer::instance::InstanceRaw;
use crate::renderer::vertex::Vertex;
use nalgebra::{Rotation3, Translation3};

#[derive(Debug)]
pub struct Quad {
    pub position: Translation3<f32>,
    rotation: Rotation3<f32>,
    texture_index: u32,
}

impl Quad {
    pub fn new(pos: Coord3D, facing: Rotation, texture_index: u32) -> Self {
        let rotation: Rotation3<f32>;
        match facing {
            Rotation::UP => {
                let rot: f32 = -90.;
                rotation = Rotation3::from_euler_angles(rot.to_radians(), 0., 0.);
            }
            Rotation::DOWN => {
                let rot: f32 = 90.;
                rotation = Rotation3::from_euler_angles(rot.to_radians(), 0., 0.);
            }
            Rotation::LEFT => {
                let rot: f32 = -90.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
            Rotation::RIGHT => {
                let rot: f32 = 90.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
            Rotation::FRONT => {
                rotation = Rotation3::from_euler_angles(0., 0., 0.);
            }
            Rotation::BACK => {
                let rot: f32 = 180.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
        }
        Quad {
            position: Translation3::from([pos.x as f32, pos.y as f32, pos.z as f32]),
            rotation,
            texture_index,
        }
    }
    pub fn to_raw(&self) -> InstanceRaw {
        let t_matrix: [[f32; 4]; 4] =
            (self.position.to_homogeneous() * self.rotation.matrix().to_homogeneous()).into();

        InstanceRaw {
            t_matrix,
            texture_index: self.texture_index,
        }
    }
}

pub enum Rotation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}

pub const VERTICES: &[Vertex] = &[
    //tr
    Vertex {
        position: [-0.5, 0.5, 0.5],
        tex_coords: [0., 0.],
    },
    //tl
    Vertex {
        position: [0.5, 0.5, 0.5],
        tex_coords: [1., 0.],
    },
    //br
    Vertex {
        position: [-0.5, -0.5, 0.5],
        tex_coords: [0., 1.],
    },
    //bl
    Vertex {
        position: [0.5, -0.5, 0.5],
        tex_coords: [1., 1.],
    },
];

pub const INDICES: &[u32] = &[0, 2, 1, 3, 1, 2];
