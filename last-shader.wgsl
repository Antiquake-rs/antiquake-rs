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

struct PushConstants {
    transform:mat4x4<f32>,
    model_view:mat4x4<f32>
}
var<push_constant> push_constants: PushConstants;


 


fn convert_from_quake(in1: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(-in1.y, in1.z, -in1.x);
}

//read https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#writing-the-shaders
//clip position is the new as gl_position
@vertex
fn main_vs(
    @location(0) a_position1: vec3<f32>,
    @location(2) a_normal: vec3<f32>,
    @location(3) a_diffuse: vec2<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.f_normal =  a_normal;//mat3x3(transpose(inverse(push_constants.model_view))) * convert(a_normal);
    result.f_diffuse = a_diffuse;
    result.pos = vec4(convert_from_quake(a_position1), 1.0);   //push_constants.transform *
    return result;
}
 
 // shader global ResourceBinding { group: 0, binding: 1 } is not available in the layout pipeline layout



// set 1: per-entity 
@group(1)@binding(1) var<uniform> u_diffuse_sampler:sampler;

// set 2: per-texture chain 
@group(2)@binding(0) var<uniform> u_diffuse_texture:texture_2d<f32>;
 
 
 

@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
    var result: FragmentOutput;

   // let tex = textureLoad(r_color, vec2<i32>(vertex.tex_coord * 256.0), 0);
    //let v = f32(tex.x) / 255.0;
    //return vec4<f32>(1.0 - (v * 5.0), 1.0 - (v * 15.0), 1.0 - (v * 50.0), 1.0);
 
  result.diffuse_attachment = textureSample(
   u_diffuse_texture, u_diffuse_sampler, vertex.f_diffuse
  );
 
  // TODO: get ambient light from uniform
  result.light_attachment = vec4(0.25);

  // rescale normal to [0, 1]
  result.normal_attachment = vec4(vertex.f_normal / 2.0 + 0.5, 1.0);
 
  
  
  return result;
}
 