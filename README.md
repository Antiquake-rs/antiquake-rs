
## Soulgate Engine 

A game engine written in Rust that leverages quake mod tools such as the PAK file format and the standard Quake map format.  

Levels can be designed using Trenchbroom editor. 


## HOW TO RUN 
RUST_LOG=debug cargo +nightly run




cargo run --debug --bin quake-client


### Credits 

Rendering libraries taken from  Thinkofname/rust-quake [github]




## Shaders 

https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#writing-the-shaders


rewrite my shaders to put them into wgsl 

wgsl shaders dont need to be compiled like vert-frag ones do ! 




 

## resources 
https://www.models-resource.com/pc_computer/quake/model/33486/


## use this for ecs 
https://www.youtube.com/watch?v=oHYs-UqS458&t=2077s


## shaders project ref !
 https://github.com/kvark/baryon
 https://github.com/kvark/vange-rs/tree/master/res/shader/terrain

wgpu examples mipmap 



 def refernce vange-rs to figure out how to implement shader pipeline ! 




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
