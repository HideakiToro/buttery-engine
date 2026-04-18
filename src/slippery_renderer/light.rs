use super::camera::{Camera, Projection};

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
            direction: [1.0, 0.0, 0.0, 0.0]
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
        let dir = camera.direction();
        self.direction = [dir.x, dir.y, dir.z, 0.0];
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BiasUniform {
    pub bias: [f32; 4],
}