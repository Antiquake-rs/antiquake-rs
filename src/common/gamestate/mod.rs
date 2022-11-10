




//all the possible things that can affect the physical gamestate !! 
/*


    Unit ID:  The id used by quake protocol for track 'entities' in its domain 

    Entity ID: the id used by bevy in the ECS world 


*/
use std::{fmt, rc::Rc, collections::HashMap};
use cgmath::{Deg, Vector3, Angle,InnerSpace};


pub mod system;

use self::system::physics::{ EntityPostureType, PhysBodyType};

pub struct GameStateDelta {

    pub command: DeltaCommand,
    pub source_entity_id: u32, 

    pub source_player_id: u32, //0 for server 
    pub source_tick_count: u32, 



}




impl GameStateDelta{
    pub fn new(delta_cmd:DeltaCommand, source_entity_id:u32, source_player_id:u32,source_tick_count:u32  ) -> GameStateDelta {
        GameStateDelta { 
                command: delta_cmd,
                source_entity_id ,
                source_player_id ,
                source_tick_count  

        }
    }  
}




impl fmt::Display for GameStateDelta {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{} - {} - {} - {}", 
            self.command.to_string(), 
            self.source_entity_id,
            self.source_player_id,
            self.source_tick_count 
         )
    }
}


bitflags! {
pub struct DeltaCommandFlags: u16 {
    const SetLookVector = 1 << 0;
    const SetMovementVector = 1 << 1;
    const ReportLocationVector = 1 << 2;
    const ReportVelocityVector = 1 << 3;
}
}


fn gamestate_delta_to_flag_type( d:&GameStateDelta ) -> Option<DeltaCommandFlags> {
    match(d.command){
        DeltaCommand::ReportLocationVector { .. } => Some( DeltaCommandFlags::ReportLocationVector),
        DeltaCommand::ReportVelocityVector { .. } => Some( DeltaCommandFlags::ReportVelocityVector),
        DeltaCommand::SetLookVector { .. } => Some(DeltaCommandFlags::SetLookVector),
        DeltaCommand::SetMovementVector { .. } => Some(DeltaCommandFlags::ReportVelocityVector),
        DeltaCommand::PerformEntityAction { .. } => None,
    }
}

fn should_append_delta(d:&GameStateDelta, unit_cmd_flags: &HashMap<u32,u16> ) -> bool {

    let unit_flags:Option<&u16> = unit_cmd_flags.get(  &d.source_entity_id );

    match unit_flags {
        Some(u_flags) => {

            let flag_type = gamestate_delta_to_flag_type(d);
            
            match flag_type {
                Some(f_type) => {
                    return u_flags & f_type.bits() == 0; 
                }
                None => return true
            }
        },
        None => return true
    }

}

pub struct GameStateDeltaBuffer {
    //put big arrays in a box so they dont overflow our stack 
    deltas: Box<Vec<GameStateDelta>>,
    unit_cmd_flags: HashMap<u32,u16>, //prevents from too many redundant cmds !
    capacity: i16 //not used for now 

}

impl GameStateDeltaBuffer {

    pub fn new() -> GameStateDeltaBuffer{
        GameStateDeltaBuffer {
            deltas: Box::new( Vec::new() ),
            unit_cmd_flags: HashMap::new(),
            capacity:100
        }
    }

    pub fn reset_flags(&mut self){
        self.unit_cmd_flags.clear();
    }


    //there should be a way this respects flags and a way it doesnt
    pub fn push( &mut self, d: GameStateDelta  ){

        if should_append_delta( &d , &self.unit_cmd_flags ) {
            self.set_delta_flags( &d );
            self.deltas.push(d);
        } 

    }

    pub fn pop( &mut self ) -> Option<GameStateDelta> {

        return self.deltas.pop();       

    }

    pub fn is_empty(&self) -> bool {
        return self.deltas.is_empty()
    }


    fn set_delta_flags(&mut self, d:&GameStateDelta) -> Option<u16> {
        let flag_type = gamestate_delta_to_flag_type(d);
        
        match flag_type {
            Some(f_type) => {
                let existing_flags = match self.unit_cmd_flags.get(&d.source_entity_id) {
                        Some(f) => f.to_owned(),
                        None => 0
                };
 
                let new_flags:u16 = (existing_flags | f_type.bits()); 
                self.unit_cmd_flags.insert(d.source_entity_id, new_flags)
            },
            None => {
                // do nothing as this type of command is not flaggable 
                None
            }
        }
    }



}


/*
    Each 'tick', a client is building an array of entity commands (every 33 ms).  At the end of that tick, 
    the client predictively applies that array of UserCommands to their local physical gamestate
     and broadcasts that array along with the tickNumber to the server.  
     
     The server actually  collects the user commands and appends them to the current 'UserCommandDeltaBuffer' for this tick, even if it is 5 ticks ahead of that client.
     At the end of each tick, the server applies all of the 'UserCommandDeltas' in the buffer to its current gamestate and broadcasts all of those deltas to the other clients 
      (typically only 20 according to valve -- can do filtering based on occlusion )   

 */
 
pub enum DeltaCommand {
    ReportLocationVector { loc: Vector3<f32>   },  //used by clients to tell the server where they THINK they are, and by the server to tell clients where they ACTUALLY are -- rubberband them back
    ReportVelocityVector { angle: Vector3< f32 >   } , // used by the server to tell clients the ACTUAL entity velocity (incase an entity gets thrown by explosion ,etc)

    SetLookVector { angle: Vector3<Deg<f32>>    },//always normalized to magnitude of 1
    SetMovementVector { vector: Vector3<f32>   }, //always normalized to magnitude of 1.  Z is ignored unless you can fly ? 

    PerformEntityAction { action: DeltaAction , target_id: u32  },
} 



impl fmt::Display for DeltaCommand {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        
        match self {
            DeltaCommand::ReportLocationVector { loc } =>write!(f, "ReportLocationVector" ),
            DeltaCommand::ReportVelocityVector { angle } => write!(f, "ReportVelocityVector" ),
            DeltaCommand::SetLookVector { angle } => write!(f, "SetLookVector" ),
            DeltaCommand::SetMovementVector { vector } =>write!(f, "SetMovementVector" ),
            DeltaCommand::PerformEntityAction { action, target_id } => write!(f, "PerformEntityAction" ),
        }
    }
}



pub enum DeltaAction {

    BeginJump,


    EquipWeapon { slotId: u32, weaponId: u32 },  //this can affect an entities equipped abilities! But of course in a totally deterministic way.  Usually the slot is always 0 but having 1 would allow dual wield! 

    SetUseWeapon{ weaponId:u32, active:bool }, // 0 for left client, 1 for right click  typically 
   
    ReloadWeapon ,

    EquipAbility {slotId:u32, abilityId:u32},  //kind of vague  --maybe not used at all 
    SetUseAbility {abilityId:u32, active:bool},  //kind of open-ended on purpose 


    SetPosture( EntityPostureType ),

    SetZoomState (bool), //zoomed is true 

    SetPhysBodyType( PhysBodyType )  //only admins can do this !

}

