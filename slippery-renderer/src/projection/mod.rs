pub mod orthographic;
pub mod perspective;

use cgmath::{Matrix4, Vector4};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::from_cols(
    Vector4::new(1.0, 0.0, 0.0, 0.0),
    Vector4::new(0.0, 1.0, 0.0, 0.0),
    Vector4::new(0.0, 0.0, 0.5, 0.0),
    Vector4::new(0.0, 0.0, 0.5, 1.0),
);

pub trait Projection {
    fn calc_matrix(&self) -> Matrix4<f32>;
}
