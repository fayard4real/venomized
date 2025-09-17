use common::world::world::World as CommonWorld;

use crate::entity::EntityManager;

pub struct World {
    pub world: CommonWorld,
    pub entity_manager: EntityManager,
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World {
            world: CommonWorld::new(width, height).unwrap(), // TODO, but, i think panic here not problem
            entity_manager: EntityManager::new(), // Because, with incorrect world size world cannot exist :(
        }
    }
}
