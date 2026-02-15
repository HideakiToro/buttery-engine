struct OffsetUniform {
    offset: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_offset: OffsetUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(2) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    // @location(1) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    // @location(0) color: vec3<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // out.color = model.color;
    out.tex_coords = model.tex_coords;
    // var pos = vec3<f32>(model.position.x + u_offset.offset.x, model.position.y + u_offset.offset.y, model.position.z + u_offset.offset.z);
    // out.clip_position = vec4<f32>(pos.x / (1 + pos.z), pos.y / (1 + pos.z), pos.z / 1000.0, 1.0);
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(in.color, 1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}