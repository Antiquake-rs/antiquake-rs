 

struct VertexOutput {
    @location(0) f_texcoord: vec2<f32>,  
    @builtin(position) pos: vec4<f32>, 
};

 
struct FragmentOutput {
    @location(0) color_attachment: vec4<f32> , 
};
  


@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var u_texture: texture_2d<f32>;   
  

  
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
   
    let color:vec4<f32> = textureSample( u_texture, u_sampler , vec2<f32>(vertex.f_texcoord) );  
    
    result.color_attachment = color; 
    
    return result;
}
 