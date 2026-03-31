struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec4f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec4f,
};

struct WaveMaterial {
    // webgl structs must be 16 byte aligned
    @if(sixteen_byte_align)
    _webgl2_padding: vec3f,
    time: f32,
}

@group(1) @binding(0)
var<uniform> material: WaveMaterial;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4f(model.position, 1.0);

    let offset = saturate(out.clip_position.y) * sin(material.time);
    out.clip_position.x += offset;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}
