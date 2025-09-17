//! Provides a wrapper over `snake.move_forward` method that
//! causes movement in the specified direction (which is a state).

use crate::entity::EntityManager;

pub struct MovementSystem;

impl MovementSystem {
    pub fn tick(entities: &mut EntityManager) {
        for entity in entities.iter_mut() {
            entity.move_forward();
        }
    }
}
