use std::collections::HashMap;

use bevy_ecs::prelude::Entity;


 
pub struct BevyEntityLookupRegistry {

    entities: HashMap<u32, Entity>

}

impl BevyEntityLookupRegistry {

    pub fn new() -> BevyEntityLookupRegistry {
        BevyEntityLookupRegistry {
            entities: HashMap::new() 
        }
    }

    pub fn get(&self, unit_id:&u32) {
        return self.entities.get(unit_id);
    }

}




