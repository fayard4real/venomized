//! Provides a mechanism for updating the player's field of view.

use std::collections::HashMap;

use common::world::world::ChunkId;

use crate::entity::{EntityId, EntityManager};

type PresenceMap = HashMap<ChunkId, Vec<EntityId>>;

pub struct PresenceSystem {
    pub presence_map: PresenceMap,
}

impl PresenceSystem {
    pub fn new() -> PresenceSystem {
        PresenceSystem {
            presence_map: HashMap::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk_id: ChunkId) {
        self.presence_map.insert(chunk_id, Vec::new());
    }

    pub fn tick(&mut self, entity_manager: EntityManager) {
        // 1. need to have chunk relative position (local) of entity, not global
        // 2. need to add/remove from some chunks
    }

    pub fn add_entity(&mut self) {}

    pub fn remove_entity(&mut self) {}
}
