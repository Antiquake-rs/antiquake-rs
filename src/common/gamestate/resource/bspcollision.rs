  
use bevy_ecs::system::Resource;
use cgmath::{Vector3, Deg};

use crate::common::{
    bsp::{BspCollisionHull,BspCollisionNode},  
    math::Hyperplane
};
 




const MAX_HULLS: usize = 3;

pub enum CollisionHullLayer {
    POINT_LAYER=0,
    CHARACTER_LAYER=1,
    HUGE_LAYER=2 
}


pub struct CachedCollisionHull {
    
    planes: Box<[Hyperplane]>,
    nodes: Box<[BspCollisionNode]>,
    node_id: usize,
    node_count: usize,
    mins: Vector3<f32>,
    maxs: Vector3<f32>,

}


//#[derive(Resource)]
pub struct BspCollisionResource {
    
    cached_hulls: [BspCollisionHull;MAX_HULLS] //array with size 3 

}

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




 

}