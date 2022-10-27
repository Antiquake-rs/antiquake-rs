
// if this is changed, it must also be changed in client::entity
let MAX_LIGHTS:i32 = 32;


struct VertexOutput {
    @location(0) f_texcoord: vec2<f32>, 
    @builtin(position) pos: vec4<f32>, 
};

struct FragmentOutput {
    @location(0) color_attachment: vec4<f32> 
};
 

 
struct DeferredUniforms {
    inv_projection:mat4x4<f32>,
    light_count:i32,
    _pad1:i32,
    _pad2:vec2<i32>,
    lights:array< vec4<f32>, MAX_LIGHTS >
}
 

 


@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var u_diffuse: texture_multisampled_2d<f32>;  //texture2DMS //texture_multisampled_2d
@group(0) @binding(2) var u_normal: texture_multisampled_2d<f32>;  //texture2DMS
@group(0) @binding(3) var u_light: texture_multisampled_2d<f32>;  //texture2DMS
@group(0) @binding(4) var u_depth: texture_multisampled_2d<f32>;  //texture2DMS

@group(0) @binding(5) var<uniform> u_deferred: DeferredUniforms;  
  

  
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
  




  
fn dlight_origin( dlight:vec4<f32> ) -> vec3<f32> {
  return dlight.xyz;
}

fn dlight_radius( dlight:vec4<f32> ) -> f32 {
  return dlight.w;
}

fn reconstruct_position(depth :f32, f_texcoord:vec2<f32>) -> vec3<f32> {

    // ???

   let x:f32 = 0.5; //f_texcoord.s * 2.0 - 1.0;
   let y:f32 = 0.5; //(1.0 - f_texcoord.t) * 2.0 - 1.0;

   let ndc:vec4<f32> = vec4(x, y, depth, 1.0);
   let view:vec4<f32> = u_deferred.inv_projection * ndc;
   
   return view.xyz / view.w;
}




  

@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
    var result: FragmentOutput;

   let dims:vec2<i32> = textureDimensions( u_diffuse );
   let texcoord:vec2<f32> =  vec2<f32>( f32(dims.x) * vertex.f_texcoord.x, f32(dims.y) * vertex.f_texcoord.y );
 
  let in_color:vec4<f32> = textureLoad( u_diffuse, vec2<i32>(texcoord) , 0 ); //texel fetch 

  // scale from [0, 1] to [-1, 1]
 

    let in_normal:vec3<f32> = 2.0
    * textureLoad(u_normal, vec2<i32>(texcoord), 0).xyz   //was texel fetch 
    - 1.0;

  // Double to restore overbright values.
  
  let in_light:vec4<f32> = 2.0 * textureLoad( u_light,  vec2<i32>(texcoord) ,0 );

  //let in_depth:f32 = textureSample( u_depth, u_sampler , vec2<f32>(texcoord) ).x;
  let in_depth:f32 = textureLoad( u_depth,   vec2<i32>(texcoord) , 0 ).x;

  let position:vec3<f32> = reconstruct_position(in_depth, vertex.f_texcoord);

  let out_color:vec4<f32> = in_color;

  var light:f32 = in_light.x + in_light.y + in_light.z + in_light.w;
  for (var i:i32 = 0; i < u_deferred.light_count && i < MAX_LIGHTS; i++) {
    let dlight:vec4<f32> = u_deferred.lights[i];
    let dir:vec3<f32> = normalize(position - dlight_origin(dlight));
    let dist:f32 = abs(distance(dlight_origin(dlight), position));
    let radius:f32 = dlight_radius(dlight);

    if (dist < radius && dot(dir, in_normal) < 0.0) {
      // linear attenuation
      light += (radius - dist) / radius;
    }
  }

  result.color_attachment = vec4(light * out_color.rgb, 1.0);


  
  return result;
}
 