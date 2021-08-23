use crate::graphics::Graphics;
use crate::uniform::MatrixData;
use nalgebra::{Matrix4, Point3, Vector3};
use winit::event::DeviceEvent;

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fov: f32,
    near: f32,
    far: f32,
    controller: CameraController
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
        let controller = CameraController::new();
        Self {
            eye: Point3::new(0., 0., 1.),
            target: Point3::new(0., 0., -1.),
            up: Vector3::y(),
            aspect: graphics.sc_desc.width as f32 / graphics.sc_desc.height as f32,
            fov: 60.,
            near: 0.1,
            far: 100.0,
            controller
        }
    }

    pub fn create_global_matrix(&self) -> MatrixData {
        let projection = Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far);
        let view = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);
        let result: [[f32; 4]; 4] = (OPENGL_TO_WGPU_MATRIX * projection * view).into();
        let data = MatrixData {
            proj_view_model_matrix: result,
        };
        println!("{:?}", data);
        return data;
    }

    pub fn resize(&mut self, graphics: &Graphics) {
        self.aspect = graphics.sc_desc.width as f32 / graphics.sc_desc.height as f32;
    }

    pub fn update(&mut self) {
        self.controller.update_camera(self);
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent) {
        self.controller.process_input(event);
    }
}

struct CameraController {
    speed: f32,
    sensitivity: f64,
    forward: f32,
    backward: f32,
    left: f32,
    right: f32,
    up: f32,
    down: f32,
    yaw: f32,
    pitch: f32,
}

impl CameraController {
    pub fn new() -> Self {
        CameraController {
            speed: 0.03,
            sensitivity: 0.1,
            forward: 0.,
            backward: 0.,
            left: 0.,
            right: 0.,
            up: 0.,
            down: 0.,
            yaw: 270.0,
            pitch: 0.0,
        }
    }

    pub fn process_input(&mut self, event: &winit::event::DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.yaw = (delta.0 * self.sensitivity) as f32;
                self.pitch = (delta.1 * self.sensitivity) as f32;

                if self.pitch > 89.0 {
                    self.pitch = 89.0;
                } else if self.pitch < -89.0 {
                    self.pitch = -89.0;
                }

                if self.yaw > 360.0 {
                    self.yaw = 0.0;
                } else if self.yaw < 0.0 {
                    self.yaw = 360.0;
                }

            }
            DeviceEvent::MouseWheel { .. } => {


            }
            DeviceEvent::Motion { .. } => {


            }
            DeviceEvent::Button { .. } => {


            }
            DeviceEvent::Key(_) => {


            }
            _ => ()
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        if self.yaw > 360.0 {
            self.yaw = 0.0;
        } else if self.yaw < 0.0 {
            self.yaw = 360.0;
        }
        camera.target = Point3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        let target = Vector3::new(camera.target.x, 0.0, camera.target.z).normalize();
        camera.eye += &target * self.speed * (self.forward - self.backward);
        camera.eye += &target.cross(&camera.up) * self.speed * (self.right - self.left);
        camera.eye += Vector3::new(0.0, 1.0, 0.0) * self.speed * (self.up - self.down);
    }
}