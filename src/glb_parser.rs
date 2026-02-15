use crate::{
    gltf_dto::{GLTF, GLTFAccessorType, GLTFMaterial, GLTFNode},
    mesh::Mesh,
    vertex::Vertex,
};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read};
use thiserror::Error;
use wgpu::{Device, util::DeviceExt};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("File is not a glb file")]
    WrongFormat,
    #[error("A chunk of data was not a vec3")]
    NotAVec3,
    #[error("A chunk of data was not a vec2")]
    NotAVec2,
    #[error("An accessor has the wrong type")]
    AccessorHasWrongType,
    #[error("A mesh is missing")]
    MissingMesh,
}

// TODO: Change result type to Vec<Mesh>
pub fn parse_glb(
    path: &str,
    offset: Option<[f32; 3]>,
    scale: Option<[f32; 3]>,
    device: &Device,
) -> anyhow::Result<Vec<Mesh>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // File Header
    let (file_is_gltf, buffer) = buffer.split_at(4);
    let (version, buffer) = buffer.split_at(4);
    let (file_size, buffer) = buffer.split_at(4);

    let is_gltf = String::from_utf8(file_is_gltf.to_vec())? == "glTF".to_string();
    if !is_gltf {
        return Err(ParseError::WrongFormat.into());
    }

    let version = String::from_utf8(version.to_vec())?;
    println!("GLB-File Version: {version}");

    let file_size = u32::from_le_bytes(file_size.try_into()?);
    println!("Total file size: {file_size} bytes");

    // glTF Header
    let (blob_size, buffer) = buffer.split_at(4);
    let (format, buffer) = buffer.split_at(4);

    let is_json = String::from_utf8(format.to_vec())? == "JSON".to_string();
    if !is_json {
        return Err(ParseError::WrongFormat.into());
    }

    let blob_size = u32::from_le_bytes(blob_size.try_into()?);
    println!("Total data size: {blob_size} bytes");

    // glTF Content
    let (blob, buffer) = buffer.split_at(blob_size as usize);
    let gltf = serde_json::from_slice::<GLTF>(blob)?;

    println!("{}", gltf.buffers[0].byte_length);

    // glB Header
    let (blob_size, buffer) = buffer.split_at(4);
    let (format, buffer) = buffer.split_at(4);

    let is_binary = String::from_utf8(format.to_vec())?.contains("BIN");
    if !is_binary {
        return Err(ParseError::WrongFormat.into());
    }

    let blob_size = u32::from_le_bytes(blob_size.try_into()?);
    println!("Total binary size: {blob_size} bytes");

    let (blob, _buffer) = buffer.split_at(blob_size as usize);

    // binary parsing
    let mut buffer_views = Vec::new();
    for buffer_view in gltf.buffer_views {
        let (_, buffer) = blob.split_at(buffer_view.byte_offset.unwrap_or(0) as usize);
        let (view, _) = buffer.split_at(buffer_view.byte_length as usize);

        buffer_views.push(view);
    }

    let mut accessors = Vec::new();
    for accessor in gltf.accessors {
        let buffer = buffer_views[accessor.buffer_view.unwrap_or(0) as usize];

        let chunks = buffer.chunks(buffer.len() / accessor.count as usize);
        let accessor_data = match accessor.r#type {
            GLTFAccessorType::Vec3 => {
                let mut parsed = Vec::new();
                for chunk in chunks {
                    let chunks = chunk.chunks(4).collect::<Vec<&[u8]>>();
                    let mut parts = Vec::new();
                    for chunk in chunks {
                        let part = f32::from_le_bytes(chunk.try_into()?);
                        parts.push(part);
                    }

                    let vec3: [f32; 3] = parts.try_into().map_err(|_| ParseError::NotAVec3)?;
                    parsed.push(vec3);
                }
                ParsedAccessorData::Vec3(parsed)
            }
            GLTFAccessorType::Vec2 => {
                let mut parsed = Vec::new();
                for chunk in chunks {
                    let chunks = chunk.chunks(4).collect::<Vec<&[u8]>>();
                    let mut parts = Vec::new();
                    for chunk in chunks {
                        let part = f32::from_le_bytes(chunk.try_into()?);
                        parts.push(part);
                    }

                    let vec2: [f32; 2] = parts.try_into().map_err(|_| ParseError::NotAVec3)?;
                    parsed.push(vec2);
                }
                ParsedAccessorData::Vec2(parsed)
            }
            GLTFAccessorType::Scalar => {
                let mut parsed = Vec::new();
                for chunk in chunks {
                    let scalar = u16::from_le_bytes(chunk.try_into()?);
                    parsed.push(scalar);
                }
                ParsedAccessorData::Scalar(parsed)
            }
            _ => {
                continue;
            }
        };
        accessors.push(accessor_data);
    }

    let mut groups = Vec::new();
    for mesh in gltf.meshes {
        let mut meshes = Vec::new();
        for primitive in mesh.primitives {
            let ParsedAccessorData::Scalar(indices) = accessors[primitive.indices as usize].clone()
            else {
                return Err(ParseError::AccessorHasWrongType.into());
            };
            let ParsedAccessorData::Vec3(normals) =
                accessors[primitive.attributes.normal as usize].clone()
            else {
                return Err(ParseError::AccessorHasWrongType.into());
            };
            let ParsedAccessorData::Vec3(positions) =
                accessors[primitive.attributes.position as usize].clone()
            else {
                return Err(ParseError::AccessorHasWrongType.into());
            };
            let ParsedAccessorData::Vec2(texture_coordinates) =
                accessors[primitive.attributes.texcoord_0 as usize].clone()
            else {
                return Err(ParseError::AccessorHasWrongType.into());
            };

            meshes.push(ParsedMesh {
                material: gltf.materials[primitive.material as usize].clone(),
                indices,
                vertex_positions: positions,
                vertex_normals: normals,
                texture_positions: texture_coordinates,
            });
        }
        groups.push(ParsedMeshGroup {
            name: mesh.name,
            meshes,
        });
    }

    // Apply transforms due to world-position and parenting
    for node in &gltf.nodes {
        let Some(transform) = node.translation else {
            continue;
        };
        let meshes = node.get_all_mesh_indices(&gltf.nodes);
        for mesh_index in meshes {
            let Some(res) = groups.get_mut(mesh_index as usize) else {
                return Err(ParseError::MissingMesh.into());
            };

            res.apply_transform(transform);
        }
    }

    // Parse into Buttery-Engine Mesh Format
    let mut meshes: Vec<Mesh> = Vec::new();
    for group in groups {
        for mut mesh in group.meshes {
            let mut vertices = Vec::new();
            for ((pos, _normal), texture) in mesh
                .vertex_positions
                .iter()
                .zip(mesh.vertex_normals)
                .zip(mesh.texture_positions)
            {
                println!("{texture:#?}");
                vertices.push(Vertex {
                    position: pos.clone(),
                    tex_coords: texture.clone(),
                });
            }

            mesh.indices.reverse();

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            meshes.push(Mesh {
                index_buffer,
                num_indices: mesh.indices.len() as u32,
                vertices,
            });
        }
    }

    Ok(meshes)
}

#[derive(Clone)]
enum ParsedAccessorData {
    Vec3(Vec<[f32; 3]>),
    Vec2(Vec<[f32; 2]>),
    Scalar(Vec<u16>),
}

#[derive(Debug, Clone)]
struct ParsedMeshGroup {
    name: String,
    meshes: Vec<ParsedMesh>,
}

impl ParsedMeshGroup {
    pub fn apply_transform(&mut self, transform: [f32; 3]) {
        for mesh in self.meshes.iter_mut() {
            for vertex in mesh.vertex_positions.iter_mut() {
                for (part, transform) in vertex.iter_mut().zip(transform) {
                    *part += transform;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ParsedMesh {
    vertex_positions: Vec<[f32; 3]>,
    vertex_normals: Vec<[f32; 3]>,
    texture_positions: Vec<[f32; 2]>,
    indices: Vec<u16>,
    material: GLTFMaterial,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct ParsedVertex {
    vertex_index: u16,
    texture_index: Option<u16>,
    normal_index: Option<u16>,
}

impl GLTFNode {
    fn get_all_mesh_indices(&self, nodes: &Vec<GLTFNode>) -> Vec<u32> {
        let mut meshes = Vec::new();
        if let Some(children) = &self.children {
            for child in children {
                meshes.append(&mut nodes[child.clone() as usize].get_all_mesh_indices(nodes));
            }
        }

        meshes.push(self.mesh);

        meshes
    }
}
