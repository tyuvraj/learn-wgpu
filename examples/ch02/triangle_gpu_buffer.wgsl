struct VertexInput {
    @location(0)  pos: vec2f,
    @location(1) color: vec3f,
};

struct VertexOutput {
    @builtin(position)  position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(
    in: VertexInput
) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(in.pos, 0.0, 1.0);
    output.color = vec4f(in.color, 1.0);
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}   