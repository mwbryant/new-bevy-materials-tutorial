// for the globals.time
#import bevy_sprite::mesh2d_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct CoolMaterial {
    color: vec4<f32>,
    health: f32,
}

@group(1) @binding(0)
var<uniform> material: CoolMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var our_sampler: sampler;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var output_color = vec4<f32>(globals.time % 1.0, 1.0, 1.0, 1.0);
    output_color = output_color * textureSample(texture, our_sampler, input.uv);
    return output_color * material.color;
}
