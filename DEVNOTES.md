
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

 ServerCmd::LightStyle 



- Parse ogre.toml and preload up our slime context with allllll of that kind of data 
- we will use the raw static data in our slime context to ultimately call methods 
- GOAL: get 'builtin_precache_model' to run bc of things spawning into our map!  (like ogres) 





Server needs to be using the msg_send cache with reliable packet stream 
Client needs to be sending 'MsgKind::Ack '

        //if got an ACK from a client, send to proper q sock so it knows 

        client's  pub fn recv_msg(&mut self, block: BlockingMode has ack 

        servers does not 

        also there is a recv buffer !

        implement 'recv_msg' on the server so it can understand ACKs that come back 





Spoike
 — 
Yesterday at 5:26 PM
it uses a different server udp socket for each client, instead of the one the user origonally connected to. which is stupid and breaks nats
so yeah, client sends ccreq_connect packet, server responds to let the client know which server port to accept packets from (complete poop.)
the server then starts throwing some reliables at you.
and expects some acks.
Spoike
 — 
Yesterday at 5:44 PM
those reliables have various svc stuff
the server expects some specific clc replies at various points.
then it'll start sending some unreliables mixed with the odd reliable. woo. easy, right?...


 

WHERE I AM  STUCK : 
src/client/mod 1377 happens 
  Ok(Connection {
        state: ClientState::new(stream),
        kind: ConnectionKind::Server {
            qsock,
            compose: Vec::new(),
        },
        conn_state: ConnectionState::SignOn(SignOnStage::Prespawn),
    })


https://www.gamers.org/dEngine/quake/QDP/qnp.html#connection_req

https://fabiensanglard.net/quakeSource/quakeSourceNetWork.php



need clients 'self.entities' to be populates !!

-fix client state.rs >  fn update_listener , fn calc_final_view

- need this to fire :  ServerCmd::FastUpdate(ent_update) => { 

    src/client/mod 430 


 Connection {
            state: ref cl_state,



Client is trying todo first render pass and getting 

' panicked at 'send_msg_unreliable_multicast
called `Result::unwrap()` on an `Err` value: NoSuchLightmapAnimation(0)', src\client\render\mod.rs:753:58

Some(Connection {
            state: ref cl_state,
            ref conn_state,
            ref kind,

the connection's cl_state  data is not all filled in yet 



#### NETWORKING 
Associating a remote address with a socket is fine clientside, but serverside it doesn't make sense.

https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.send_to
Reference the real quake server code : https://github.com/id-Software/Quake/blob/master/QW/server/sv_main.c



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