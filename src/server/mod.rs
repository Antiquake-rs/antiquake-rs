// Copyright © 2018 Cormac O'Brien.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


/*
    This should use entity-component-system architecture for the entities 
*/


pub mod precache;
 
pub mod world; 
pub mod levelstate;
pub mod slime; 



use std::{
    thread::{self},
    cell::{Ref, RefCell},
    collections::{HashMap,VecDeque},
    rc::Rc,
    net::{ToSocketAddrs,SocketAddr},
    io::{self, BufRead}, 
    fmt::{self,Display}, 
};

use num::FromPrimitive;


use byteorder::{LittleEndian, NetworkEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    common::{
        cvars::{register_cvars},
        bsp::{self},
        console::CvarRegistry,
        engine::{duration_from_f32, duration_to_f32},
        math::Hyperplane,
        model::{Model,ModelFlags,ModelKind},
        parse,
        vfs::{Vfs,VirtualFile},
        default_base_dir,
        net::{
            self, NetError, ServerCmd,  GameType,  SignOnStage, MsgKind,
            server::{ConnectSocket,ServerConnectionManager,ServerQSocket, ClientPacket, Request, Response, ResponseServerInfo, ResponseAccept, SpecialServerAction},
            
        }, 
        util::read_f32_3, 
    },
    
};

 

use self::{
    precache::{Precache,MAX_PRECACHE_ENTRIES,MAX_PRECACHE_PATH},
   
    net::{ ServerCmdCode  },
    world::{ 
      World, EntityId
    },
    levelstate::{LevelState},
    slime::{
        Slime, SlimeError
    },
};
 
use io::BufReader;
use thiserror::Error;
  

use cgmath::{ Vector3, Zero, Deg};
use chrono::Duration;


 


/// The state of a client's connection to the server.
/*pub enum ClientConnectionState {
    /// The client is still connecting.
    Connecting,

    /// The client is active.
    Active,
}*/

pub struct ClientState {  


    client_id: i32, 
    
    /// If true, client may execute any command.
    privileged: bool,

    /// ID of the entity controlled by this client.
    entity_id: Option<EntityId>,

    client_connected: bool,  

    socket_addr: SocketAddr 
}

bitflags! {
    pub struct SessionFlags: i32 {
        const EPISODE_1 =      0x0001;
        const EPISODE_2 =      0x0002;
        const EPISODE_3 =      0x0004;
        const EPISODE_4 =      0x0008;
        const NEW_UNIT =       0x0010;
        const NEW_EPISODE =    0x0020;
        const CROSS_TRIGGERS = 0xFF00;
    }
}

/// A fixed-size pool of client connections.
pub struct ClientSlots {
    /// Occupied slots are `Some`.
    //slots: Vec< ClientState >,

    limit: usize,

    //client_id => clientstate 
    slots: HashMap<i32, Option<ClientState>>


}


/*

You will have just that one UdpSocket on the server, and use it to send traffic to all the clients.
You use this function to do it https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.send_to
*/
impl ClientSlots {
    /// Creates a new pool which supports at most `limit` clients.
    pub fn new(limit: usize) -> ClientSlots {
        let mut slots = HashMap::with_capacity(limit);
        //slots.resize_with(limit, || None);

        ClientSlots { slots, limit }
    }

    pub fn add_client(&mut self, socket_addr:SocketAddr, privileged:bool) -> Result<i32, NetError>  { 
 
        let client_id_result = self.find_next_available_slot_id();

        match client_id_result {
            Some(client_id) => {

                let new_client_state = ClientState { 
                    client_id, 
                    client_connected: false,
                    entity_id: None,
                    socket_addr  ,
                    privileged    
                }; 
        
                self.slots.insert( client_id,  Some(new_client_state) );
                Ok(client_id) 
            },
            None  => {   Err(  NetError::Other(format!( "Server could not add new client")   ) )  } 
        }

       
    }

    /// Returns a reference to the client in a slot.
    ///
    /// If the slot is unoccupied, or if `id` is greater than `self.limit()`,
    /// returns `None`.
    pub fn get(&self, key: &i32) -> Option<&ClientState> {
        self.slots.get(key)?.as_ref()
    }

    /// Returns the maximum number of simultaneous clients.
    pub fn limit(&self) -> usize {
        self.slots.len()
    }

    
    pub fn find_next_available_slot_id(&mut self) -> Option< i32  > {
        //let slot = self.slots.iter_mut().find(|s| s.is_none())?;
        //Some(slot.insert(ClientState::Connecting))

        //loop through each one to find one that is none and return the id 

        let length = self.slots.len();

        if(self.limit <= length) {return None;}

        return Some(length as i32);
    }
}




/// An error returned by a game server.
#[derive(Error, Debug)]
pub enum GameServerError {
    #[error("Unable to load map")]
    MapLoadingError,
    #[error("Unable to load progs.dat")]
    ProgsLoadingError,
    #[error("Unable to load slime.json")]
    SlimeLoadingError,
    #[error("Invalid CD track number")]
    InvalidCdTrack,
    #[error("No such CD track: {0}")]
    NoSuchCdTrack(i32),
    #[error("Message size ({0}) exceeds maximum allowed size {}", net::MAX_MESSAGE)]
    MessageTooLong(u32),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Network error: {0}")]
    Net(#[from] NetError),
}


struct GameServerMessage {
   // view_angles: Vector3<Deg<f32>>,
   // msg_range: Range<usize>,
}


/// A server that yields commands from a demo file.
pub struct GameServer {
   // track_override: Option<u32>,
 

     
    protocol_version: u8,
    port: u32,

    serverConnectionManager: ServerConnectionManager, 

   
    server_session: Session, // may not exist yet  
 

    

}

impl GameServer {

  
    /// Construct a new `GameServer` that loads the specified map.  This runs in a new thread ?
    pub fn new( ) -> Result<GameServer, GameServerError> { 
 
        let max_clients = 1; 

        
        println!("Starting server on port 27500");
        let mut addr = SocketAddr::from(([127, 0, 0, 1], 27500)) ;
        let mut serverConnectionManager = ServerConnectionManager::bind( addr ,max_clients ).unwrap();


 


        let max_clients = 1; 

        Ok(GameServer { 

            port:27500, 
          
            protocol_version: net::PROTOCOL_VERSION,
    
            server_session: Session::new( max_clients ),

            serverConnectionManager,
 

        })
    }

    pub fn loadSlime(&mut self,slime_file_name:String) -> Result< Slime, GameServerError> {
 
        let base_dir = default_base_dir(); 

        let vfs = Rc::new( Vfs::with_base_dir(base_dir ) ) ;  
        
        //load once then its read only 
        let slime =  Slime::load( &vfs , &slime_file_name  ).unwrap();
        
        return Ok(slime)

    }

    pub fn loadMap(&mut self, map_file_name:  String, slime:Slime) -> Result< (), GameServerError> {

         //// do we have to load map here ?  waste since world does it too.. ?
          

            //add map to the world 
         /*   all_models.push(Model{
                name: map_name,
                kind: ModelKind::None,
                flags: ModelFlags::empty()

            }  ); */

                
    

          let base_dir = default_base_dir(); 

          let vfs = Rc::new( Vfs::with_base_dir(base_dir ) ) ;
  

          let loaded_result = self.server_session.load_level( vfs, slime, map_file_name   ) ; 

          match loaded_result {

            Ok(_) => {println!("Server loaded level")}
            Err( error ) => {panic!("{}", error )}
          }

 
          Ok( () )

    }


    //this should now own the reference to self 
    pub fn start( &mut self )   {
 
            
                //do this in a new thread  
           
 

                //this is the main server loop at the moment 
                //need to send a  command to load map 

                //recv_request
                loop {
                    //make sure this is not blocking ? 
                    let msg_result  = self.serverConnectionManager.recv_msg(); 
 

                   match msg_result {

                        Ok((msg, msg_kind_opt,  socket_addr_option)) =>  {


                              let client_packet_result = GameServer::parse_msg_result( msg, msg_kind_opt,  socket_addr_option  );

                              match client_packet_result {
                                Ok((client_packet_opt,socket_addr)) => {

                                    match client_packet_opt {
                                        Some(client_packet) => {
                                            let process_result = self.process_client_packet_action(client_packet, socket_addr); 
                                        }
                                        None => {

                                            debug!("No client packet to process");
                                        }
                                    }
                                    

                                }
                                Err(e) => {

                                    debug!( "{}",e );
                                    //do nothing 
                                }
                              }

                             
                        }

                        Err(error) => { println!("Unable recv msg properly "); }

                    }

                    self.update()
                    
  


                } //loop


               


                    //update -- run ECS system and send messages to all clients as needed 
                  
 

    }


    fn parse_msg_result( msg:Vec<u8>, msg_kind_opt: Option<MsgKind>,  socket_addr_option: Option<SocketAddr> ) ->  Result<(Option<ClientPacket>,SocketAddr) , NetError>  {


        let msg_kind = match msg_kind_opt {
            Some(m) => m,
            None =>  {
                return Err(NetError::InvalidData(format!(
                    "Could not parse msg kind " 
                )))
            }

        };


        let socket_addr = match socket_addr_option {
            Some(s) => s,
            None =>  {
                return Err(NetError::InvalidData(format!(
                    "Could not parse socket addr " 
                )) )
            }
        };



        if(!msg.is_empty()){
            println!("Server is about to deserialize a message with kind {} from a connected client {:02X?}", msg_kind, msg.clone().as_slice() );

                        
            let client_packet_result = ServerConnectionManager::parse_client_packet( msg.as_slice() , msg_kind );

            match client_packet_result {

                Ok(client_packet_opt) =>  {
                    

                    return Ok( (client_packet_opt, socket_addr))
                
                }
                Err(e) =>  return Err( e )
            }

           
 
        }else{
            return Ok((None, socket_addr))
        } 

        
    }
 
   fn deserialize_client_msg<R>(reader: &mut R) -> Result<Option<ServerCmdCode>, NetError>
    where
        R: BufRead + ReadBytesExt,
    {
        println!("deserialize_client_msg");


        let code_num = match reader.read_u8() {
            Ok(c) => c,
            Err(ref e) if e.kind() == ::std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(NetError::from(e)),
        };

        let code = match ServerCmdCode::from_u8(code_num) {
            Some(c) => c,
            None => {
                return Err(NetError::InvalidData(format!(
                    "Invalid server command code: {}",
                    code_num
                )))
            }
        };

        println!("deserialize_client_msg code {}", code_num);

        Ok(Some(code))
    }


    fn register_new_client( &mut self, socketAddr: SocketAddr ) -> Result<i32,NetError> {
        println!(" Registering new client {}",  socketAddr  );


        
        let persist =  &mut self.server_session.persist;
        let add_client_result = persist.client_slots.add_client( socketAddr , true );

        match add_client_result{
            Ok(client_id)=> {
                //is this the best way to do this ? 
                //maybe we do this in a method in the ServerConnectManager
                //need to be very careful of these maps remaining valid as players leave and join 
                self.serverConnectionManager.serverQSockets.insert(client_id , ServerQSocket::new( socketAddr ));
                self.serverConnectionManager.clientRemoteAddrs.insert(  socketAddr, client_id) ; 

             
                return Ok(client_id)
 
 
                }, 
             
            
               Err(_) => {return Err(NetError::Other(format!("Could not register new client"))) }
        }
 

 
    }



    fn get_client_port_from_client_id(client_id:&i32) -> i32{
        return 27500
    }

    fn process_client_packet_action( &mut self, packet:ClientPacket, socket_addr:SocketAddr  ) -> Result< (), NetError>  {

        println!("Server processing client packet : {}", packet.to_string());

        match packet {
            ClientPacket::Connect( request_connect ) => {

                //let (game_name,proto_ver) = request_connect;

                 let client_id = self.register_new_client( socket_addr  )?; 


                 let client_port = GameServer::get_client_port_from_client_id( &client_id  );
  
                        
                        let response = Response::Accept(ResponseAccept { port:client_port } );
                        
                        //this is kind of spaghetti ? 
                        let send_response_result = self.serverConnectionManager.send_response( response , socket_addr );
                        

                        //DO THIS ALL RELIABLY 
                        // send server info 

                        //send model precaches 

                        //send signon value 




                        let level_state_opt = self.server_session.level(); 

                        let level_state = match level_state_opt {
                            Some(lvl) => lvl, 
                            None => {
                                println!("Cannot give level data before level loads");
                                return Err(NetError::InvalidData(format!(
                                    "Cannot give level data before level loads" 
                                )))
                            }

                        };

                        println!("models are");
                        let mut world_models:Vec<String> = level_state.get_all_world_model_names() ; 

                        println!("sounds are");
                        /*let mut sound_precache:Vec<String> = level_state.get_sound_precache_data().iter().map(
                            
                            |&s|  {println!("{}",s);  s.into()}
                        ).collect(); */
 
                                
                        let serverInfoCmd = ServerCmd::ServerInfo {
                            protocol_version: i32::from(self.protocol_version),
                            max_clients: (self.server_session.persist.getMaxClients() as u8),
                            game_type: GameType::SinglePlayer,
                            message: String::from("Test message"),
                            model_precache : world_models, 
                            sound_precache : vec![String::from("player/death1.wav"), String::from("player/death2.wav")]   
                        };    
 

                        println!("sending server cmd {}", serverInfoCmd.to_string()  );


                        let send_client_serverinfo_result = self.serverConnectionManager.send_cmd_to_client_reliable( 

                            serverInfoCmd,
                            client_id 
                        
                        );
                        


                        //Got server cmd CdTrack { track: 5, loop_: 5 }
                       // Got server cmd SetView { ent_id: 1 }
                        
                        let signonCmd = ServerCmd::SignOnStage {
                            stage: SignOnStage::Prespawn
                        };    
 
                        let send_client_signon_result = self.serverConnectionManager.send_cmd_to_client_reliable( 

                            signonCmd,
                            client_id 
                        
                        );

                        //client should send us back a prespawn packet and that is when we send them lighting and statics 




                        return Ok(())
                     
                },


                ClientPacket::ServerInfo( request_server_info ) => { 
                    println!("client packet- server info");
                    return Ok(())

                 },
                ClientPacket::PlayerInfo( request_player_info ) => { 
                    println!("client packet- player info");
                    return Ok(())

                 },
                ClientPacket::RuleInfo( request_rule_info ) => { 
                    //the client is asking us for all of the rules of the match -- we give tham and step them thru signon 

                    let prev_cvar = request_rule_info.prev_cvar;

                    let client_id_result = self.serverConnectionManager.get_client_id_from_address( socket_addr  ) ; 

                    let client_id:i32 = match client_id_result {

                        Some(c) => c,
                        None => {

                            return Err(NetError::Other(format!(
                               "Could not find client id for client packet"
                            )))
                        }
 
                    };

                    let level_state_opt = self.server_session.level(); 

                    let level_state = match level_state_opt {
                        Some(lvl) => lvl, 
                        None => {
                            return Err(NetError::InvalidData(format!(
                                "Cannot give level data before level loads" 
                            )))
                        }

                    };


                    match prev_cvar.as_str() {

                        
                      

                        "prespawn" => {
                            // we should send out tons of stuff 

                            println!("sending the player tons of stuff here !!");

                            // give SpawnStaticSound 
                            // give SpawnStatic
                            // give SpawnBaseline 
                           


                            //for each entity 

                    /*        let entities_list:Vec<HashMap<&str, &str>> = level_state.get_entity_list() ;


                            for entity in entity_list {
                                level.spawn_entity_from_map(entity).unwrap();
                            }
                            
                            let mapped = entities_list.iter().map(|ent|
 
                                
                                for (key, value) in  ent {
                                    println!("{} / {}", key, value);
                                }
                            );*/ 

 
                          /*   let spawnBaselineCmd = ServerCmd::SpawnBaseline {
                                ent_id,
                                model_id,
                                frame_id,
                                colormap,
                                skin_id,
                                origin,
                                angles,
                            };   */


                            let signonCmd = ServerCmd::SignOnStage {
                                stage: SignOnStage::ClientInfo
                            };    
     
                            let send_client_signon_result = self.serverConnectionManager.send_cmd_to_client_reliable( 
    
                                signonCmd,
                                client_id 
                            
                            );

                        }


                        "clientinfo" => {
                            /*
                                give time,
                                update names , frag, color 
                              

                                give light styles 
                                give update stat
                                set angle

                                get playerdata 
                                 
                            */

                            //give light maps
                            for id in 0..64 {
                                
                                let send_client_signon_result = self.serverConnectionManager.send_cmd_to_client_reliable( 
    
                                    ServerCmd::LightStyle {
                                       id,
                                       value: String::from("") //what do we put here ?
                                    }   ,
                                    client_id 
                                
                                );

                            }


                            let signonCmd = ServerCmd::SignOnStage {
                                stage: SignOnStage::Begin
                            };    
     
                            let send_client_signon_result = self.serverConnectionManager.send_cmd_to_client_reliable( 
    
                                signonCmd,
                                client_id 
                            
                            );




                            //then our fast updates will put the clinets signonstage into done  -- then they will render the map

                        }

                        "begin" => {
                            println!(" client is ready to begin ");

                            //is this right ?
                            let signonCmd = ServerCmd::SignOnStage {
                                stage: SignOnStage::Done
                            };    
     
                            let send_client_signon_result = self.serverConnectionManager.send_cmd_to_client_reliable( 
    
                                signonCmd,
                                client_id 
                            
                            );

                        }


                        _ => println!("cant give rule info for unknown - {}" , prev_cvar),

                    }



 
                    return Ok(())

                 },


                ClientPhysicsState => {
                    println!("client packet- client phys ");
                    return Ok(())

                }
 
               
           /* DisconnectClient => {
                Ok(())
            }*/

        }
    }

        /*
        let response = match request {

            Request::Connect( _ ) => {    

                let client_port_result = self.register_new_client( socketAddr  );

                match client_port_result {
                    Ok(client_port) => {  return Ok(  Response::Accept(ResponseAccept { port:client_port }) ) }
                    NetError => { return Err(NetError::Other(format!("Could not register new client")))}
                }

                //this is  going to the client and properly turning into q socket 
                
            },

            Request::ServerInfo(_) => {

                // ServerCmdCode::ServerInfo
                
                // let packet = response_server_info.to_bytes().unwrap();

                //need to send a  ServerCmd::ServerInfo ! 
                return Ok(
                    Response::ServerInfo(ResponseServerInfo {
                    address: String::from("127.0.0.1"),
                    hostname: String::from("localhost"),
                    levelname: String::from("e1m1"),
                    client_count: 1,
                    client_max: 16,
                    protocol_version: 15,
                    } ) 
                )

            },
            Request::PlayerInfo(_) => {

                return Err(NetError::Other(format!("Not Implemented:PlayerInfo")));
            },
            Request::RuleInfo(_) => {

                return Err(NetError::Other(format!("Not Implemented:RuleInfo")));
            },
            

        };*/
 

    


    fn update( &mut self ){

        println!("server is updating");

        self.serverConnectionManager.update();

       

    }


}




/*


MOVE ALL BELOW TO A FILE NAMED SESSION 

*/


#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Unable to load map")]
    MapLoadingError,
    #[error("Unable to load progs.dat")]
    ProgsLoadingError,
    #[error("Unable to load slime.json")]
    SlimeLoadingError,
    #[error("Invalid CD track number")]
    InvalidCdTrack,
    #[error("No such CD track: {0}")]
    NoSuchCdTrack(i32),
    #[error("Message size ({0}) exceeds maximum allowed size {}", net::MAX_MESSAGE)]
    MessageTooLong(u32),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Network error: {0}")]
    Net(#[from] NetError),
}



/// Server state that persists between levels.
pub struct SessionPersistent {
    client_slots: ClientSlots,
    flags: SessionFlags,
}

impl SessionPersistent {
    pub fn new(max_clients: usize) -> SessionPersistent {
        SessionPersistent {
            client_slots: ClientSlots::new(max_clients),
            flags: SessionFlags::empty(),
        }
    }
 

    pub fn getMaxClients(&self) -> usize {

        return self.client_slots.limit();
    }

    pub fn client(&self, slot: &i32) -> Option<&ClientState> {
        self.client_slots.get(slot)
    }
}

/// The state of a server.
pub enum SessionState {

    //the server is starting -- only has persistant state 
    Starting(),

    /// The server is loading.
    ///
    /// Certain operations, such as precaching, are only permitted while the
    /// server is loading a level.
    Loading(SessionLoading),

    Preparing(), //handoff between loading and active 

    /// The server is active (in-game).
    Active(SessionActive),
}

/// Contains the state of the server during level load.
pub struct SessionLoading {
    level: LevelState,
}

impl SessionLoading {
    pub fn new(
        vfs: Rc<Vfs>,
        cvars: Rc<RefCell<CvarRegistry>>,
        slime: Slime,
       
        models: Vec<Model>,
        entmap: String,
    ) -> SessionLoading {

        //are these still the best inputs ? 
        SessionLoading {
            level: LevelState::new(vfs, cvars, slime, models, entmap),
        }
    }

    /// Adds a name to the sound precache.
    ///
    /// If the sound already exists in the precache, this has no effect.
    #[inline]
    pub fn precache_sound(&mut self, name: String)   {
        self.level.precache_sound(name)
    }

    /// Adds a name to the model precache.
    ///
    /// If the model already exists in the precache, this has no effect.
    #[inline]
    pub fn precache_model(&mut self, name: String)   {
        self.level.precache_model(name)
    }

     
    /// This consumes the `ServerLoading` and returns a `ServerActive`.
    pub fn finishLoading(self) -> SessionActive {
        SessionActive { level: self.level }
    }

 
}

/// State specific to an active (in-game) server.
pub struct SessionActive {
    level: LevelState,
}

/// A server instance.
pub struct Session {
    persist: SessionPersistent,
    state: SessionState,
}

impl Session {
    pub fn new(
        max_clients: usize 
    ) -> Session {
        Session {
            persist: SessionPersistent::new(max_clients),
            state: SessionState::Starting(),
        }
    }

    pub fn load_level( 
        &mut self,
        vfs: Rc<Vfs>,
        slime: Slime,
        map_file_name: String, 
 
    ) -> Result<(),SessionError> {
      
            
        let mut map_file = match vfs.as_ref().open( map_file_name )  {
            Ok(f) => f,
            Err(e) => return  Err( SessionError::MapLoadingError   )
        };  


        let (mut brush_models, mut entmap) = bsp::load(map_file).unwrap();
            
              
       

      //  let slime = Slime::load(vfs.as_ref(), "slime.toml").unwrap();
            
 

        let con_names = Rc::new(RefCell::new(Vec::new()));    

        //populate cvars from slime ??

        let cvars = Rc::new(RefCell::new(CvarRegistry::new(con_names.clone())));
        // what is this doing and why ? 
        register_cvars(&cvars.borrow()).unwrap();
        


        //do i need to give the level more than just brushmodels ? how does it get entity models..? the progs slime ? 

        //prep for precaching 
        self.state =  SessionState::Loading(  SessionLoading::new(vfs, cvars, slime, brush_models, entmap) );
 


        //set self state to preparing to do the handoff of the level to active 
        let loaded_state = std::mem::replace(&mut self.state, SessionState::Preparing());

        let session_active_opt = match loaded_state {
            SessionState::Loading(state) => Some(state.finishLoading()),
            _ => None,
            };

        let session_active = match session_active_opt {
            Some(act) => act,
            None => panic!("Could not finish loading level")
        };

        self.state = SessionState::Active( session_active );

            //precache !! map and  brush and entity models for sure 

            //how does precaching work !? how do i get the map in there 
        /*
          self.precache_model(map_file_name);
  
          //get these from the slime ? 
          self.precache_sound("player/death1.wav");
            */
 

        //FIX ME LATER 

        //    self.state =  SessionState::Active(self.finishLoading());

         Ok(())

    }

       /// Completes the loading process.
    ///
    /// This consumes the `ServerLoading` and returns a `ServerActive`.
  /* pub fn finishLoading( &self)  -> SessionActive {

        let session_active_opt = match &self.state {
            SessionState::Starting() => None,
            SessionState::Loading(state) => Some(state.finishLoading()),
            SessionState::Active(state) => None
            };
            
            //self.state.finishLoading();

        let session_active = match session_active_opt {
            Some(act) => act,
            None => panic!("Could not finish loading level")
        };
 

        //self.state = SessionState::Active( session_active );
        return session_active
        
    }*/  


    /// Returns the maximum number of clients allowed on the server.
    pub fn max_clients(&self) -> usize {
        self.persist.client_slots.limit()
    }

    #[inline]
    pub fn client(&self, slot: &i32) -> Option<&ClientState> {
        self.persist.client(&slot)
    }

    pub fn precache_sound(&mut self, name: String) {
        if let SessionState::Loading(ref mut loading) = self.state {
            loading.precache_sound(name);
        } else {
            panic!("Sounds cannot be precached after loading");
        }
    }

    pub fn precache_model(&mut self, name: String) {
        if let SessionState::Loading(ref mut loading) = self.state {
            loading.precache_model(name);
        } else {
            panic!("Models cannot be precached after loading");
        }
    }

    #[inline]
    fn level(&self) -> Option< &LevelState > {
        match self.state {
            SessionState::Starting() => None,
            SessionState::Loading(ref loading) => Some(&loading.level),
            SessionState::Preparing() => None,
            SessionState::Active(ref active) => Some(&active.level),
        }
    }

    #[inline]
    fn level_mut(&mut self) -> Option< &mut LevelState > {
        match self.state {
            SessionState::Starting() => None,
            SessionState::Loading(ref mut loading) => Some(&mut loading.level),
            SessionState::Preparing() => None,
            SessionState::Active(ref mut active) => Some(&mut active.level),
        }
    }

    #[inline]
    pub fn sound_id(&self, name: String) -> Option<usize> {
 
   
        self.level()?.sound_id(name)

    }

    #[inline]
    pub fn model_id(&self, name: String) -> Option<usize> {

        self.level()?.model_id(name)

    }

    #[inline]
    pub fn set_lightstyle(&mut self, index: usize, val: String ) {
        match  self.level_mut() {
            Some(lvl) => {

                lvl.set_lightstyle(index, val);
            },
            None => { debug!( "Cannot set light style on a null level " ); }

        }
        

    }

    /// Returns the amount of time the current level has been active.
    #[inline]
    pub fn time(&self) -> Option<Duration> {
        match self.state {
            SessionState::Starting() => None,
            SessionState::Loading(_) => None,
            SessionState::Preparing() => None,
            SessionState::Active(ref active) => Some(active.level.get_time()),
        }
    }
}
