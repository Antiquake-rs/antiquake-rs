
## Antiquake Engine 

A game engine written in Rust that leverages quake mod tools such as the PAK file format and the standard Quake map format.  
 

## HOW TO RUN 
RUST_LOG=debug cargo +nightly run


 ### STEPS TO RUN 
 boot with this: cargo +nightly run --bin quake-client
 type in console: playdemo demo1




cargo run --debug --bin quake-client



## what is done 
gfx pipelines yay!!!


#### Next Steps 

-Try to import a map from trenchbroom and see if we can load it like an indie game 
-Add ECS architecture?
-ability to load a pak-zip (pak3) 

-basically improve the engine so that an indie game will run in it 




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

  
 ## probable bugs 
 client/input/game -> line 354 -> mousewheels 

 
 
 

## Spells System

10:00 time 
https://www.youtube.com/watch?v=Lv6WEFGzqNQ


## stat system
game_stat  (by tantan)


## entity comp arch

https://www.youtube.com/watch?v=oHYs-UqS458&list=PL0rDS3s8z_DBdjxl0GK87p1rFZ5c2Fz1e&index=12
-build an ECS registry like at end of video 
-load stuff from config files 