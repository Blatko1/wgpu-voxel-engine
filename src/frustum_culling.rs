use crate::camera::Camera;
use crate::coordinate::{ChunkCoord3D, Coord3DF};
use nalgebra::Matrix4;

pub struct Frustum {
    planes: Vec<Plane>,
}

impl Frustum {
    pub fn new(camera: &Camera) -> Self {
        let mat = camera.global_matrix;
        let planes = Frustum::matrix_to_planes(mat);
        Self { planes }
    }

    pub fn contains(&self, pos: &ChunkCoord3D) -> bool {
        let p1 = pos.to_world_position_f32();
        let mut edges = Vec::new();
        edges.push(p1);
        edges.push(Coord3DF::new(p1.x + 32., p1.y, p1.z));
        edges.push(Coord3DF::new(p1.x, p1.y, p1.z + 32.));
        edges.push(Coord3DF::new(p1.x + 32., p1.y, p1.z + 32.));
        edges.push(Coord3DF::new(p1.x, p1.y + 32., p1.z));
        edges.push(Coord3DF::new(p1.x + 32., p1.y + 32., p1.z));
        edges.push(Coord3DF::new(p1.x, p1.y + 32., p1.z + 32.));
        edges.push(Coord3DF::new(p1.x + 32., p1.y + 32., p1.z + 32.));
        'outer: for p in 0..edges.len() {
            for i in 0..self.planes.len() {
                let dist = self.planes[i].a * edges[p].x as f32
                    + self.planes[i].b * edges[p].y as f32
                    + self.planes[i].c * edges[p].z as f32
                    + self.planes[i].d;
                if dist < 0. {
                    continue 'outer;
                }
            }
            return true;
        }
        return false;
    }

    pub fn update(&mut self, camera: &Camera) {
        let mat = camera.global_matrix;
        self.planes = Frustum::matrix_to_planes(mat);
    }

    fn matrix_to_planes(matrix: Matrix4<f32>) -> Vec<Plane> {
        let mut left = Plane::new();
        let mut right = Plane::new();
        let mut top = Plane::new();
        let mut bottom = Plane::new();
        let mut near = Plane::new();
        let mut far = Plane::new();
        let data = matrix.data.0;
        left.a = data[0][3] + data[0][0];
        left.b = data[1][3] + data[1][0];
        left.c = data[2][3] + data[2][0];
        left.d = data[3][3] + data[3][0];

        right.a = data[0][3] - data[0][0];
        right.b = data[1][3] - data[1][0];
        right.c = data[2][3] - data[2][0];
        right.d = data[3][3] - data[3][0];

        top.a = data[0][3] - data[0][1];
        top.b = data[1][3] - data[1][1];
        top.c = data[2][3] - data[2][1];
        top.d = data[3][3] - data[3][1];

        bottom.a = data[0][3] + data[0][1];
        bottom.b = data[1][3] + data[1][1];
        bottom.c = data[2][3] + data[2][1];
        bottom.d = data[3][3] + data[3][1];

        near.a = data[0][3] + data[0][2];
        near.b = data[1][3] + data[1][2];
        near.c = data[2][3] + data[2][2];
        near.d = data[3][3] + data[3][2];

        far.a = data[0][3] - data[0][2];
        far.b = data[1][3] - data[1][2];
        far.c = data[2][3] - data[2][2];
        far.d = data[3][3] - data[3][2];
        vec![near, far, left, right, top, bottom]
    }
}

#[derive(Debug)]
struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
        }
    }
}
