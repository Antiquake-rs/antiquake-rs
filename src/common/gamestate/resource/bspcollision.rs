  
use bevy_ecs::system::Resource;
use cgmath::{Vector3, Deg};

use crate::{common::{
    bsp::{BspCollisionHull,BspCollisionNode, BspError},  
    math::Hyperplane
}, server::world::Trace};
  
const MAX_HULLS: usize = 3;

pub enum CollisionHullLayer {
    POINT_LAYER=0,
    CHARACTER_LAYER=1,
    HUGE_LAYER=2 
}
 


//#[derive(Resource)]  //do this to all resources after bevy update 0.9 
pub struct BspCollisionResource {
    
    cached_hulls: [BspCollisionHull;MAX_HULLS] //array with size 3 

}

/*

    Clone the physics hulls that the bsp map loaded in 

    They get used for physics trace checks in our bevy ECS for gamestate deltas (entity move transforms)
*/
impl BspCollisionResource {

    pub fn new(hulls: &[BspCollisionHull]) -> BspCollisionResource {
        BspCollisionResource { 
            cached_hulls: [
                hulls[0].clone(),
                hulls[1].clone(),
                hulls[2].clone()

            ]
        }
    }

    //use this to validate the gamestate deltas 
    //in fact they should not just be validated, but corrected -- take out one component for example so the player slides along the wall when trying to go into it diag . 
    pub fn trace_collision( &self, start:Vector3<f32>, end: Vector3<f32>, hull_layer: CollisionHullLayer )  -> Trace {

        let hull:&BspCollisionHull = &self.cached_hulls[hull_layer as usize];

        let trace_result = hull.trace(start, end);

        let trace_output = match trace_result {
            Ok( t) => t,
            Err(_) => {panic!(" ERROR: Unable to trace collision data ! ")} //why would this ever happen ? 
        };



        return trace_output
    } 
 

}