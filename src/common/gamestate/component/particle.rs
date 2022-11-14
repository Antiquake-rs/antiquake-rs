use bevy_ecs::prelude::Component;
use cgmath::{Vector3, Deg};

use crate::common::net::UnitState;



#[derive(Component)]
pub struct ParticleComponent {
    
    
    pub color: usize, 
}

impl ParticleComponent {

    pub fn new() -> ParticleComponent {
        ParticleComponent { 
            color: 0, 
         
        }
    }

    /*pub fn from_baseline(baseline: &UnitState) -> ParticleComponent {

        ParticleComponent { 
            model_id: baseline.model_id,
            skin_id: baseline.skin_id,
            frame_id: 0 
        }

    }*/

   

}