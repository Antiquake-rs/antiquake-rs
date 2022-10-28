 

struct VertexOutput {
    @location(0) f_texcoord: vec2<f32>, 
    @builtin(position) pos: vec4<f32>, 
};

struct FragmentOutput {
    @location(0) diffuse_attachment: vec4<f32> ,
    @location(2) light_attachment: vec4<f32> ,
};
 
struct PushConstants {
    transform:mat4x4<f32>,
    color: i32,
}
var<push_constant> push_constants: PushConstants;
 
 


@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var u_texture: texture_2d<f32>; 

  

  
@vertex
fn main_vs(
    @location(0) a_position: vec3<f32>,
    @location(1) a_texcoord: vec2<f32>, 
) -> VertexOutput {
    var result: VertexOutput;
    result.f_texcoord =  a_texcoord; 
 
    result.pos = push_constants.transform * vec4(a_position,1.0);   
    return result;
}
  
 
 

//why would a particle shader need an array of 256 textures being pumped in ? seems overkill   

@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
var result: FragmentOutput;

  
  let tex_color:vec4<f32> = textureSample(   
     u_texture, u_sampler, vertex.f_texcoord  // u_texture[push_constants.color], u_sampler, f_texcoord
  );

  if (tex_color.a == 0.0) {
    discard;
  }

  result.diffuse_attachment = tex_color;
  result.light_attachment = vec4(0.25);

  
  return result;
}
 