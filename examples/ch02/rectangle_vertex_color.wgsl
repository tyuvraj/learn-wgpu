struct Output {
    @builtin(position) Position: vec4f,
    @location(0) Color:vec4f,
};


@vertex
fn vs_main(
    @builtin(vertex_index) VertexIndex: u32
) -> Output {
    var pos = array<vec2f, 6>(
        vec2f(-1, -1),
        vec2f(1, -1),
        vec2f(-1, 1),
        vec2f(-1, 1),
        vec2f(1, -1),
        vec2f(1, 1)
    );

    var color = array<vec3f, 6>(
        vec3f(1.0, 1.0, 0.0),
        vec3f(1.0, 1.0, 0.0),
        vec3f(0.0, 1.0, 1.0),
        vec3f(0.0, 1.0, 1.0),
        vec3f(1.0, 0.0, 1.0),
        vec3f(1.0, 0.0, 1.0)
    );

    var output: Output;
    let p = pos[VertexIndex];
    let q = (2.0 * p) - vec2f(1.0, -1.0);
    output.Position = vec4f(q, 0.0, 1.0);
    output.Color = vec4f(color[VertexIndex], 1.0);
    return output;
}

@fragment
fn fs_main(@location(0) vColor: vec4f) -> @location(0) vec4f {
    return vColor;
}   