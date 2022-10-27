 

struct VertexOutput {
    @location(0) f_texcoord: vec2<f32>, 
    @builtin(position) pos: vec4<f32>, 
};

struct FragmentOutput {
    @location(0) color_attachment: vec4<f32> , 
};
 
struct PostProcessUniforms {
    color_shift:vec4<f32>, 
}
 


@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var u_color: texture_2d<f32>;   // was texture2DMS
@group(0) @binding(2) var<uniform> postprocess_uniforms: PostProcessUniforms ; 
  

  
@vertex
fn main_vs(
    @location(0) a_position: vec2<f32>,
    @location(1) a_texcoord: vec2<f32>, 
) -> VertexOutput {
    var result: VertexOutput;
    result.f_texcoord =  a_texcoord; 
 
    result.pos = vec4(a_position * 2.0 - 1.0, 0.0, 1.0);   
    return result;
}
  
 
 
 
@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
    var result: FragmentOutput;

    let dims:vec2<i32> = textureDimensions( u_color );
    let texcoord:vec2<f32> =  vec2<f32>( f32(dims.x) * vertex.f_texcoord.x, f32(dims.y) * vertex.f_texcoord.y );
   
   
    let in_color:vec4<f32> = textureSample( u_color, u_sampler , vec2<f32>(texcoord) ); //was texel fetch 


    let src_factor:f32 = postprocess_uniforms.color_shift.a;
    let dst_factor:f32 = 1.0 - src_factor;
    let color_shifted:vec4<f32> = src_factor * postprocess_uniforms.color_shift + dst_factor * in_color;

    result.color_attachment = color_shifted;
  
    return result;
}
 