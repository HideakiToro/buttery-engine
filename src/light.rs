#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    /// 4th parameter is only for padding. Just ignore it in shader and code.
    pub position: [f32; 4],
    /// 4th parameter is only for padding. Just ignore it in shader and code. Light shouldn't have alpha?
    pub color: [f32; 4],
}
