 

struct VertexOutput {
    @location(0) f_texcoord: vec2<f32>, 
    @location(1) f_layer: u32,
    @builtin(position) pos: vec4<f32>, 
};

 
struct FragmentOutput {
    @location(0) output_attachment: vec4<f32> , 
};
 
 


@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var u_texture: texture_2d<f32>;    ///used to be tex array -- why ?
  

  
@vertex
fn main_vs(
    @location(0) a_position: vec2<f32>,
    @location(1) a_texcoord: vec2<f32>, 

    @location(2) a_instance_position: vec2<f32>,
    @location(3) a_instance_scale: vec2<f32>,
    @location(4) a_instance_layer: u32,

) -> VertexOutput {
    var result: VertexOutput;
    result.f_texcoord =  a_texcoord; 
    result.f_layer = a_instance_layer;
    result.pos = vec4(a_instance_scale * a_position + a_instance_position, 0.0, 1.0);
    return result;
}
  
 
 
 
@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
    var result: FragmentOutput;
 
   
    let color:vec4<f32> = textureSample( u_texture, u_sampler , vec2<f32>(vertex.f_texcoord) ); 

    if (color.a == 0f) {  //will never happen ?
            discard;
    } else {
        result.output_attachment = color;
    }
    
    return result;
}
 