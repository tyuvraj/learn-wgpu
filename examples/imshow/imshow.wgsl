struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) tex_coords: vec2f,
}

@group(0) @binding(0) var uImage: texture_2d<f32>;
@group(0) @binding(1) var uSampler: sampler;

@vertex
fn vs_main(
    @builtin(vertex_index) VertexIndex: u32
) -> VertexOutput {
     var pos = array<vec2f, 6>(
        vec2f(0, 0),
        vec2f(0, 1),
        vec2f(1, 0),
        vec2f(1, 0),
        vec2f(0, 1),
        vec2f(1, 1)
    );


    var output: VertexOutput;
    var uik = pos[VertexIndex];
    output.tex_coords = vec2f(uik.x, uik.y);
    uik = (2.0*uik-1.0);
    output.clip_position = vec4f(uik.x, -uik.y, 0.0, 1.0);
    return output;
}

@fragment
fn fs_main(ou: VertexOutput) -> @location(0) vec4f {
    let vColor = textureSample(uImage, uSampler, ou.tex_coords);
    return vColor;
}   