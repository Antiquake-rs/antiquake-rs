




//all the possible things that can affect the physical gamestate !! 
/*


    Unit ID:  The id used by quake protocol for track 'entities' in its domain 

    Entity ID: the id used by bevy in the ECS world 


*/
use std::{fmt, rc::Rc, collections::HashMap, slice::IterMut};
use cgmath::{Deg, Vector3, Angle,InnerSpace};

pub mod resource;
pub mod component;
pub mod system;
pub mod entity;



use crate::server::world::Trace;

use self::system::physics::{ EntityPostureType, PhysMovementType};

#[derive(Clone)]
pub struct GameStateDelta {

    pub command: DeltaCommand,
    pub source_unit_id: usize,  //the quake unit id (not the bevy entity id)

    pub source_player_id: usize, //0 for server 
    pub source_tick_count: usize, 
 
}




impl GameStateDelta{
    pub fn new(delta_cmd:DeltaCommand, source_unit_id:usize, source_player_id:usize,source_tick_count:usize  ) -> GameStateDelta {
        GameStateDelta { 
                command: delta_cmd,
                source_unit_id ,
                source_player_id ,
                source_tick_count  

        }
    }


    pub fn modify_via_collision_trace(self , trace:Trace ) -> GameStateDelta{
        //self.command.modify_via_collision_trace( trace );

        let command_type = &self.command; 

        match command_type {

            DeltaCommand::TranslationMovement (translation)=>  { 
                      
                    let trace_end = trace.end();

                    match trace_end.kind() {

                        
                        crate::server::world::TraceEndKind::Terminal =>  { 
                            
                            let mut result_delta = self.clone();
                            let mut result_translation = translation.clone();

                            result_translation.speed = 0.0; //-1.0 * result_translation.speed.abs();

                            result_delta.command = DeltaCommand::TranslationMovement( result_translation );                            
                            return result_delta;

                          } 
                        crate::server::world::TraceEndKind::Boundary(bound) => {

                            //state_delta.command.

                            let boundary_plane_normal = bound.plane.normal().clone();

                            //need this to fire ! 
                            print!("boundary plane normal {:?}", boundary_plane_normal);
 

                            let mut result_delta = self.clone();
                            let mut result_translation = translation.clone();
                            result_translation.speed = 0.0;
                            result_delta.command = DeltaCommand::TranslationMovement( result_translation );  
                            return result_delta;


                        }
                    }



                 },

            _ => {
                        return self 
                } //do nothing 
        }
    }

}





impl fmt::Display for GameStateDelta {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{} - {} - {} - {}", 
            self.command.to_string(), 
            self.source_unit_id,
            self.source_player_id,
            self.source_tick_count 
         )
    }
}


//these flags limit the commands to 1 per unit per tick 
bitflags! {
pub struct DeltaCommandFlags: u16 {
    const ReportEntityPhys = 1 << 0;
    const TranslationMovement = 1 << 1;
    const ReportLocation = 1 << 2;
    const ReportVelocity = 1 << 3;
}
}


fn gamestate_delta_to_flag_type( delta:&GameStateDelta ) -> Option<DeltaCommandFlags> {
    match(delta.command){
        
        DeltaCommand::ReportEntityPhys { .. } => Some(DeltaCommandFlags::ReportEntityPhys),
        DeltaCommand::TranslationMovement { .. } => Some(DeltaCommandFlags::TranslationMovement),
        DeltaCommand::ApplyForce { .. } => None,
        DeltaCommand::PerformEntityAction { .. } => None,
    }
}

fn should_append_delta(d:&GameStateDelta, unit_cmd_flags: &HashMap<usize,u16> ) -> bool {

    let unit_flags:Option<&u16> = unit_cmd_flags.get(  &d.source_unit_id   );

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
    unit_cmd_flags: HashMap<usize,u16>, //unit_id => current cmd flags 
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

    pub fn iter_mut( &mut self ) -> IterMut< GameStateDelta> {

        return self.deltas.iter_mut();       

    }

    pub fn clear( &mut self )  {

        return self.deltas.clear();       

    }


    pub fn is_empty(&self) -> bool {
        return self.deltas.is_empty()
    }


    fn set_delta_flags(&mut self, d:&GameStateDelta) -> Option<u16> {
        let flag_type = gamestate_delta_to_flag_type(d);
        
        match flag_type {
            Some(f_type) => {
                let existing_flags = match self.unit_cmd_flags.get(&d.source_unit_id) {
                        Some(f) => f.to_owned(),
                        None => 0
                };
 
                let new_flags:u16 = (existing_flags | f_type.bits()); 
                self.unit_cmd_flags.insert(d.source_unit_id, new_flags)
            },
            None => {
                // do nothing as this type of command is not flaggable 
                None
            }
        }
    }



}



#[derive(Clone)]
pub struct AppliedForce {  // f = ma ! 
    pub origin_loc: Vector3<f32>,
    pub acceleration: Vector3<f32> , //not normalized 
    pub phys_move_type: usize ,
    pub unit_mass: f32 
}

impl AppliedForce {

    pub fn get_scaled_force(&self) -> Vector3<f32> {
        let horiz_force_scalar =  self.unit_mass / 100.0 ;
        return Vector3::new( self.acceleration.x * horiz_force_scalar, self.acceleration.y * horiz_force_scalar, self.acceleration.z   )
    }

}

#[derive(Clone)]
pub struct MovementTranslation {
    pub origin_loc: Vector3<f32>,
    pub vector: Vector3<f32> ,
    pub speed: f32,
    pub phys_move_type: usize 
}
/*
    Each 'tick', a client is building an array of entity commands (every 33 ms).  At the end of that tick, 
    the client predictively applies that array of UserCommands to their local physical gamestate
     and broadcasts that array along with the tickNumber to the server.  
     
     The server actually  collects the user commands and appends them to the current 'UserCommandDeltaBuffer' for this tick, even if it is 5 ticks ahead of that client.
     At the end of each tick, the server applies all of the 'UserCommandDeltas' in the buffer to its current gamestate and broadcasts all of those deltas to the other clients 
      (typically only 20 according to valve -- can do filtering based on occlusion )   


      phys_move_type is PhysMovementType but in usize form 

 */
 
#[derive(Clone)]
pub enum DeltaCommand {
    
    ReportEntityPhys { origin: Option<Vector3<f32>> ,velocity: Option<Vector3<f32>>, look: Option<Vector3<Deg<f32>>>   },

    TranslationMovement (MovementTranslation), //vector always normalized to magnitude of 1.  Z is ignored unless you can fly 
    ApplyForce ( AppliedForce ), //used to modify velocity -- typically for gravity and explosions and stuff.  Hitting a wall makes XY velocity go to zero, hitting ground makes Z velocity 0

    PerformEntityAction { action: DeltaAction    },
} 

impl DeltaCommand {

     


}

impl fmt::Display for DeltaCommand {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        
        match self {
         
            DeltaCommand::ReportEntityPhys { .. } => write!(f, "ReportEntityPhys" ),
            DeltaCommand::TranslationMovement {  ..  } =>write!(f, "SetMovementVector" ),
            DeltaCommand::PerformEntityAction { action  } => write!(f, "PerformEntityAction" ),
            DeltaCommand::ApplyForce( .. ) => write!(f, "ApplyForce" ),
        }
    }
}


#[derive(Clone)]
pub enum DeltaAction {

    BeginJump {origin: Vector3<f32> }, //put this here ?  

    Interact { targetId: u32  }, //trigger 

    EquipWeapon { slotId: u32, weaponId: u32 },  //this can affect an entities equipped abilities! But of course in a totally deterministic way.  Usually the slot is always 0 but having 1 would allow dual wield! 

    SetUseWeapon{ weaponId:u32, active:bool }, // 0 for left client, 1 for right click  typically 
   
    ReloadWeapon ,

    EquipAbility {slotId:u32, abilityId:u32},  //kind of vague  --maybe not used at all 
    SetUseAbility {abilityId:u32, active:bool},  //kind of open-ended on purpose 


    SetPosture( EntityPostureType ),

    SetZoomState (bool), //zoomed is true 

    SetPhysMovementType( PhysMovementType )  //only admins can do this ! -- for setting to noclip 

}

