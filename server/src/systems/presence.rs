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
    // test it
    pub fn add_chunks(&mut self, total_chunks: u32) {
        for chunk_id in 0..total_chunks {
            self.presence_map.insert(chunk_id, Vec::new());
        }
    }
    /// O(Σ S_i) algorithm
    pub fn tick(&mut self, entity_manager: &mut EntityManager) {

    }

    pub fn add_entity(&mut self) {}

    pub fn remove_entity(&mut self) {}
}
