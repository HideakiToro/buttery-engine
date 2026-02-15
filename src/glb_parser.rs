use crate::{
    binary_loader::load_binary,
    gltf_dto::{GLTF, GLTFAccessorType, GLTFMaterial, GLTFNode},
    mesh::Mesh,
    vertex::Vertex,
};
use image::GenericImageView;
use thiserror::Error;
use wgpu::{BindGroupLayout, Device, IndexFormat, Queue, util::DeviceExt};

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
    #[error("No textures defined but referenced")]
    MissigTextures,
    #[error("No images defined but referenced")]
    MissigImages,
}

pub async fn parse_glb(
    path: &str,
    device: &Device,
    queue: &Queue,
    texture_bind_group_layout: &BindGroupLayout,
) -> anyhow::Result<Vec<Mesh>> {
    let buffer = load_binary(path).await?;

    // File Header
    let (file_is_gltf, buffer) = buffer.split_at(4);
    let (version, buffer) = buffer.split_at(4);
    let (file_size, buffer) = buffer.split_at(4);

    let is_gltf = String::from_utf8(file_is_gltf.to_vec())? == "glTF".to_string();
    if !is_gltf {
        return Err(ParseError::WrongFormat.into());
    }

    let _version = String::from_utf8(version.to_vec())?;

    let _file_size = u32::from_le_bytes(file_size.try_into()?);

    // glTF Header
    let (blob_size, buffer) = buffer.split_at(4);
    let (format, buffer) = buffer.split_at(4);

    let is_json = String::from_utf8(format.to_vec())? == "JSON".to_string();
    if !is_json {
        return Err(ParseError::WrongFormat.into());
    }

    let blob_size = u32::from_le_bytes(blob_size.try_into()?);

    // glTF Content
    let (blob, buffer) = buffer.split_at(blob_size as usize);
    let gltf = serde_json::from_slice::<GLTF>(blob)?;

    // glB Header
    let (blob_size, buffer) = buffer.split_at(4);
    let (format, buffer) = buffer.split_at(4);

    let is_binary = String::from_utf8(format.to_vec())?.contains("BIN");
    if !is_binary {
        return Err(ParseError::WrongFormat.into());
    }

    let blob_size = u32::from_le_bytes(blob_size.try_into()?);

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

            let mut image_bytes = None;
            if let Some(texture) = &gltf.materials[primitive.material as usize]
                .pbr_metallic_roughness
                .base_color_texture
            {
                let Some(textures) = &gltf.textures else {
                    return Err(ParseError::MissigTextures.into());
                };

                let tex = &textures[texture.index as usize];
                let Some(images) = &gltf.images else {
                    return Err(ParseError::MissigImages.into());
                };
                let image = &images[tex.source as usize];
                let buffer = buffer_views[image.buffer_view as usize];
                image_bytes = Some(buffer);
            }

            meshes.push(ParsedMesh {
                _material: gltf.materials[primitive.material as usize].clone(),
                indices,
                vertex_positions: positions,
                vertex_normals: normals,
                texture_positions: texture_coordinates,
                image_bytes,
            });
        }
        groups.push(ParsedMeshGroup {
            _name: mesh.name,
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
        for mesh in group.meshes {
            // Load Texture
            let image_bytes = mesh
                .image_bytes
                .unwrap_or(include_bytes!("models/tree.png"));
            let diffuse_image = image::load_from_memory(image_bytes)?;
            let diffuse_rgba = diffuse_image.to_rgba8();
            let dimensions = diffuse_image.dimensions();
            let texture_size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1, // This is a 2D texture. So there is no depth
            };
            let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[],
            });

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &diffuse_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &diffuse_rgba,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                texture_size,
            );

            let diffuse_texture_view =
                diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("diffuse_bind_group"),
                layout: texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    },
                ],
            });

            // Parse the actual mesh
            let mut vertices = Vec::new();
            for ((pos, _normal), texture) in mesh
                .vertex_positions
                .iter()
                .zip(mesh.vertex_normals)
                .zip(mesh.texture_positions)
            {
                vertices.push(Vertex {
                    position: pos.clone(),
                    tex_coords: texture.clone(),
                });
            }

            // mesh.indices.reverse();

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            meshes.push(Mesh {
                index_buffer,
                vertex_buffer,
                num_indices: mesh.indices.len() as u32,
                index_format: IndexFormat::Uint16,
                texture_bind_group,
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
struct ParsedMeshGroup<'a> {
    _name: String,
    meshes: Vec<ParsedMesh<'a>>,
}

impl<'a> ParsedMeshGroup<'a> {
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
struct ParsedMesh<'a> {
    vertex_positions: Vec<[f32; 3]>,
    vertex_normals: Vec<[f32; 3]>,
    texture_positions: Vec<[f32; 2]>,
    indices: Vec<u16>,
    _material: GLTFMaterial,
    image_bytes: Option<&'a [u8]>,
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
