struct VertexOutput {
    @location(0) f_normal: vec3<f32>,
    @location(1) f_diffuse: vec2<f32>,
    @builtin(position) pos: vec4<f32>, 
};

struct FragmentOutput {
    @location(0) diffuse_attachment: vec4<f32>,
    @location(1) normal_attachment: vec4<f32>, 
    @location(2) light_attachment: vec4<f32>, 
};
 

struct FrameUniforms {
     //light_anim_frames: array<f32,64>,
    camera_pos: vec4<f32>,
    time:f32, 
}
struct EntityUniforms {
    u_transform:mat4x4<f32>,
    u_model:mat4x4<f32>
}

// set 0: per-frame - inside render/world/mod 
@group(0) @binding(0) var<uniform>  frameuniforms: FrameUniforms;


// set 1: per-entity
@group(1) @binding(0) var<uniform> entity_uniforms: EntityUniforms;
@group(1) @binding(1) var u_diffuse_sampler: sampler;
 
// set 2: per-texture
@group(2) @binding(0) var u_diffuse_texture: texture_2d<f32>;  //texture2D -> texture_2d


fn convert_from_quake(in1: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(-in1.y, in1.z, -in1.x);
}

//read https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#writing-the-shaders
//clip position is the new as gl_position
@vertex
fn main_vs(
   @location(0) a_position: vec3<f32>,
    @location(1) a_normal: vec3<f32>,
    @location(2) a_diffuse: vec2<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.f_normal =  a_normal;//mat3x3(transpose(inverse(entity_uniforms.u_model))) * convert(a_normal);
    result.f_diffuse = a_diffuse;
    result.pos = entity_uniforms.u_transform * vec4(convert_from_quake(a_position),1.0);   
    return result;
}
 
 // shader global ResourceBinding { group: 0, binding: 1 } is not available in the layout pipeline layout

 
  

@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
    var result: FragmentOutput;

  
  result.diffuse_attachment = textureSample(
   u_diffuse_texture, u_diffuse_sampler, vertex.f_diffuse
  );
 
  

  // rescale normal to [0, 1]
  result.normal_attachment = vec4(vertex.f_normal / 2.0 + 0.5, 1.0);
 
  
  result.light_attachment = vec4(1.0, 1.0, 1.0, 1.0);
  
  return result;
}
 