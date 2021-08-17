use crate::vertex::Vertex;
use crate::instance::InstanceRaw;
use nalgebra::{Translation3, Rotation3};

pub struct Quad {
    position: Translation3<f32>,
    rotation: Rotation3<f32>
}

impl Quad {
    pub fn new(position: [i32; 3], rotation: Rotation3<f32>) -> Self {
        Quad {
            position: Translation3::from(position as [f32; 3]),
            rotation
        }
    }
    pub fn to_raw(&self) -> InstanceRaw{
        let t_matrix: [[f32; 4]; 4] = (self.position.to_homogeneous() * self.rotation.to_homogeneous()).into();

        InstanceRaw {
            t_matrix
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1., -1., 1.]
    },
    Vertex {
        position: [1., -1., 1.]
    },
    Vertex {
        position: [-1., 1., 1.]
    },
    Vertex {
        position: [1., 1., 1.]
    },
];

pub const INDICES: &[u32] = &[
    0, 1, 2, 2, 1, 3
];