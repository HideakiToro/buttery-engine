// IMPORTANT: Commented out, because it is no longer reasonable to maintain this code

// use crate::{mesh::Mesh, vertex::Vertex};
// use std::{collections::HashMap, fs::File, io::Read};
// use wgpu::{Device, IndexFormat, util::DeviceExt};

// pub fn parse_obj(path: &str, device: &Device) -> anyhow::Result<Mesh> {
//     let mut file = File::open(path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;

//     let lines = content.split("\n").collect::<Vec<&str>>();
//     let mut vertices = Vec::new();
//     let mut faces = Vec::new();

//     let _temp_texture_coords = [
//         [0.4131759, 0.00759614],
//         [0.0048659444, 0.43041354],
//         [0.28081453, 0.949397],
//         [0.85967, 0.84732914],
//         [0.9414737, 0.2652641],
//     ];

//     // vertex-index => texture-index => real-index
//     let mut combinations: HashMap<u16, HashMap<u16, u16>> = HashMap::new();

//     for line in lines {
//         if line.starts_with("# ") {
//             continue;
//         }
//         if line.is_empty() {
//             continue;
//         }

//         let mut parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
//         match parts.first().unwrap() {
//             &"v" => {
//                 let mut position = Vec::with_capacity(3);
//                 for part in parts.split_off(1) {
//                     let mut coord = part.parse::<f32>()?;
//                     coord *= 1.0;

//                     match position.len() {
//                         0 => coord += 0.0,  // x (left/right)
//                         1 => coord += -3.0, // y (up/down)
//                         2 => coord += 5.0,  // z (forward/backward)
//                         _ => {}
//                     }

//                     position.push(coord);
//                 }

//                 vertices.push(Vertex {
//                     position: position.try_into().unwrap(),
//                     // tex_coords: temp_texture_coords.get(4 - vertices.len()).unwrap().clone(), // color: [0.25, 0.25, 0.25],
//                     tex_coords: [-1.0, -1.0],
//                 });
//             }
//             &"f" => {
//                 let mut vertex_indices = Vec::new();
//                 let mut texture_indices = Vec::new();
//                 let mut normals_indices = Vec::new();
//                 for part in parts.split_off(1) {
//                     let part = part.split("/").collect::<Vec<&str>>();

//                     let vertex_index = part.first().unwrap();
//                     let vertex_index = vertex_index.parse::<u16>()? - 1;

//                     if let Some(texture_index) = part
//                         .get(1)
//                         .and_then(|texture_index| texture_index.parse::<u16>().ok())
//                     {
//                         let texture_index = texture_index - 1;
//                         texture_indices.push(texture_index);

//                         if let Some(textures) = combinations.get(&vertex_index) {
//                             if let Some(real_index) = textures.get(&texture_index) {
//                                 vertex_indices.push(*real_index);
//                             } else {
//                                 let Some(vertex) = vertices.get(vertex_index as usize) else {
//                                     panic!(
//                                         "Is the obj file not ordered to have vertices as the first elements?"
//                                     );
//                                 };

//                                 let real_index = vertices.len() as u16;
//                                 vertices.push(vertex.clone());

//                                 let mut entry = HashMap::new();
//                                 entry.insert(texture_index, real_index);
//                                 combinations.insert(vertex_index, entry);

//                                 vertex_indices.push(real_index);
//                             }
//                         } else {
//                             let mut entry = HashMap::new();
//                             entry.insert(texture_index, vertex_index);
//                             combinations.insert(vertex_index, entry);
//                             vertex_indices.push(vertex_index);
//                         }
//                     } else {
//                         vertex_indices.push(vertex_index);
//                     }

//                     if let Some(normals_index) = part
//                         .get(2)
//                         .and_then(|normals_index| normals_index.parse::<u16>().ok())
//                     {
//                         let normals_index = normals_index - 1;
//                         normals_indices.push(normals_index);
//                     }
//                 }

//                 // TODO: Also convert textures and normals
//                 let tris = vertex_indices.len() - 2;
//                 if tris > 1 {
//                     let mut triangles = Vec::new();
//                     for i in 0..tris {
//                         triangles.push(vertex_indices[vertex_indices.len() - 1]);
//                         triangles.push(vertex_indices[i + 1]);
//                         triangles.push(vertex_indices[i]);
//                     }
//                     vertex_indices = triangles;
//                 }

//                 faces.append(&mut vertex_indices);
//             }
//             &"vt" => {}
//             _ => {}
//         }
//     }

//     let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Index Buffer"),
//         contents: bytemuck::cast_slice(&faces),
//         usage: wgpu::BufferUsages::INDEX,
//     });
//     let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Vertex Buffer"),
//         contents: bytemuck::cast_slice(&vertices),
//         usage: wgpu::BufferUsages::VERTEX,
//     });

//     let mesh = Mesh {
//         index_buffer,
//         vertex_buffer,
//         num_indices: faces.len() as u32,
//         index_format: IndexFormat::Uint16,
//     };

//     return Ok(mesh);
// }
