
## Antiquake Engine 

A game engine written in Rust that leverages quake mod tools such as the PAK file format and the standard Quake map format.  
 

## HOW TO RUN WITH DEBUG LOGS
RUST_LOG=debug cargo run


 ### STEPS TO RUN 
 boot with this: cargo run --bin quake-client
 type in console: playdemo demo1  [ENTER]




cargo run --debug --bin quake-client



## what is done 
gfx pipelines yay!!!


#### Next Steps 
-make this not require nightly (see richter draft pr )(https://github.com/cormac-obrien/richter/pull/48)

-Try to import a map from trenchbroom and see if we can load it like an indie game 
-Add ECS architecture?
-ability to load a pak-zip (pak3) 

-basically improve the engine so that an indie game will run in it 

https://www.youtube.com/watch?v=57TKNzYTf5U




### Credits 

Rendering libraries taken from  richter


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