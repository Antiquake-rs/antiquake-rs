 

struct VertexOutput {
    @location(0) f_texcoord: vec2<f32>, 
 //   @location(1) f_layer: u32,
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
    @location(4) a_instance_glyph_index: u32,

) -> VertexOutput {



    var result: VertexOutput;

    /*
    const GLYPH_COLS: usize = 16;
    const GLYPH_ROWS: usize = 16;
    */

    let GLYPH_ROWS:u32 = 16u;
    let GLYPH_COLS:u32 = 16u;

    let tile_scale = 0.0625f;

    let x_row:u32 = a_instance_glyph_index % GLYPH_COLS;
    let y_row:u32 = a_instance_glyph_index / GLYPH_COLS;


    //need to modify texcoords based on the a_instance_glyph_index

   // let atlas_r = 

    result.f_texcoord =  vec2(a_texcoord.x * tile_scale +  f32(x_row) * tile_scale , a_texcoord.y  *  tile_scale + f32(y_row) * tile_scale); 

    

   // result.f_layer = a_instance_layer;
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
 