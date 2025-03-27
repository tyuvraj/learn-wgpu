struct Output {
    @builtin(position) Position: vec4f,
    @location(0) Color:vec4f,
};

@vertex
fn vs_main(
    @builtin(vertex_index) VertexIndex: u32
) -> Output {
    var pos = array<vec2f, 3>(
        vec2f(0.0, 0.5),
        vec2f(-0.5, -0.5),
        vec2f(0.5, -0.5)
    );

    var color = array<vec3f, 3>(
        vec3f(1.0, 0.0, 0.0),
        vec3f(0.0, 1.0, 0.0),
        vec3f(0.0, 0.0, 1.0)
    );

    var output: Output;
    output.Position = vec4f(pos[VertexIndex], 0.0, 1.0);
    output.Color = vec4f(color[VertexIndex], 1.0);
    return output;
}

@fragment
fn fs_main(@location(0) vColor: vec4f) -> @location(0) vec4f {
    return vColor;
}   