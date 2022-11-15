use bevy_ecs::system::{Query, Res, ResMut};
use cgmath::{Vector3, Deg, Angle, InnerSpace};

use crate::common::gamestate::{
    component::physics::{PhysicsComponent}, GameStateDeltaBuffer, GameStateDelta, DeltaCommand,
    entity::{BevyEntityLookupRegistry}, resource::bspcollision::{BspCollisionResource, CollisionHullLayer} 
};
 

pub enum EntityPostureType {
    Stand,
    Crouch,
    Prone 
}

pub enum PhysMovementType {
    Walk = 0,
    Hover = 1,
    Fly = 2,
    NoClip = 3,
    Swim = 4 
}

//do this a better way ? 
impl From<usize> for PhysMovementType {
    fn from(move_type: usize) -> Self {
        match move_type {
            0 => PhysMovementType::Walk,
            1 => PhysMovementType::Hover,
            2 => PhysMovementType::Fly,
            3 => PhysMovementType::NoClip,
            4 => PhysMovementType::Swim,

            _ => PhysMovementType::NoClip
        }
    }
}
 

//consider moving all this to the physics component system !! 

pub fn movement_constrained_flat(physBodyType: PhysMovementType) -> bool{

    match physBodyType {
        PhysMovementType::Walk => return true,
        PhysMovementType::Hover => return true,
        PhysMovementType::Fly => return false,
        PhysMovementType::NoClip => return false,
        PhysMovementType::Swim => return false,
    }
}
pub fn body_has_collision(physBodyType: PhysMovementType) -> bool{

    match physBodyType {
        PhysMovementType::Walk => return true,
        PhysMovementType::Hover => return true,
        PhysMovementType::Fly => return true,
        PhysMovementType::NoClip => return false,
        PhysMovementType::Swim => return true,
    }
}

pub fn euler_angles_to_cartesian(pitch:Deg<f32>,yaw:Deg<f32>,roll:Deg<f32> ) -> Vector3<f32> {

    return Vector3::new( yaw.cos() * pitch.cos(), yaw.sin()*pitch.cos(), pitch.sin() );
    
}

pub fn calc_movement_vector( input_cmds: Vector3<i16>, facing: Vector3<Deg<f32>>, physMovementType: PhysMovementType) -> Option<Vector3<f32>>{
 
        
    //pitch roll yaw 
    let forward_dir = euler_angles_to_cartesian(facing.x,facing.y,facing.z) ;

    let forward_dir_normalized = match movement_constrained_flat(physMovementType){
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



    //https://bevy-cheatbook.github.io/programming/world.html





pub fn apply_gamestate_delta_collisions (
    mut delta_buffer: ResMut<GameStateDeltaBuffer>,
    bsp_collision_option: Option<Res<BspCollisionResource>>
    //mut query: Query<(&mut StaticCollisionHull)> 
) {
    println!("apply gs deltas collisions 1");
    match bsp_collision_option {

        Some(bsp_collision) => {
 


                for state_delta in delta_buffer.iter_mut() {

                    println!("apply gs deltas collisions 2");

                    match state_delta.command {
                        
                        DeltaCommand::TranslationMovement { 
                            origin_loc, vector, speed, phys_move_type
                        } =>  {

                            if body_has_collision(phys_move_type.into()) {


                                  //vector is always normalized to 1 
                                //speed is typically 1 
                                let proposed_end_loc = origin_loc.clone() + (vector.normalize() * speed);

                                let collision_trace = bsp_collision.trace_collision(
                                    origin_loc, proposed_end_loc, 
                                    CollisionHullLayer::CHARACTER_LAYER );



                                    /*
                                    
                                            Maybe  trace w the X leg and trace w the Y leg separately and if either one hits, cancel out that portion of the vector 
                                    */


                                println!( " trace is {:?}" , collision_trace );


                            }
                          

                        },
                        
                        _ => {}
                    }

                    //delta.


                // delta.modify_using_collision_trace( collision_trace );


                }



        }

        None => {} 
    }
    

    


}

pub fn apply_collision_to_gamestate_delta (
    mut delta_buffer: ResMut<GameStateDeltaBuffer>,
    bsp_collision: Res<BspCollisionResource>
    //mut query: Query<(&mut StaticCollisionHull)> 
) {
    

    


} 




//this is called now 
pub fn update_physics_movement(
    // unit id registry 
    entity_lookup: Res<BevyEntityLookupRegistry>,
    mut delta_buffer: ResMut<GameStateDeltaBuffer>,
    mut query: Query<(&mut PhysicsComponent)> 
){

    //for each delta buffer, apply it to the corresponding entitys phys component
    while  !delta_buffer.is_empty(){
        
        let next_delta = delta_buffer.pop();
        
        match next_delta {
            Some(delta) => {

                let unit_id = delta.source_unit_id;  

                let bevy_entity_id = entity_lookup.get( unit_id );

                match bevy_entity_id {
                    Some(ent_id) => {

                        match query.get_mut(*ent_id) {

                            Ok(mut phys_comp) => {
                                self::apply_gamestate_delta_buffer(   &delta,  phys_comp.as_mut()  );
                            }
                            _ => {}

                        }

                    }
                    _ => {}
                }
                  
               
                 

            }
            _ => {}

        }
     

    }

    delta_buffer.reset_flags();

 

}

fn apply_gamestate_delta_buffer( 
     delta:  &GameStateDelta ,
     physComp: &mut PhysicsComponent
 ){


    match &delta.command {
        DeltaCommand::ReportLocation { loc } => {},
        DeltaCommand::ReportVelocity { angle } => {},
        DeltaCommand::ReportLookVector { angle } => {},
        DeltaCommand::TranslationMovement { vector, origin_loc, speed, phys_move_type } => {
            

            //if the suggest origin_loc is way off the past_origin , maybe we do something  -- ? 
            // it means whoever created the delta is way out of sync with our state 

            let past_origin = physComp.origin.clone();

           // let move_speed = 10.0;
            //println!("moving {} {} {}", vector.normalize().x, vector.normalize().y, vector.normalize().z);
            let new_origin:Vector3<f32> = past_origin.clone() + (vector.normalize() * speed.to_owned());
    
            //walk
            physComp.set_origin(    new_origin  ) ;

        },
        DeltaCommand::PerformEntityAction { action, target_id } => {},
    }

    
 }