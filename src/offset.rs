#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OffsetUniform {
    pub offset: [f32; 4],
}
