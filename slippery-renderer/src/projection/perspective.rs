use crate::projection::{OPENGL_TO_WGPU_MATRIX, Projection};
use cgmath::{Matrix4, Rad, perspective};

pub struct PerspectiveProjection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl PerspectiveProjection {
    pub fn new<F: Into<Rad<f32>>>(width: f32, height: f32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width / height,
            fovy: fovy.into(),
            znear,
            zfar: zfar,
        }
    }
}

impl Projection for PerspectiveProjection {
    fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}
