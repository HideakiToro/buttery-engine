use crate::vertex::Vertex;

#[derive(Clone)]
pub struct Mesh {
    pub index_buffer: wgpu::Buffer,
    pub vertices: Vec<Vertex>,
    pub num_indices: u32,
}
