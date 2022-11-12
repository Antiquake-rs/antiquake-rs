use std::collections::HashMap;

use bevy_ecs::prelude::Entity;


 
pub struct BevyEntityLookupRegistry {

    entities: HashMap<usize, Entity>

}

impl BevyEntityLookupRegistry {

    pub fn new() -> BevyEntityLookupRegistry {
        BevyEntityLookupRegistry {
            entities: HashMap::new() 
        }
    }

    pub fn get(&self, unit_id:usize) -> Option<&Entity> {
        return self.entities.get(&unit_id);
    }

    pub fn insert(&mut self, unit_id: usize, entity: Entity) {
        self.entities.insert(unit_id,entity);
    }

}




