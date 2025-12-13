// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
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
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
