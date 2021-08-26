use crate::renderer::instance::InstanceRaw;
use crate::renderer::vertex::Vertex;
use nalgebra::{Rotation3, Translation3};

pub struct Quad {
    pub position: Translation3<f32>,
    rotation: Rotation3<f32>,
    texture_index: u32,
}

impl Quad {
    pub fn new(pos: [f32; 3], rotation: Rotation3<f32>, texture_index: u32) -> Self {
        Quad {
            position: Translation3::from(pos),
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
