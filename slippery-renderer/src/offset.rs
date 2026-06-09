use cgmath::Matrix4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelTransform {
    pub offset: [f32; 4],
    pub rotation: [[f32; 4]; 4],
}

impl ModelTransform {
    pub fn matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(cgmath::Vector3::new(
            self.offset[0],
            self.offset[1],
            self.offset[2],
        ));
        let rotation = Matrix4::from(self.rotation);
        translation * rotation
    }
}
