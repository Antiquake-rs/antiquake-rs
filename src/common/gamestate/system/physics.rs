use bevy_ecs::system::{Query, Res, ResMut};
use cgmath::{Vector3, Deg, Angle, InnerSpace};

use crate::common::{gamestate::{
    component::physics::{PhysicsComponent}, GameStateDeltaBuffer, GameStateDelta, DeltaCommand,
    entity::{BevyEntityLookupRegistry}, resource::bspcollision::{BspCollisionResource, CollisionHullLayer} 
}, bsp::BspLeafPhysMaterial};
 
#[derive(Clone)]
pub enum EntityPostureType {
    Stand,
    Crouch,
    Prone 
}

#[derive(Clone, Copy)]
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


/* 
    pub fn apply_collision_to_gamestate_delta (
        mut delta_buffer: ResMut<GameStateDeltaBuffer>,
        bsp_collision: Res<BspCollisionResource>
        //mut query: Query<(&mut StaticCollisionHull)> 
    ) {
        
     
    } */
 


/*

    If the component is not on the ground, add gravity acceleration 


*/
pub fn apply_gravity_system ( 
    bsp_collision: Res<BspCollisionResource>,
    mut query: Query<&mut PhysicsComponent> 
){  

    let gravity_accel = Vector3::new(0.0,0.0,-0.9);
    let water_accel = Vector3::new(0.0,0.0,-0.2);
 

    let unit_height = 40.0; 

    for mut phys_comp in query.iter_mut(){
       

       // let CHECK_DIST = 80.0;

        let start_loc = phys_comp.origin  + (gravity_accel.normalize()   * -5.0 ) ;  
        let proposed_end_loc = phys_comp.origin  + (gravity_accel.normalize() * unit_height);

        let on_ground_trace = bsp_collision.trace_collision(
            start_loc, proposed_end_loc, 
            CollisionHullLayer::CHARACTER_LAYER );   

        

            println!( "grav trace is {:?}" , on_ground_trace );
        match on_ground_trace.contents_type() {

            BspLeafPhysMaterial::Empty => {
                phys_comp.apply_acceleration_to_velocity(  gravity_accel  );
            }

            BspLeafPhysMaterial::Water => {
                phys_comp.apply_acceleration_to_velocity(  water_accel  );
            }


            BspLeafPhysMaterial::Solid => {
                

                        
                 
                    //pop out of world if under it 
                    let trace_end_point = on_ground_trace.end_point();
                    let solid_start = on_ground_trace.start_solid(); 
                    let ground_z = trace_end_point.z;

                    //if units feet are under ground 
                    if  phys_comp.origin.z  < ground_z +unit_height && !solid_start{            
                        phys_comp.origin.z = ground_z + unit_height;
                    }


                    if solid_start {
                        //pop out of solid 
                        phys_comp.apply_acceleration_to_velocity(  gravity_accel * -1.0  );

                        let trace_start_point = on_ground_trace.start_point();

                        //as far as we know, ground is above the start point so start moving towards there 
                        if  phys_comp.origin.z  < trace_start_point.z    {            
                            phys_comp.origin.z = trace_start_point.z  ;
                        }
                    }




            }

            _ => {} 

        }

            
        

    }

    

}


pub fn apply_gamestate_delta_collisions (
    mut delta_buffer: ResMut<GameStateDeltaBuffer>,
    bsp_collision: Res<BspCollisionResource>
     
) {

    let mut modified_deltas:Vec<GameStateDelta> = Vec::new();

    let unmodified_deltas:Vec<GameStateDelta> = delta_buffer.deltas.drain(..).collect(); 

    
    let unit_height = 40.0; 
    let vertical_vector = Vector3::new(0.0,0.0,1.0);

    for state_delta in unmodified_deltas  {

        
        match &state_delta.command {
            
            DeltaCommand::TranslationMovement (translation) =>  {

                if body_has_collision( translation.phys_move_type.into()) {


                    let CHECK_DIST= 90.0;

                        //vector is always normalized to 1 
                    //speed is typically 1 
                    let start_loc = translation.origin_loc  + (translation.vector.normalize() * 15.0) + vertical_vector*unit_height; //helps get unstuck 
                    let proposed_end_loc = translation.origin_loc + (translation.vector.normalize() * CHECK_DIST)  + vertical_vector*unit_height ;  

                    let forwards_trace = bsp_collision.trace_collision(
                        start_loc, proposed_end_loc, 
                        CollisionHullLayer::CHARACTER_LAYER );

                    /*  let backwards_trace = bsp_collision.trace_collision(
                        proposed_end_loc,   start_loc,
                        CollisionHullLayer::CHARACTER_LAYER );*/
                            

                        println!( " trace is {:?}" , forwards_trace );

                        

                        if  forwards_trace.contents_type() == BspLeafPhysMaterial::Solid                                        
                            {
                            modified_deltas.push(  state_delta.modify_via_collision_trace(forwards_trace) ) ; 
                        }else{
                            modified_deltas.push( state_delta );
                        } 


                }
                

            },
            
            _ => {
                modified_deltas.push( state_delta ); 
            }
        }

        

    }

  

    delta_buffer.deltas = Box::new(modified_deltas.drain(..).collect());
    


}




//applies gamestate delta buffers 
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
        DeltaCommand::ApplyForce (force) => {

            let acceleration = force.get_scaled_force() ;

            physComp.apply_acceleration_to_velocity(  acceleration  ) ;

        },
        DeltaCommand::TranslationMovement  (translation) => {
            

            //if the suggest origin_loc is way off the past_origin , maybe we do something  -- ? 
            // it means whoever created the delta is way out of sync with our state 

            let past_origin = physComp.origin.clone();

           // let move_speed = 10.0;
            //println!("moving {} {} {}", vector.normalize().x, vector.normalize().y, vector.normalize().z);
            let new_origin:Vector3<f32> = past_origin  + (translation.vector.normalize() * translation.speed.to_owned());
    
            //walk
            physComp.set_origin(  new_origin  ) ;

        },
        DeltaCommand::PerformEntityAction { action, target_id } => {},
    }

    
 }


 /*
 
        If V trace hits ground, kill vertical velocity. 

        If H trace hits wall, kill horizontal velocity 
 */
 pub fn apply_phys_velocities_system( 
    bsp_collision: Res<BspCollisionResource>,
    mut query: Query<&mut PhysicsComponent> 
){


    let unit_height = 40.0; 

    
    for mut phys_comp in query.iter_mut(){
        

        //ignore insignificant velocities to speed up this system 
        if phys_comp.velocity.magnitude() < 0.0000001 { continue; }

 

        let start_loc = phys_comp.origin  + (phys_comp.velocity.normalize() * 1.0); //helps get unstuck 
        let proposed_end_loc = phys_comp.origin  + (phys_comp.velocity.normalize() * unit_height);

        let forwards_trace = bsp_collision.trace_collision(
            start_loc, proposed_end_loc, 
            CollisionHullLayer::CHARACTER_LAYER );

            
        
        if forwards_trace.contents_type() == BspLeafPhysMaterial::Solid {

            //zero out velocity if we hit something 
            phys_comp.set_velocity(Vector3::new(0.0,0.0,0.0));

        } 

 
        phys_comp.origin = phys_comp.origin + phys_comp.velocity; 
                    
        

    }




}