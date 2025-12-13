// Vertex shader

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
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // Out coordinates seem to be from -1 to 1
    // So this hard-codes locations based on the index, rather than any actual
    // vertex data. For example, the X values are 0.5, 0, -0.5, making the
    // vertices of the triangle in counter-clockwise order.
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}
