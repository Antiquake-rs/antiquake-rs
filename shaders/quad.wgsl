 

struct VertexOutput {
    @location(0) f_texcoord: vec2<f32>, 
    @builtin(position) pos: vec4<f32>, 
};

 
struct FragmentOutput {
    @location(0) color_attachment: vec4<f32> , 
};
 
struct QuadUniforms {
    transform:mat4x4<f32>, 
}
 


@group(0) @binding(0) var quad_sampler: sampler;
@group(1) @binding(0) var quad_texture: texture_2d<f32>;    
@group(2) @binding(0) var<uniform> quad_uniforms: QuadUniforms ; 
  

  
@vertex
fn main_vs(
    @location(0) a_position: vec2<f32>,
    @location(1) a_texcoord: vec2<f32>, 
) -> VertexOutput {
    var result: VertexOutput;
    result.f_texcoord =  a_texcoord; 
 
    result.pos = quad_uniforms.transform * vec4(a_position, 0.0, 1.0);   
    return result;
}
  
 
 
 
@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
    var result: FragmentOutput;
 
   
    let color:vec4<f32> = textureSample( quad_texture, quad_sampler , vec2<f32>(vertex.f_texcoord) ); 

    if (color.a == 0f) {  //will never happen ?
            discard;
    } else {
        result.color_attachment = color;
    }
    
    return result;
}
 