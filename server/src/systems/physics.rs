//! Provides processing of physical interactions in the game.
//! Total physical interactions:
//!
//! 1. Collision of one entity with another
//! 2. Collision of an entity with itself (logically, this may be the same as the first point)
//! 3. Collision with `Tile::Wall`

use crate::systems::presence::PresenceSystem;

pub struct PhysicsSystem;

impl PhysicsSystem {
    // TODO: physics tick
    pub fn tick(presence_system: &mut PresenceSystem) {
        // IDEA: make O(N**2) bad
        // i need to propoganate presence system and make chunk
        // oriented checks, maybe i need also make a multithreading
    }
}
