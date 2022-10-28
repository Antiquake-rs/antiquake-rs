 
/*
 we should provide the color in a buffer instead of a push constant . push constants are already stressed to the MAX_LIGHTS
 */

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

    //mod the texcoord by the color 
    let color_index:i32 = push_constants.color;

    let COLOR_ROWS:i32 = 8;
    let COLOR_COLS:i32 = 8;

    let x_row:i32 = color_index % COLOR_ROWS;
    let y_row:i32 = color_index / COLOR_COLS;

    let tile_scale = 0.125f;

    //similar to glyph code 
   // result.f_texcoord =  a_texcoord; 
    result.f_texcoord =  vec2((a_texcoord.x  +  f32(x_row))* tile_scale ,(  a_texcoord.y + f32(y_row)) * tile_scale); 

 
    result.pos = push_constants.transform * vec4(a_position,1.0);   
    return result;
}
  
 
  
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
 