




//all the possible things that can affect the physical gamestate !! 

pub enum GameStateDelta {

    EntityCommand, 

}

/*
    Each 'tick', a client is building an array of entity commands (every 33 ms).  At the end of that tick, 
    the client predictively applies that array of UserCommands to their local physical gamestate
     and broadcasts that array along with the tickNumber to the server.  
     
     The server actually  collects the user commands and appends them to the current 'UserCommandDeltaBuffer' for this tick, even if it is 5 ticks ahead of that client.
     At the end of each tick, the server applies all of the 'UserCommandDeltas' in the buffer to its current gamestate and broadcasts all of those deltas to the other clients 
      (typically only 20 according to valve -- can do filtering based on occlusion )   

 */

pub enum EntityCommand {
    ReportLocationVector { loc: Vector3(f32) },  //used by clients to tell the server where they THINK they are, and by the server to tell clients where they ACTUALLY are -- rubberband them back
    ReportVelocityVector { angle: Vector3<Deg>} , // used by the server to tell clients the ACTUAL entity velocity (incase an entity gets thrown by explosion ,etc)

    SetLookVector { angle: Vector3<Deg>},//always normalized to magnitude of 1
    SetMovementVector { angle: Vector3<Deg>}, //always normalized to magnitude of 1.  Z is ignored unless you can fly ? 

    PerformEntityAction { action: EntityAction },
}


pub enum EntityAction {

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
 