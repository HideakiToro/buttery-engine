#[derive(Clone)]
pub struct Mesh {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub index_format: wgpu::IndexFormat,
    // Texture
    pub texture_bind_group: wgpu::BindGroup,
}
