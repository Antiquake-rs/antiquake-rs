
## Antiquake Engine 

A game engine written in Rust that leverages quake mod tools such as the PAK file format and the standard Quake map format.  
 

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
- be able to load a map  by spinning up a local server on port 27500 
- clean up  src/server/mod  -- it is very large and messy  (specifically trait GameServer)


thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Os { code: 10048, kind: AddrInUse, message: "Only one usage of each socket address (protocol/network address/port) is normally permitted." })', src\server\mod.rs:283:82


#### NETWORKING 
Associating a remote address with a socket is fine clientside, but serverside it doesn't make sense.

https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.send_to




-now that client can load into their own local gameserver, make them spawn and render the map! 
-server needs to tell client to open the map bsp 



__ 
On client/mod  L1370 , 
state is a new stream 
client.connection.conn_state =  ConnectionState::SignOn(SignOnStage::Prespawn),


client/mod/308  is parse_server_msg   -- where client is handling qosck messages ( ithink) 







- turn spawn_entity_from_map -> execute program   back on (for progs.dat)
-complete the progs.dat engine ?  or build something else ??  (depends on trenchbroom output)



-Try to import a map from trenchbroom and see if we can load it  
-Add ECS architecture?
 

- STRETCH: improve the engine so that it can run mods effectively

https://www.youtube.com/watch?v=57TKNzYTf5U





## inspiration 
jpiolho/QuakePlugins -- cool system w lua hooks 

https://github.com/Novum/vkQuake/tree/master/Quake

  
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