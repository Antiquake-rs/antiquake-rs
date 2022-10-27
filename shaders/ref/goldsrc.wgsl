struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

struct Globals {
    transform: mat4x4<f32>,
};
@group(0)
@binding(0)
var<uniform> r_globals: Globals;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
) -> VertexOutput {
    var in_position: vec4<f32>;
    in_position.x = position.x;
    in_position.y = position.y;
    in_position.z = position.z;
    in_position.w = 1.0;

    var out: VertexOutput;
    out.tex_coord = tex_coord;
    out.position = r_globals.transform * in_position;
    return out;
}

@group(2)
@binding(0)
var r_texture: texture_2d<f32>;
@group(2)
@binding(1)
var r_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var tex_color: vec4<f32>;
    tex_color = textureSample(r_texture, r_sampler, in.tex_coord);
    return tex_color;
}