
## Antiquake Engine 

A game engine written in Rust that leverages quake mod tools such as the PAK file format and the standard Quake map format.  

Levels can be designed using Trenchbroom editor. 


## HOW TO RUN 
RUST_LOG=debug cargo +nightly run


 ### STEPS TO RUN 
 boot with this: cargo +nightly run --bin quake-client
 type in console: playdemo demo1




cargo run --debug --bin quake-client



#### 
need to fix the git commit history ...  just clone richter again and cop ythese files  on it 




### Credits 

Rendering libraries taken from  richter


## GOOD SHADER INFO 
https://anteru.net/blog/2016/mapping-between-HLSL-and-GLSL/
https://app.element.io/#/room/#wgpu:matrix.org
Replace .stpq with .xyzw  (they are the same) 



#### pipelines 
refactor pipelines in client.render/mod so they are more like ... components that share a single 'interface' and are in an array (registered) that just gets looped through .         




 

## resources 
https://www.models-resource.com/pc_computer/quake/model/33486/


## use this for ecs 
https://www.youtube.com/watch?v=oHYs-UqS458&t=2077s


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


## inspiration 
jpiolho/QuakePlugins -- cool system w lua hooks 

 ### Shader files to rewrite !!
 -sprite.wgsl
 -quad.wgsl
 -postprocess.wgsl
 -particle.wgsl
 -glyph.wgsl
 -deferred.wgsl 
 -brush.wgsl
 -blit.wgsl 
 -alias.wgsl 


 ## probable bugs 
 client/input/game -> line 354 -> mousewheels 



-swapchain became texture view. i think that is what the pipeline writes to.  its totally jacked up .



## TODO 
fix renderer -> push constants -> https://github.com/gfx-rs/naga/blob/master/tests/in/push-constants.wgsl


float is not provided by the pipeline !! 





## Spells System

10:00 time 
https://www.youtube.com/watch?v=Lv6WEFGzqNQ


## stat system
game_stat  (by tantan)


## entity comp arch

https://www.youtube.com/watch?v=oHYs-UqS458&list=PL0rDS3s8z_DBdjxl0GK87p1rFZ5c2Fz1e&index=12
-build an ECS registry like at end of video 
-load stuff from config files 