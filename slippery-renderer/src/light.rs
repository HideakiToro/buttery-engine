use crate::projection::{Projection, orthographic::OthrographicProjection};
use buttery_engine::light::{Light, LightType};
use cgmath::{InnerSpace, Matrix4, Vector3};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub view_proj: [[f32; 4]; 4],
    /// 4th parameter is only for padding. Just ignore it in shader and code.
    pub view_position: [f32; 4],
    /// 4th parameter is only for padding. Just ignore it in shader and code. Light shouldn't have alpha?
    pub color: [f32; 4],
    /// 4th parameter is only for padding. Just ignore it in shader and code.
    pub direction: [f32; 4],
}

impl LightUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            view_position: [0.0; 4],
            color: [1.0; 4],
            direction: [1.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn update_view_proj(&mut self, light: &Light) {
        match light.l_type {
            LightType::Directional(_direction) => {
                let dir = light.direction();
                let projection =
                    OthrographicProjection::new(200.0, 200.0, 0.1, light.render_distance);
                // projection is used for direction while camera is used for position
                self.view_proj = (projection.calc_matrix() * light.calc_matrix()).into();
                self.direction = [dir.x, dir.y, dir.z, 0.0];
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BiasUniform {
    pub bias: [f32; 4],
}

pub trait SlipperyLight {
    fn calc_matrix(&self) -> Matrix4<f32>;

    fn direction(&self) -> Vector3<f32>;
}

impl SlipperyLight for Light {
    fn calc_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.direction(), Vector3::unit_y())
    }

    fn direction(&self) -> Vector3<f32> {
        match self.l_type {
            LightType::Directional(direction) => {
                let (sin_pitch, cos_pitch) = direction.pitch.0.sin_cos();
                let (sin_yaw, cos_yaw) = direction.yaw.0.sin_cos();
                Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
            }
        }
    }
}
