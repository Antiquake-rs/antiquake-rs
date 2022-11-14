use bevy_ecs::prelude::Component;
use cgmath::{Vector3, Deg};

use crate::common::net::UnitState;



#[derive(Component)]
pub struct RenderModelComponent {
    
    
    pub model_id: usize,

    pub skin_id: usize,
   
    pub frame_id: usize,


}

impl RenderModelComponent {

    pub fn new() -> RenderModelComponent {
        RenderModelComponent { 
            model_id: 0, 
            skin_id: 0,
            frame_id: 0
        }
    }

    pub fn from_baseline(baseline: &UnitState) -> RenderModelComponent {

        RenderModelComponent { 
            model_id: baseline.model_id,
            skin_id: baseline.skin_id,
            frame_id: 0 
        }

    }

   

}