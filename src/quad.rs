use crate::vertex::Vertex;

pub struct Quad {

}

impl Quad {
    pub fn to_raw(&self) -> {

    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1., -1., -1.]
    },
    Vertex {
        position: [1., -1., -1.]
    },
    Vertex {
        position: [-1., 1., -1.]
    },
    Vertex {
        position: [1., 1., -1.]
    },
];

pub const INDICES: &[u32] = &[
    0, 1, 2, 2, 1, 3
];