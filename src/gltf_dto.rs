use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTF {
    pub accessors: Vec<GLTFAccessor>,
    pub asset: GLTFAsset,
    pub buffer_views: Vec<GLTFBufferView>,
    pub buffers: Vec<GLTFBuffer>,
    pub images: Option<Vec<GLTFImage>>,
    pub materials: Vec<GLTFMaterial>,
    pub meshes: Vec<GLTFMesh>,
    pub nodes: Vec<GLTFNode>,
    pub samplers: Option<Vec<GLTFSampler>>,
    pub scene: i32,
    pub scenes: Vec<GLTFScene>,
    pub textures: Option<Vec<GLTFTexture>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFAccessor {
    pub buffer_view: Option<u32>,
    pub byte_offset: Option<u32>,
    /// 5120: Byte (int8), 5121 UByte (u8/uint8), 5122: Short (int16), 5123: UShort (uint16), 5125: UInt, 5126: Float (f12?)
    pub component_type: u32, // TODO: Use GLTFAccessorComponentType instead. This is currently failing to parse...
    /// When null, then assume false
    pub normalized: Option<bool>,
    /// The number of elements referenced
    pub count: u32,
    /// Specifies if the accessor’s elements are scalars, vectors, or matrices.
    pub r#type: GLTFAccessorType,
    pub max: Option<Vec<i32>>,
    pub min: Option<Vec<i32>>,
    pub sparse: Option<Value>, // TODO: define
    pub name: Option<String>,
    pub extensions: Option<Value>,
    pub extras: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[repr(i32)]
pub enum GLTFAccessorComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInteger = 5125,
    Float = 5126,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum GLTFAccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFAsset {
    pub generator: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFBufferView {
    /// Referenced buffer
    pub buffer: u32,
    pub byte_length: u32,
    /// when null, then assume 0
    pub byte_offset: Option<u32>,
    /// The stride, in bytes, between vertex attributes. When this is not defined, data is tightly packed. Min >= 4 Max <= 252
    pub byte_stride: Option<i32>,
    /// Recommended GPU Buffer Type
    pub target: Option<u32>, // TODO: Use GLTFBufferViewTarget instead. This is currently failing to parse...
    pub name: Option<String>,
    pub extensions: Option<Vec<Value>>,
    pub extras: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize)]
#[repr(i32)]
pub enum GLTFBufferViewTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFBuffer {
    pub byte_length: i32,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFImage {
    pub buffer_view: u32,
    pub mime_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFMaterial {
    pub double_sided: bool,
    pub name: String,
    pub pbr_metallic_roughness: GLTFMaterialPBRMetallicRoughness,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFMaterialPBRMetallicRoughness {
    pub base_color_factor: Option<[f32; 4]>,
    pub base_color_texture: Option<GLTFMaterialBaseColorTexture>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFMaterialBaseColorTexture {
    pub index: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFMesh {
    pub primitives: Vec<GLTFMeshPrimitive>,
    pub weights: Option<Vec<i32>>,
    pub name: String,
    pub extensions: Option<Vec<Value>>,
    pub extras: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFMeshPrimitive {
    pub attributes: GLTFMeshPrimitiveAttributes,
    pub indices: u32,
    pub material: u32,
    /// When null, then assume Triangles
    pub mode: Option<GLTFMeshMode>,
    pub targets: Option<Value>,
    pub extensions: Option<Vec<Value>>,
    pub extras: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Each number is a reference to an accessor.
pub struct GLTFMeshPrimitiveAttributes {
    /// In Accessor, this is gonna be a list of vec3 (3 * float)
    pub normal: u32,
    /// In Accessor, this is gonna be a list of vec3 (3 * float)
    pub position: u32,
    /// In Accessor, this is gonna be a list of vec3 (3 * float)
    pub texcoord_0: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[repr(i32)]
pub enum GLTFMeshMode {
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFNode {
    pub mesh: u32,
    pub name: String,
    pub translation: Option<[f32; 3]>,
    pub children: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFSampler {
    pub mag_filter: i32,
    pub min_filter: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFScene {
    pub name: String,
    pub nodes: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTFTexture {
    pub sampler: u32,
    pub source: u32,
}
