#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelTransform {
    pub offset: [f32; 4],
    pub rotation: [[f32; 4]; 4],
}
