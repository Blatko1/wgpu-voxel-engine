use crate::graphics::Graphics;
use crate::uniform::MatrixData;
use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fov: f32,
    near: f32,
    far: f32,
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            eye: Point3::new(0., 0., 1.),
            target: Point3::new(0., 0., -1.),
            up: Vector3::y(),
            aspect: graphics.sc_desc.width as f32 / graphics.sc_desc.height as f32,
            fov: 60.,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn create_global_matrix(&self) -> MatrixData {
        let projection = Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far);
        let result: [[f32; 4]; 4] = (OPENGL_TO_WGPU_MATRIX * projection).into();
        let data = MatrixData {
            proj_view_model_matrix: result,
        };
        println!("{:?}", data);
        return data;
    }

    pub fn resize(&mut self, graphics: &Graphics) {
        self.aspect = graphics.sc_desc.width as f32 / graphics.sc_desc.height as f32;
    }
}
