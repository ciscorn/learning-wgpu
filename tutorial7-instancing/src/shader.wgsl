// Vertex shader

struct WorldUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> world: WorldUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
}

struct InstanceInput {
    @location(2) model_mat_0: vec4<f32>,
    @location(3) model_mat_1: vec4<f32>,
    @location(4) model_mat_2: vec4<f32>,
    @location(5) model_mat_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(vert: VertexInput, inst: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    let model_mat = mat4x4<f32>(
        inst.model_mat_0,
        inst.model_mat_1,
        inst.model_mat_2,
        inst.model_mat_3
    );

    out.clip_position = world.view_proj * model_mat * vec4<f32>(vert.position.xyz, 1.0);
    out.tex_coord = vert.tex_coord;
    return out;
}
 

@group(1) @binding(0)
var color_texture: texture_2d<f32>;
@group(1) @binding(1)
var color_sampler: sampler;

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(color_texture, color_sampler, in.tex_coord);
}
