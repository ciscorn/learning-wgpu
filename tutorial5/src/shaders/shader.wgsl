// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
}

@group(0) @binding(0)
var color_texture: texture_2d<f32>;
@group(0) @binding(1)
var color_sampler: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position.xyz, 1.0);
    out.tex_coord = in.tex_coord;
    return out;
}
 
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(color_texture, color_sampler, in.tex_coord);
}
