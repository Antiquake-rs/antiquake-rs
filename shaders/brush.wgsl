//see brush.vert and brush.frag  and this file for a guide on how to convert these boyz !

let TEXTURE_KIND_REGULAR:i32 = 0;
let TEXTURE_KIND_WARP:i32 = 1;
let TEXTURE_KIND_SKY:i32 = 2; 

let WARP_AMPLITUDE:f32 = 0.15;
let WARP_FREQUENCY:f32 = 0.25;
let WARP_SCALE:f32 = 1.0;

let  LIGHTMAP_ANIM_END:i32  = 255;

struct VertexOutput {
    @location(0) f_normal: vec3<f32>,
    @location(1) f_diffuse: vec2<f32>,
    @location(2) f_lightmap: vec2<f32>,
    @location(3) f_lightmap_anim: vec4<f32>,
    @builtin(position) pos: vec4<f32>, 
};

struct FragmentOutput {
    @location(0) diffuse_attachment: vec4<f32>,
    @location(1) normal_attachment: vec4<f32>, 
    @location(2) light_attachment: vec4<f32>, 
};

//see struct at render/world/mod 275 
struct FrameUniforms {
     //light_anim_frames: array<f32,64>,
    camera_pos: vec4<f32>,
    time:f32,
      r_lightmap:  u32
}

struct TextureUniforms { 
    kind:i32
}

struct PushConstants {
    transform: mat4x4<f32>,
    model_view: mat4x4<f32>,
    texture_kind:i32
}
var<push_constant> push_constants: PushConstants;




// set 0: per-frame - inside render/world/mod 
@group(0) @binding(0) var<uniform>  frameuniforms: FrameUniforms;


// set 1: per-entity
@group(1) @binding(1) var u_diffuse_sampler: sampler;
@group(1) @binding(2) var u_lightmap_sampler: sampler;

// set 2: per-texture
@group(2) @binding(0) var u_diffuse_texture: texture_2d<f32>;  //texture2D -> texture_2d
@group(2) @binding(1) var u_fullbright_texture: texture_2d<f32>;
//@group(2) @binding(2) var texture_uniforms: TextureUniforms;

//@group(3) @binding(0) var u_lightmap_texture:  texture_2d_array<f32>;

 

// convert from Quake coordinates
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
    @location(3) a_lightmap: vec2<f32>,
 //   @location(4) a_lightmap_anim: vec4<f32>,
) -> VertexOutput {
    
    var result: VertexOutput;
  
  
     if (push_constants.texture_kind == TEXTURE_KIND_SKY) {
        var dir:vec3<f32> = a_position - frameuniforms.camera_pos.xyz;
        dir = vec3(dir.x,dir.y,dir.z * 3.0);

        // the coefficients here are magic taken from the Quake source
        let len:f32 = 6.0 * 63.0 / length(dir);
        dir = vec3(dir.xy * len, dir.z);
        result.f_diffuse = ( (8.0 * frameuniforms.time % 128.0) + dir.xy) / 128.0;     //instead of modf just use %
    } else {
        result.f_diffuse = a_diffuse;
    }

    result.f_normal = a_normal; //mat3x3(transpose(inverse(push_constants.model_view))) * convert(a_normal);
    result.f_lightmap = a_lightmap;
  //  result.f_lightmap_anim = a_lightmap_anim;
    result.pos = push_constants.transform * vec4(convert_from_quake(a_position), 1.0);



    return result;
}
 
 // shader global ResourceBinding { group: 0, binding: 1 } is not available in the layout pipeline layout

 
//@group(0)@binding(0) var u_diffuse_texture: texture_2d<f32>;
//@group(0)@binding(1) var s_diffuse: sampler;

//@group(2) @binding(1) var u_diffuse_sampler: sampler;


//big loop that is costly !
 /*  fn calc_light( vertex: VertexOutput ) -> vec4<f32> {
    var light:vec4<f32> = vec4(0.0, 0.0, 0.0, 0.0);
    for (var i:i32 = 0; i < 4 && vertex.f_lightmap_anim[i] != LIGHTMAP_ANIM_END; i++) {
        let map:f32 = textureSample(
              u_lightmap_texture[i], u_lightmap_sampler , vertex.f_lightmap
        ).r;

        // range [0, 4]
        let style:f32 = frameuniforms.light_anim_frames[vertex.f_lightmap_anim[i]];
        light[i] = map * style;
    }

    return light;
}*/
 

@fragment
fn main_fs(vertex: VertexOutput) -> FragmentOutput {
   
    var result: FragmentOutput;

       switch (push_constants.texture_kind) {
        case 0: { // TEXTURE_KIND_REGULAR
            result.diffuse_attachment = textureSample(
               u_diffuse_texture, u_diffuse_sampler,  vertex.f_diffuse
            );

            let fullbright:f32 = textureSample(
               u_fullbright_texture,  u_diffuse_sampler ,  vertex.f_diffuse
            ).r;

            if (fullbright != 0.0) {
                result.light_attachment = vec4(0.25);
            } else {
               result.light_attachment = vec4(0.25);
               // result.light_attachment = calc_light( vertex );
            }
            break;
        }
        case 1: { //TEXTURE_KIND_WARP
            // note the texcoord transpose here
          /*  let wave1:vec2<f32> = 3.14159265359
                * (WARP_SCALE * vertex.f_diffuse.ts
                    + WARP_FREQUENCY * frameuniforms.time);

            let warp_texcoord:vec2<f32> = vertex.f_diffuse.st + WARP_AMPLITUDE
                * vec2(sin(wave1.s), sin(wave1.t));  */

             let emptywave:vec2<f32> = vec2(0.0);

            result.diffuse_attachment = textureSample(
                u_diffuse_texture, u_diffuse_sampler, emptywave //warp_texcoord
            );
            result.light_attachment = vec4(0.25);
            break;
            }


            //swizzling is  x, y, z, w  instead of  stpq (for texture coordinates)
        case 2: { //TEXTURE_KIND_SKY
            let base:vec2<f32> =  (vertex.f_diffuse + frameuniforms.time % 1.0);  //just use % instead of modf 
            let cloud_texcoord:vec2<f32> = vec2(0.0, 1.0); //vec2(base.s * 0.5, base.t);
            let sky_texcoord:vec2<f32> = vec2( 0.5, 1.0); //vec2(base.s * 0.5 + 0.5, base.t);

            let sky_color:vec4<f32> = textureSample(
                u_diffuse_texture, u_diffuse_sampler,
                sky_texcoord
            );
            let cloud_color:vec4<f32> = textureSample(
                u_diffuse_texture, u_diffuse_sampler,
                cloud_texcoord
            );

            // 0.0 if black, 1.0 otherwise
            var cloud_factor:f32;
            if (cloud_color.r + cloud_color.g + cloud_color.b == 0.0) {
                cloud_factor = 0.0;
            } else {
                cloud_factor = 1.0;
            }
            result.diffuse_attachment = mix(sky_color, cloud_color, cloud_factor);
            result.light_attachment = vec4(0.25);
            break;
        }
        // not possible
        default:{
            break;
        }
    }

    // rescale normal to [0, 1]
    result.normal_attachment = vec4(vertex.f_normal / 2.0 + 0.5, 1.0);


    return result;
}
 


 