use buttery_engine::{camera::Camera, projection::ProjectionType};
use cgmath::{Deg, InnerSpace, Matrix4, Vector3};

use crate::projection::{
    Projection, orthographic::OthrographicProjection, perspective::PerspectiveProjection,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub view_proj: [[f32; 4]; 4],
    pub view_position: [f32; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            view_position: [0.0; 4],
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, width: f32, height: f32) {
        self.view_position = camera.position.to_homogeneous().into();
        let projection = match camera.projection {
            ProjectionType::Perspective => Box::new(PerspectiveProjection::new(
                width,
                height,
                Deg(camera.fov),
                0.1,
                camera.render_distance,
            )) as Box<dyn Projection>,
            ProjectionType::Orthographic => Box::new(OthrographicProjection::new(
                width,
                height,
                0.1,
                camera.render_distance,
            )),
        };
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

pub trait SlipperyCamera {
    fn calc_matrix(&self) -> Matrix4<f32>;

    fn direction(&self) -> Vector3<f32>;
}

impl SlipperyCamera for Camera {
    fn calc_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.direction(), Vector3::unit_y())
    }

    fn direction(&self) -> Vector3<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }
}
