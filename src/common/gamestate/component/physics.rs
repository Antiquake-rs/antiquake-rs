use bevy_ecs::prelude::Component;
use cgmath::{Vector3, Deg};



#[derive(Component)]
pub struct PhysicsComponent {
    
    
    pub origin: Vector3<f32>,
   
    pub angles: Vector3<Deg<f32>>,


}

impl PhysicsComponent {

    pub fn new() -> PhysicsComponent {
        PhysicsComponent { 
            origin: Vector3::new(0.0,0.0,0.0), 
            angles: Vector3::new(Deg(0.0),Deg(0.0),Deg(0.0)) 
        }
    }

    pub fn set_origin( &mut self, vec:Vector3<f32>  ) {
        self.origin = vec.clone();
    }

    pub fn set_angles( &mut self, vec:Vector3<Deg<f32>>  ) {
        self.angles = vec.clone();
    }

}