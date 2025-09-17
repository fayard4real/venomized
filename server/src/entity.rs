use std::collections::HashMap;

use common::entities::snake::Snake;

/// For identifying clients on the server and among themselves
type Id = u64;

type EntityMap = HashMap<Id, Snake>;

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
        // TODO: add UUID generation or something like that
        let k = 5; // TODO: test variant
        self.entities.insert(k, Snake::new());
    }

    pub fn remove(&mut self, id: Id) {
        self.entities.remove(&id);
    }

    pub fn get(&self, id: Id) -> Option<&Snake> {
        self.entities.get(&id)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut Snake> {
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
