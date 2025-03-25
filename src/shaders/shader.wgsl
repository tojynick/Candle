struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(
    input: VertexInput
) -> VertexOutput {

    var out: VertexOutput;

    out.uv = input.uv;
    out.clip_position = vec4<f32>(input.position, 1.0);
   


    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {

    return textureSample(t_diffuse, s_diffuse, input.uv);
}
 