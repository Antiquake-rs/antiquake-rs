
## Antiquake Engine 

A game engine written in Rust that leverages quake mod tools such as the PAK file format and the standard Quake map format.  
 
Quake specs 
https://www.gamers.org/dEngine/quake/spec/quake-spec34/index0.htm



## HOW TO RUN WITH DEBUG LOGS
RUST_LOG=debug cargo run


 ### STEPS TO RUN 
 boot with this: cargo run --bin quake-client
 type in console: playdemo demo1  [ENTER]

-- need to fix the cvar registration.  wtf is a cvar -server constant ?  i dont like the system. 


cargo run --debug --bin quake-client



## what is done 
gfx pipelines yay!!!



#### Next Steps 
 
 -add gravity 
 -add jumping 

- fix the main menu  -- also give it a custom bg texture ! 

- come up with a game loop, a game theme and story and design 






 - i think clients need their own local GameStateDelta buffer and a server-based GameStateDelta ( but clients shouldnt apply the same delta twice -- maybe each has an ID )


 -  keep refactoring stuff out of ClientState 
  

- upgrade src/client/mod so that the client has a 'PhysicalGameState' which is a virtual machine that advances by 33ms ticks.   This virtual machine is a replica to that which is on the server.  (see fn frame() )


-client 'parse_server_msg' is very goofy why are 5 raw values being passed in just for that.. 

 
 
 
- Improve the networking code so it is more like QuakeWorld (client side prediction and rubber banding -- clients just need to know abt physics engine and sim it themselves ) 

- Build a toml file that describes what happens when a player joins the server (find them a spawn point and spawn them in -- ? --do this during prespawn for that client --   )
- Make the spawnbaseline command happen automagically from the level state 
- make the set view command give the client their actual client id number not always 1 


 

https://www.gamers.org/dEngine/quake/QDP/qnp.html#connection_req

https://fabiensanglard.net/quakeSource/quakeSourceNetWork.php

 

-fix client state.rs >  fn update_listener , fn calc_final_view
 
 Connection {
            state: ref cl_state,
 


IMPLEMENT MORE SLIMES  


## WORLD 
physics look into https://crates.io/crates/bevy_rapier3d
https://docs.rs/bevy_ecs/latest/bevy_ecs/

ultimately build map like this https://www.youtube.com/watch?v=rp9-q_imCnk


#### NETWORKING 
Associating a remote address with a socket is fine clientside, but serverside it doesn't make sense.

https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.send_to
Reference the real quake server code : https://github.com/id-Software/Quake/blob/master/QW/server/sv_main.c


- client sends unreliables WAYY too often 

- client and server need a proper gamestate tick loop: 
https://stackoverflow.com/questions/28008549/limit-while-loop-to-run-at-30-fps-using-a-delta-variable-c

(i have done this before! )
(maybe we do this w bevy tick timers! )
https://github.com/bevyengine/bevy/blob/main/examples/ecs/fixed_timestep.rs
https://crates.io/crates/game-loop
https://sunjay.dev/learn-game-dev/game-loop.html


 
 
 - fix client/state.rs  788 

 - do not send client cmd to the server !! instead, put that in a buffer which we flush every tick (  flush by applying to our own ECS then tell the server 
)





- turn spawn_entity_from_map -> execute program   back on (for progs.dat)


-Try to import a map from trenchbroom and see if we can load it  
-Add ECS architecture?
 

- STRETCH: improve the engine so that it can run mods effectively

https://www.youtube.com/watch?v=57TKNzYTf5U





## inspiration 
jpiolho/QuakePlugins -- cool system w lua hooks 

https://github.com/Novum/vkQuake/tree/master/Quake

use iced gui ?
add gltf models ?


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



##  MOD FILE FORMAT FYI 

the FGD/DEF are files that describe what entities the progs.dat will accept and use
for map editors, so they know what things do

trenchbroom outputs a map of all entities along with their custom hashmaps of data 

progs.dat has compiled C functions inside , for each entity classname, that help to specify context data about that entity for the engine  (but this paradigm can be swapped out for something else !!) 