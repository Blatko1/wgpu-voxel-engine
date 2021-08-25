use crate::renderer::instance::InstanceRaw;
use crate::renderer::vertex::Vertex;
use nalgebra::{Rotation3, Translation3};

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
    //tr
    Vertex {
        position: [-0.5, 0.5, -0.5],
        color: [1., 0., 0.],
        tex_coords: [1., 0.]
    },
    //tl
    Vertex {
        position: [0.5, 0.5, -0.5],
        color: [0., 1., 0.],
        tex_coords: [0., 0.]
    },
    //br
    Vertex {
        position: [-0.5, -0.5, -0.5],
        color: [0., 0., 1.],
        tex_coords: [1., 1.]
    },
    //bl
    Vertex {
        position: [0.5, -0.5, -0.5],
        color: [0., 0., 1.],
        tex_coords: [0., 1.]
    },
];

pub const INDICES: &[u32] = &[0, 1, 2, 1, 3, 2];
