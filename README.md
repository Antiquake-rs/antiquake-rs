
## Soulgate Engine 

A game engine written in Rust that leverages quake mod tools such as the PAK file format and the standard Quake map format.  

Levels can be designed using Trenchbroom editor. 






cargo run --debug --bin quake-client


### Credits 

Rendering libraries taken from  Thinkofname/rust-quake [github]




## Shaders 

https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#writing-the-shaders


rewrite my shaders to put them into wgsl 

wgsl shaders dont need to be compiled like vert-frag ones do ! 



- need to change pipeline  L267  -- the way shaders are created.  


## shaders project ref !
 https://github.com/kvark/baryon
 https://github.com/kvark/vange-rs/tree/master/res/shader/terrain


 def refernce vange-rs to figure out how to implement shader pipeline ! 