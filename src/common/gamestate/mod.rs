




//all the possible things that can affect the physical gamestate !! 
/*


    Unit ID:  The id used by quake protocol for track 'entities' in its domain 

    Entity ID: the id used by bevy in the ECS world 


*/
use std::{fmt, rc::Rc};
use cgmath::{Deg, Vector3};

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



pub struct GameStateDeltaBuffer {
    //put big arrays in a box so they dont overflow our stack 
    pub deltas: Box<Vec<GameStateDelta>>

}

impl GameStateDeltaBuffer {

    pub fn new() -> GameStateDeltaBuffer{
        GameStateDeltaBuffer {
            deltas: Box::new( Vec::new() )
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


pub enum EntityPostureType {
    Stand,
    Crouch,
    Prone 
}

pub enum PhysBodyType {
    Walk,
    Hover,
    Fly,
    NoClip
}
 
pub fn movementIsConstrainedFlat(physBodyType: PhysBodyType) -> bool{

    match physBodyType {
        PhysBodyType::Walk => return true,
        PhysBodyType::Hover => return true,
        PhysBodyType::Fly => return false,
        PhysBodyType::NoClip => return false,
    }
}
pub fn physBodyHasCollision(physBodyType: PhysBodyType) -> bool{

    match physBodyType {
        PhysBodyType::Walk => return true,
        PhysBodyType::Hover => return true,
        PhysBodyType::Fly => return true,
        PhysBodyType::NoClip => return false,
    }
}