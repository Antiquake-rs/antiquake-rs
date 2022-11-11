use std::collections::HashMap;

use bevy_ecs::prelude::Entity;


 
pub struct BevyEntityLookupRegistry {

    entities: HashMap<u16, Entity>

}

impl BevyEntityLookupRegistry {

    pub fn new() -> BevyEntityLookupRegistry {
        BevyEntityLookupRegistry {
            entities: HashMap::new() 
        }
    }

    pub fn get(&self, unit_id:&u16) -> Option<&Entity> {
        return self.entities.get(unit_id);
    }
    pub fn insert(&self, unit_id: u16, entity: Entity) {
        self.entities.insert(unit_id,entity);
    }

}




