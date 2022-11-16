use bevy_ecs::prelude::Component;
use cgmath::{Vector3, Deg};

use crate::common::{net::UnitState, gamestate::system::physics::PhysMovementType};



#[derive(Component)]
pub struct PhysicsComponent {
    
    
    pub origin: Vector3<f32>,

    pub velocity: Vector3<f32>,
   
    pub angles: Vector3<Deg<f32>>,

    pub mass: f32,  //affects certain forces but not gravity 

    pub movement_type: PhysMovementType


}

impl PhysicsComponent {

    pub fn new() -> PhysicsComponent {
        PhysicsComponent { 
            origin: Vector3::new(0.0,0.0,0.0), 
            velocity: Vector3::new(0.0,0.0,0.0), 
            angles: Vector3::new(Deg(0.0),Deg(0.0),Deg(0.0)),
            mass: 100.0,
            movement_type: PhysMovementType::Walk,
        }
    }

    pub fn from_baseline(baseline: &UnitState) -> PhysicsComponent {

        PhysicsComponent { 
            origin: baseline.origin.clone(), 
            velocity: Vector3::new(0.0,0.0,0.0), 
            angles: baseline.angles.clone(),
            mass: 100.0,
            movement_type: PhysMovementType::Walk,
        }

    }

    pub fn set_origin( &mut self, vec:Vector3<f32>  ) {
        self.origin = vec.clone();
    }

    pub fn set_velocity( &mut self, vec:Vector3<f32>  ) {
        self.velocity = vec.clone();
    }

    pub fn set_angles( &mut self, vec:Vector3<Deg<f32>>  ) {
        self.angles = vec.clone();
    }

    pub fn apply_acceleration_to_velocity( &mut self, accel: Vector3<f32> ){

        self.velocity = self.velocity + accel; 
                
    }

    pub fn unit_height(&self) -> f32 {
        return 40.0 
    }

}