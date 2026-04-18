@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_position: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct Light {
    view_proj: mat4x4<f32>,
    view_position: vec4<f32>,
    color: vec4<f32>,
    direction: vec4<f32>
};
@group(1) @binding(1)
var<uniform> light: Light;

struct BiasUniform {
    bias: vec4<f32>,
};

@group(1) @binding(2)
var<uniform> u_bias: BiasUniform;

struct OffsetUniform {
    offset: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> u_offset: OffsetUniform;

@group(3) @binding(0)
var shadow_map: texture_depth_2d;

@group(3) @binding(1)
var shadow_sampler: sampler_comparison;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normals: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    var pos = vec4<f32>(model.position, 1.0) + u_offset.offset;
    out.clip_position = camera.view_proj * pos;
    out.world_position = pos.xyz;
    out.world_normal = model.normals;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {    
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    let light_space_pos = light.view_proj * vec4<f32>(in.world_position, 1.0);
    let proj = light_space_pos.xyz / light_space_pos.w;
    var uv = proj.xy * 0.5 + vec2<f32>(0.5, 0.5);
    uv.y = 1.0 - uv.y;
    let depth = proj.z;
    var shadow = textureSampleCompare(shadow_map, shadow_sampler, uv, depth - u_bias.bias.x);

    if shadow >= 0.9 {
        if dot(light.direction.xyz, in.world_normal) >= -0.1 {
            shadow = 0.25;
        } else {
            shadow = 1.0;
        }
    } else {
        shadow = 0.25;
    }

    let lighting = object_color * shadow;

    return lighting;
}