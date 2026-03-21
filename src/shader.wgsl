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
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // out.color = model.color;
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

    // let light_dir = normalize(light.view_position.xyz - in.world_position);

    // // 1. Ambient Light
    // let ambient_strength = 0.1;
    // let ambient_color = light.color.xyz * ambient_strength;

    // // 2. Diffuse Light
    // let diffuse_strength = max(dot(in.world_normal, light_dir), 0.0);
    // let diffuse_color = light.color.xyz * diffuse_strength;

    // // TODO: Cell-Shading
    // // var diffuse_color = light.color.xyz * ambient_strength;
    // // if(diffuse_strength > 0.5) {
    // //     diffuse_color = light.color.xyz;
    // // }

    // // 3. Specular Light
    // let view_dir = normalize(camera.view_position.xyz - in.world_position);
    // let half_dir = normalize(view_dir + light_dir);
    // let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);
    // let specular_color = specular_strength * light.color.xyz;

    // TODO: Cell-Shading
    // var specular_color = light.color.xyz * ambient_strength;
    // if(specular_strength > 0.8) {
    //     specular_color = light.color.xyz * 10.0;
    // }

    // let result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;

    // return vec4<f32>(result, object_color.a);

    let light_space_pos = light.view_proj * vec4<f32>(in.world_position, 1.0);
    let proj = light_space_pos.xyz / light_space_pos.w;
    var uv = proj.xy * 0.5 + vec2<f32>(0.5, 0.5);
    uv.y = 1.0 - uv.y;
    let depth = proj.z;
    var shadow = textureSampleCompare(shadow_map, shadow_sampler, uv, depth - u_bias.bias.x);

    if shadow >= 0.9 {
        shadow = 1.0;
    } else {
        shadow = 0.25;
    }

    let lighting = object_color * shadow;

    return lighting;
}