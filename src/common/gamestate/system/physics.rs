use bevy_ecs::system::{Query, Res};
use cgmath::{Vector3, Deg, Angle, InnerSpace};

use crate::common::gamestate::{component::physics::{PhysicsComponent}, GameStateDeltaBuffer, GameStateDelta, DeltaCommand};
 

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
 

//consider moving all this to the physics component system !! 

pub fn movement_constrained_flat(physBodyType: PhysBodyType) -> bool{

    match physBodyType {
        PhysBodyType::Walk => return true,
        PhysBodyType::Hover => return true,
        PhysBodyType::Fly => return false,
        PhysBodyType::NoClip => return false,
    }
}
pub fn body_has_collision(physBodyType: PhysBodyType) -> bool{

    match physBodyType {
        PhysBodyType::Walk => return true,
        PhysBodyType::Hover => return true,
        PhysBodyType::Fly => return true,
        PhysBodyType::NoClip => return false,
    }
}

pub fn euler_angles_to_cartesian(pitch:Deg<f32>,yaw:Deg<f32>,roll:Deg<f32> ) -> Vector3<f32> {

    return Vector3::new( yaw.cos() * pitch.cos(), yaw.sin()*pitch.cos(), pitch.sin() );
    
}

pub fn calc_movement_vector( input_cmds: Vector3<i16>, facing: Vector3<Deg<f32>>, physBodyType: PhysBodyType) -> Option<Vector3<f32>>{
 
        
    //pitch roll yaw 
    let forward_dir = euler_angles_to_cartesian(facing.x,facing.y,facing.z) ;

    let forward_dir_normalized = match movement_constrained_flat(physBodyType){
        true => {
            Vector3::new(forward_dir.x as f32,forward_dir.y as f32,0.0).normalize()
        },
        false => {
            Vector3::new(forward_dir.x as f32,forward_dir.y as f32,forward_dir.z as f32).normalize()
        }        
    };


    let up_vector = Vector3::new(0.0,0.0,1.0);
    let sideways_dir = forward_dir_normalized.cross(up_vector);

  

    let forward_movement = forward_dir_normalized * (input_cmds.x as f32);
    let sideways_movement = sideways_dir * (input_cmds.y as f32);

    let overall_movement = (forward_movement + sideways_movement).normalize();
 
    if !overall_movement.x.is_nan() && !overall_movement.y.is_nan() && !overall_movement.z.is_nan() {
        return Some(overall_movement) 
    }

    return None 
    
}




    //ecs help https://bevy-cheatbook.github.io/programming/queries.html



//this is called now 
pub fn update_physics_movement(
    // unit id registry 
    mut gamestateDeltaBuffer: Res<GameStateDeltaBuffer>,
    mut query: Query<(&mut PhysicsComponent)> 
){


    while  !gamestateDeltaBuffer.is_empty()   { 
        
        let next_delta = gamestateDeltaBuffer.pop();
        
        match next_delta {
            Some(delta) => {

                let unit_id = next_delta.source_unit_id; 

                 

                query.get_component(entity)
                
                self::apply_gamestate_delta_buffer(   next_delta, phys_comp  );
        

            }
            _ => {}

        }
     

    }

    gamestateDeltaBuffer.reset_flags();

 

}

fn apply_gamestate_delta_buffer( 
    delta:  &GameStateDelta ,
    mut physComp: &PhysicsComponent
 ){


    match delta.command {
        DeltaCommand::ReportLocationVector { loc } => {},
        DeltaCommand::ReportVelocityVector { angle } => {},
        DeltaCommand::SetLookVector { angle } => {},
        DeltaCommand::SetMovementVector { vector } => {
             
            let past_origin = physComp.get_origin();

            let move_speed = 10.0;
            //println!("moving {} {} {}", vector.normalize().x, vector.normalize().y, vector.normalize().z);
            let new_origin:Vector3<f32> = past_origin.clone() + (vector.normalize() * move_speed);
    
            //walk
            physComp.set_origin(    new_origin  ) ;

        },
        DeltaCommand::PerformEntityAction { action, target_id } => {},
    }

    
 }