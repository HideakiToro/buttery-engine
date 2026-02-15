use std::{collections::HashMap, fs::File, io::Read};
use wgpu::{Device, IndexFormat, util::DeviceExt};

use crate::{mesh::Mesh, vertex::Vertex};

pub fn parse_obj(
    path: &str,
    offset: Option<[f32; 3]>,
    scale: Option<[f32; 3]>,
    device: &Device,
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
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&final_indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    let mesh = Mesh {
        index_buffer,
        num_indices: final_indices.len() as u32,
        vertices: final_vertices,
        index_format: IndexFormat::Uint32,
    };
    // ai-gen: end

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
