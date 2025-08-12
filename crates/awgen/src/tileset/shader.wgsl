#import bevy_pbr::{
    mesh_functions::mesh_position_local_to_clip,
    mesh_functions::get_world_from_local,
    mesh_functions::mesh_normal_local_to_world,
}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) uv: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec3<f32>,
    @location(2) color: vec4<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
};

@group(2) @binding(0) var texture: texture_2d_array<f32>;
@group(2) @binding(1) var texture_sampler: sampler;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = mesh_position_local_to_clip(
        get_world_from_local(input.instance_index),
        vec4<f32>(input.position, 1.0),
    );
    output.normal = mesh_normal_local_to_world(
        input.normal,
        input.instance_index
    );
    output.uv = input.uv;
    output.color = input.color;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> FragmentOutput {
    var output: FragmentOutput;
    output.color = textureSample(
        texture,
        texture_sampler,
        input.uv.xy,
        i32(input.uv.z)
    ) * input.color;
    return output;
}
