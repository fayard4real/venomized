//! Provides a mechanism for tracking which entities are in which chunks.
//! This system is event-driven and reacts to entity movements and deaths.

use std::collections::{HashMap, HashSet};
use common::{entities::snake::Snake, world::{types::GridPos, world::{ChunkId, World}}};
use crate::{entity::{EntityId, EntityManager}, systems::movement::MovementEvent};

// --- Event Definitions ---
// NOTE: You would likely place this in your `movement_system.rs` file.
// It is included here to make the `tick` function signature understandable.

/// An event describing a change in an entity's presence within a chunk.
#[derive(Debug, Clone)]
pub enum PresenceEvent {
    /// An entity has appeared in a new chunk.
    EntityEnteredChunk {
        entity_id: EntityId,
        chunk_id: ChunkId,
    },
    /// An entity is no longer present in a chunk.
    EntityLeftChunk {
        entity_id: EntityId,
        chunk_id: ChunkId,
    },
}

// --- System Implementation ---

pub type PresenceMap = HashMap<ChunkId, Vec<EntityId>>;

pub struct PresenceSystem {
    pub presence_map: PresenceMap,
}

impl PresenceSystem {
    /// Creates a new, empty PresenceSystem.
    pub fn new() -> PresenceSystem {
        PresenceSystem {
            presence_map: HashMap::new(),
        }
    }

    /// Initializes the presence map with empty vectors for all possible chunks.
    /// This should be called once when the world is created.
    pub fn add_chunks(&mut self, total_chunks: u32) {
        for chunk_id in 0..total_chunks {
            self.presence_map.insert(chunk_id, Vec::new());
        }
    }
    
    /// Populates the PresenceSystem for a newly created entity.
    /// This should be called whenever a snake is spawned.
    pub fn register_new_entity(&mut self, entity_id: EntityId, snake: &Snake, world: &World) {
        let mut occupied_chunks = HashSet::new();
        for pos in snake.body.iter() {
            occupied_chunks.insert(world.chunk_at(pos));
        }

        for chunk_id in occupied_chunks {
            self.add_entity_to_chunk(chunk_id, entity_id);
        }
    }

    /// Updates the presence map based on movement events from a single game tick.
    /// This is the main update function for entity movement.
    pub fn tick(
        &mut self,
        world: &World,
        movement_events: &[MovementEvent],
        events_bus: &mut Vec<PresenceEvent>,
    ) {
        for event in movement_events {
            match event {
                MovementEvent::EntityMoved { entity_id, new_head, removed_tail } => {
                    let new_chunk = world.chunk_at(&new_head.clone());
                    let old_chunk = world.chunk_at(&removed_tail.clone());

                    // If the head and tail are in different chunks, a transition occurred.
                    if new_chunk != old_chunk {
                        // The entity is now present in the new chunk.
                        self.add_entity_to_chunk(new_chunk, *entity_id);
                        events_bus.push(PresenceEvent::EntityEnteredChunk {
                            entity_id: *entity_id,
                            chunk_id: new_chunk,
                        });

                        // Check if the entity has completely left the old chunk.
                        // This requires checking if any other body part is still in old_chunk.
                        // NOTE: This is a complex check. A simpler, more robust approach
                        // is to do a full resync on death/spawn and only handle head/tail here.
                        // For now, we will assume a simple enter/leave event model.
                        
                        // We remove the entity from the old chunk's list.
                        self.remove_entity_from_chunk(old_chunk, entity_id);
                        events_bus.push(PresenceEvent::EntityLeftChunk {
                            entity_id: *entity_id,
                            chunk_id: old_chunk,
                        });
                    }
                }
            }
        }
    }
    
    /// Handles the removal of a dead entity from the presence system.
    /// This should be called after processing PhysicsEvents.
    pub fn handle_entity_death(&mut self, entity_id: EntityId, snake: &Snake, world: &World) {
        let mut occupied_chunks = HashSet::new();
        for pos in snake.body.iter() {
            occupied_chunks.insert(world.chunk_at(&pos.clone()));
        }

        for chunk_id in occupied_chunks {
            self.remove_entity_from_chunk(chunk_id, &entity_id);
        }
    }

    /// Adds an entity ID to a specific chunk's list if it's not already there.
    pub fn add_entity_to_chunk(&mut self, chunk_id: ChunkId, entity_id: EntityId) {
        if let Some(entities_in_chunk) = self.presence_map.get_mut(&chunk_id) {
            if !entities_in_chunk.contains(&entity_id) {
                entities_in_chunk.push(entity_id);
            }
        } else {
            // This case can be treated as an error if add_chunks() was called correctly.
            eprintln!("Attempted to add entity to non-existent chunk_id: {}", chunk_id);
        }
    }

    /// Removes an entity ID from a specific chunk's list.
    pub fn remove_entity_from_chunk(&mut self, chunk_id: ChunkId, entity_id: &EntityId) {
        if let Some(entities_in_chunk) = self.presence_map.get_mut(&chunk_id) {
            // Find the position of the entity and remove it efficiently.
            if let Some(index) = entities_in_chunk.iter().position(|id| id == entity_id) {
                entities_in_chunk.swap_remove(index);
            }
        } else {
            // This case can be treated as an error if add_chunks() was called correctly.
            eprintln!("Attempted to remove entity from non-existent chunk_id: {}", chunk_id);
        }
    }
}