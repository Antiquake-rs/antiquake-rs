
 

 ## SHADER TODO 

convert particle color to not use push constants and instead use uniform buffers

 

## GOOD SHADER INFO 
https://anteru.net/blog/2016/mapping-between-HLSL-and-GLSL/
https://app.element.io/#/room/#wgpu:matrix.org
Replace .stpq with .xyzw  (they are the same) 

 

## Shaders 

https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#writing-the-shaders


rewrite my shaders to put them into wgsl 

wgsl shaders dont need to be compiled like vert-frag ones do ! 


 

## shaders project ref !
 https://github.com/kvark/baryon
 https://github.com/kvark/vange-rs/tree/master/res/shader/terrain
 https://austin-eng.com/webgpu-samples/samples/deferredRendering
 https://github.com/hecrj/wgpu_glyph wgpu glyph 
 https://github.com/austinEng/webgpu-samples/blob/main/src/sample/particles/main.ts wgpu particles ! 

wgpu examples mipmap 


 
READ THIS ABT SHADER S 
https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#how-do-we-use-the-shaders
https://www.w3.org/TR/WGSL/




 ### GLSL -> WGPU 

  Replace .stpq with .xyzw  (they are the same) 

  TexelLoad  ->  TextureLoad 
        - you can just throw away the sampler since that is not used anyways 



#### This becomes this 
layout(set = 0, binding = 1) uniform texture2D u_texture[256];

binding_array<texture_2d<f32>>




