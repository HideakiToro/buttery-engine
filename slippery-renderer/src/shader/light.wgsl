struct Light {
    view_proj: mat4x4<f32>,
    view_position: vec4<f32>,
    color: vec4<f32>,
    direction: vec4<f32>,
};
@group(0) @binding(1)
var<uniform> light: Light;

struct OffsetUniform {
    offset: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> u_offset: OffsetUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    // @location(1) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normals: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    // @location(0) color: vec3<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var pos = vec4<f32>(model.position, 1.0) + u_offset.offset;
    out.clip_position = light.view_proj * pos;
    return out;
}
