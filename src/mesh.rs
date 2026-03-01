#[derive(Clone)]
pub struct Mesh {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub index_format: wgpu::IndexFormat,
    pub offset_buffer: wgpu::Buffer,
    pub offset_bind_group: wgpu::BindGroup,
    pub texture_bind_group: wgpu::BindGroup,
}
