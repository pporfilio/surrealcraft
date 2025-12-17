// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    // @builtin(position), when used in the output of a vertex shader,
    // is the computed position of the vertex. When the name `@builtin(position)` is used
    // in a fragment shader, it refers to the pixel coordinate of that fragment. Same name,
    // different context and use.
    // @builtin maybe means that it's something created/used/modified between stages? 
    // Because I can access in.clip_position in the fs, but it's not the same as in.color,
    // even when I set them both tot vec4<f32>(x, y, 0.0, 1.0)
    // This was fairly helpful: 
    // https://webgpufundamentals.org/webgpu/lessons/webgpu-inter-stage-variables.html
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) texture_index: u32,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) texture_index: u32,
};


@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    out.texture_index = instance.texture_index;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@group(2) @binding(0)
var texture_array: texture_2d_array<f32>;
@group(2) @binding(1)
var texture_array_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_array, texture_array_sampler, in.tex_coords, in.texture_index);
}
