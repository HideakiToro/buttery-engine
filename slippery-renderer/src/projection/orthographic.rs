use crate::projection::Projection;
use cgmath::{Matrix4, ortho};

pub struct OthrographicProjection {
    width: f32,
    height: f32,
    near: f32,
    far: f32,
}

impl OthrographicProjection {
    pub fn new(width: f32, height: f32, near: f32, far: f32) -> Self {
        Self {
            width,
            height,
            near,
            far,
        }
    }
}

impl Projection for OthrographicProjection {
    fn calc_matrix(&self) -> Matrix4<f32> {
        let half_w = self.width * 0.5;
        let half_h = self.height * 0.5;

        ortho(-half_w, half_w, -half_h, half_h, self.near, self.far)
    }
}
