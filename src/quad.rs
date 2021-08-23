use nalgebra::{Rotation3, Translation3};
use crate::renderer::instance::InstanceRaw;
use crate::renderer::vertex::Vertex;

pub struct Quad {
    position: Translation3<f32>,
    rotation: Rotation3<f32>,
}

impl Quad {
    pub fn new(pos: [i32; 3], rotation: Rotation3<f32>) -> Self {
        Quad {
            position: Translation3::from([pos[0] as f32, pos[1] as f32, pos[2] as f32]),
            rotation,
        }
    }
    pub fn to_raw(&self) -> InstanceRaw {
        let t_matrix: [[f32; 4]; 4] =
            (self.position.to_homogeneous() * self.rotation.to_homogeneous()).into();

        InstanceRaw { t_matrix }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1., 0., 0.],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0., 1., 0.],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0., 0., 1.],
    },
];

pub const INDICES: &[u32] = &[0, 1, 2];
