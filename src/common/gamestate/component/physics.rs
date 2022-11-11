use bevy_ecs::prelude::Component;
use cgmath::{Vector3, Deg};



#[derive(Component)]
pub struct PhysicsComponent {
    
    
    pub origin: Vector3<f32>,
   
    pub angles: Vector3<Deg<f32>>,


}

