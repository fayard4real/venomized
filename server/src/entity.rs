use std::collections::HashMap;

use common::entities::snake::Snake;
use u64_id::U64Id;

/// For identifying clients on the server and among themselves
pub type EntityId = u64;

type EntityMap = HashMap<EntityId, Snake>;

pub struct EntityManager {
    pub entities: EntityMap,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entities: HashMap::new(),
        }
    }

    // -- Wrappers --
    pub fn add(&mut self) {
        self.entities.insert(U64Id::new().inner(), Snake::new());
    }

    pub fn remove(&mut self, id: EntityId) {
        self.entities.remove(&id);
    }

    pub fn get(&self, id: &EntityId) -> Option<&Snake> {
        self.entities.get(id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Snake> {
        self.entities.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Snake> {
        self.entities.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Snake> {
        self.entities.values_mut()
    }
    // -- Wrappers end --
}
