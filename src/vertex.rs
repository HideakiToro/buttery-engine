#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color
                // wgpu::VertexAttribute {
                //     // Offset by position size
                //     offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                //     shader_location: 1,
                //     format: wgpu::VertexFormat::Float32x3,
                // },
                // Texture
                wgpu::VertexAttribute {
                    // Offset by position size
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

// pub const VERTICES: &[Vertex] = &[
//     Vertex {
//         position: [-0.0868241, 0.49240386, 0.0],
//         color: [0.0, 0.0, 0.5],
//     }, // A
//     Vertex {
//         position: [-0.49513406, 0.06958647, 0.0],
//         color: [0.5, 0.0, 0.0],
//     }, // B
//     Vertex {
//         position: [-0.21918549, -0.44939706, 0.0],
//         color: [0.0, 0.5, 0.0],
//     }, // C
//     Vertex {
//         position: [0.35966998, -0.3473291, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // D
//     Vertex {
//         position: [0.44147372, 0.2347359, 0.0],
//         color: [0.0, 0.5, 0.5],
//     }, // E
// ];

// pub const VERTICES_A: &[Vertex] = &[
//     Vertex {
//         position: [-0.0868241, 0.49240386, 0.5],
//         color: [0.0, 0.0, 0.5],
//     }, // A
//     Vertex {
//         position: [-0.49513406, 0.06958647, 0.5],
//         color: [0.0, 0.0, 0.5],
//     }, // B
//     Vertex {
//         position: [-0.21918549, -0.44939706, 0.5],
//         color: [0.0, 0.0, 0.5],
//     }, // C
//     Vertex {
//         position: [0.35966998, -0.3473291, 0.5],
//         color: [0.0, 0.0, 0.5],
//     }, // D
//     Vertex {
//         position: [0.44147372, 0.2347359, 0.5],
//         color: [0.0, 0.0, 0.5],
//     }, // E
// ];

// pub const INDICES_A: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

// const OFFSET_Z: f32 = 0.1;
// const OFFSET_X: f32 = 0.0;

// pub const VERTICES_B: &[Vertex] = &[
//     Vertex {
//         position: [-0.0868241 + OFFSET_X, 0.49240386, OFFSET_Z],
//         color: [0.0, 0.5, 0.5],
//     }, // A
//     Vertex {
//         position: [-0.49513406 + OFFSET_X, 0.06958647, OFFSET_Z],
//         color: [0.0, 0.5, 0.5],
//     }, // B
//     Vertex {
//         position: [-0.21918549 + OFFSET_X, -0.44939706, OFFSET_Z],
//         color: [0.0, 0.5, 0.5],
//     }, // C
//     Vertex {
//         position: [0.35966998 + OFFSET_X, -0.3473291, OFFSET_Z],
//         color: [0.0, 0.5, 0.5],
//     }, // D
//     Vertex {
//         position: [0.44147372 + OFFSET_X, 0.2347359, OFFSET_Z],
//         color: [0.0, 0.5, 0.5],
//     }, // E
// ];

// pub const INDICES_B: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
