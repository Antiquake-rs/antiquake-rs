use bevy_ecs::prelude::Component;
use cgmath::{Vector3, Deg, num_traits::AsPrimitive};

use crate::common::{net::UnitState, gamestate::system::physics::PhysMovementType};

// use lerp::Lerp;

#[derive(Component)]
pub struct PhysicsComponent {
    
    
    pub origin: Vector3<f32>,

    pub velocity: Vector3<f32>,
   
    pub angles: Vector3<Deg<f32>>,

    pub mass: f32,  //affects certain forces but not gravity 

    pub movement_type: PhysMovementType,


    prev_origin: Vector3<f32>,
    prev_angles: Vector3<Deg<f32>>,


}

/* 
trait Lerp {
      fn lerp<T>(&self, other:T, lerp_factor:f32) -> T  where T: AsPrimitive<f32>,;
}

impl Lerp for Vector3<f32> {

    fn lerp<T>(&self, other:T, lerp_factor:f32) -> T
     where T: AsPrimitive<f32>,
    {

         return self.clone()  + lerp_factor * (self.clone() - other);
        }

}*/

impl PhysicsComponent {

    pub fn new() -> PhysicsComponent {
        PhysicsComponent { 
            origin: Vector3::new(0.0,0.0,0.0), 
            velocity: Vector3::new(0.0,0.0,0.0), 
            angles: Vector3::new(Deg(0.0),Deg(0.0),Deg(0.0)),
            mass: 100.0,
            movement_type: PhysMovementType::Walk,

            prev_origin: Vector3::new(0.0,0.0,0.0), 
            prev_angles: Vector3::new(Deg(0.0),Deg(0.0),Deg(0.0)),
        }
    }

    pub fn from_baseline(baseline: &UnitState) -> PhysicsComponent {

        PhysicsComponent { 
            origin: baseline.origin.clone(), 
            velocity: Vector3::new(0.0,0.0,0.0), 
            angles: baseline.angles.clone(),
            mass: 100.0,
            movement_type: PhysMovementType::Walk,

            prev_origin: baseline.origin.clone(),
            prev_angles: baseline.angles.clone(),
        }

    }

    pub fn prep( &mut self  ) {
        //used for lerp 
        self.prev_origin = self.origin.clone();
        self.prev_angles = self.angles.clone();
    }


    pub fn get_origin_lerped(&self, lerp_factor: f32) ->  Vector3<f32>{

        let difference = self.origin - self.prev_origin ;
        return self.prev_origin + (difference*lerp_factor);
    }
    
  /*   pub fn get_angle_lerped(&self, lerp_factor: f32) ->  Vector3<Deg<f32>>{

        let difference = self.angles - self.prev_angles ;
        return self.prev_angles + (difference*lerp_factor);
    }*/

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