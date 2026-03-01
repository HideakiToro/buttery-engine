// IMPORTANT: This code is not actively maintained. It is recommended to use glb instead of obj.

use bytemuck::bytes_of;
use image::GenericImageView;
use std::{collections::HashMap, fs::File, io::Read};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device, IndexFormat, Queue,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{mesh::Mesh, offset::OffsetUniform, vertex::Vertex};

pub fn parse_obj(
    path: &str,
    offset: Option<[f32; 3]>,
    scale: Option<[f32; 3]>,
    device: &Device,
    queue: &Queue,
    texture_bind_group_layout: &BindGroupLayout,
    offset_bind_group_layout: &BindGroupLayout,
) -> anyhow::Result<Mesh> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let lines = content.split("\n").collect::<Vec<&str>>();

    let mut vertices: Vec<ObjVertex> = Vec::new();
    let mut faces: Vec<ObjFace> = Vec::new();
    let mut textures: Vec<ObjTextureCoordinate> = Vec::new();
    let mut normals: Vec<ObjNormal> = Vec::new();

    for line in lines {
        if line.starts_with("# ") {
            continue;
        }
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        match parts.first().unwrap() {
            &"v" => {
                let mut position = Vec::with_capacity(3);
                for part in parts.split_off(1) {
                    let mut coord = part.parse::<f32>()?;

                    // Scale Mesh
                    if let Some(scale) = scale {
                        match position.len() {
                            0 => coord *= scale[0], // x (left/right)
                            1 => coord *= scale[1], // y (up/down)
                            2 => coord *= scale[2], // z (forward/backward)
                            _ => {}
                        }
                    }

                    // World offset
                    if let Some(offset) = offset {
                        match position.len() {
                            0 => coord += offset[0], // x (left/right)
                            1 => coord += offset[1], // y (up/down)
                            2 => coord += offset[2], // z (forward/backward)
                            _ => {}
                        }
                    }

                    position.push(coord);
                }

                vertices.push(ObjVertex {
                    position: position.try_into().unwrap(),
                });
            }
            &"vt" => {
                let mut position = Vec::with_capacity(2);
                for part in parts.split_off(1) {
                    let coord = part.parse::<f32>()?;
                    position.push(coord);
                }

                textures.push(ObjTextureCoordinate {
                    position: position.try_into().unwrap(),
                });
            }
            &"vn" => {
                let mut normal = Vec::with_capacity(3);
                for part in parts.split_off(1) {
                    let coord = part.parse::<f32>()?;
                    normal.push(coord);
                }

                normals.push(ObjNormal {
                    _normal: normal.try_into().unwrap(),
                });
            }
            &"f" => {
                let mut points = Vec::new();

                for part in parts.split_off(1) {
                    let part = part.split("/").collect::<Vec<&str>>();

                    let vertex_index = part.get(0).unwrap();
                    let vertex_index = vertex_index.parse::<u16>()? - 1;

                    let texture_index = if let Some(texture_index) = part
                        .get(1)
                        .and_then(|texture_index| texture_index.parse::<u16>().ok())
                    {
                        Some(texture_index - 1)
                    } else {
                        None
                    };

                    let normal_index = if let Some(normal_index) = part
                        .get(2)
                        .and_then(|normal_index| normal_index.parse::<u16>().ok())
                    {
                        Some(normal_index - 1)
                    } else {
                        None
                    };

                    points.push(ObjFacePoint {
                        vertex_index,
                        texture_index,
                        normal_index,
                    });
                }

                faces.push(ObjFace { points });
            }
            &"s" => {
                if parts
                    .get(1)
                    .and_then(|part| part.parse::<u16>().ok())
                    .unwrap_or(0)
                    != 0
                {
                    println!("smoothing has not been implented yet");
                }
            }
            _ => {}
        }
    }

    // ai-gen: start (Close to what I built here before, but instead of building the vertices after the face-loop, possibly breaking the index order, the vertices are built directly. Very smart. I admit defeat...)
    let mut vertex_map = HashMap::<ObjFacePoint, u32>::new();
    let mut final_vertices = Vec::<Vertex>::new();
    let mut final_indices = Vec::<u32>::new();
    for face in faces {
        let tris = face.points.len() - 2;
        for i in 0..tris {
            let mut tri = [face.points[0], face.points[i + 1], face.points[i + 2]];
            tri.reverse(); // HTa: Doing this explicitly to make code more human understandable.
            for p in tri {
                let index = if let Some(&idx) = vertex_map.get(&p) {
                    idx
                } else {
                    let pos = vertices[p.vertex_index as usize].position;
                    let tex = p
                        .texture_index
                        .and_then(|ti| textures.get(ti as usize))
                        .map(|t| t.position)
                        .unwrap_or([-1.0, -1.0]);
                    let v = Vertex {
                        position: pos,
                        tex_coords: tex,
                    };
                    let idx = final_vertices.len() as u32;
                    final_vertices.push(v);
                    vertex_map.insert(p, idx);
                    idx
                };
                final_indices.push(index);
            }
        }
    }

    // Load Texture // TODO: Load Image per model
    let diffuse_bytes = include_bytes!("models/tree.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes)?;
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

    let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
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

    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&final_indices),
        usage: BufferUsages::INDEX,
    });
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&final_vertices),
        usage: BufferUsages::VERTEX,
    });
    // ai-gen: end

    let offset = OffsetUniform {
        offset: [0.0, 0.0, 0.0, 0.0],
    };

    let offset_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Offset Buffer"),
        contents: bytes_of(&offset),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let offset_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Offset Bind Group"),
        layout: offset_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: offset_buffer.as_entire_binding(),
        }],
    });

    let mesh = Mesh {
        index_buffer,
        vertex_buffer,
        offset_buffer,
        offset_bind_group,
        num_indices: final_indices.len() as u32,
        index_format: IndexFormat::Uint32,
        texture_bind_group,
    };

    return Ok(mesh);
}

struct ObjVertex {
    position: [f32; 3],
}

struct ObjTextureCoordinate {
    position: [f32; 2],
}

struct ObjNormal {
    _normal: [f32; 3],
}

struct ObjFace {
    points: Vec<ObjFacePoint>,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct ObjFacePoint {
    vertex_index: u16,
    texture_index: Option<u16>,
    normal_index: Option<u16>,
}
