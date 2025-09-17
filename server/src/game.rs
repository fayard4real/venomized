use crate::{systems::movement::MovementSystem, world::World};

pub struct Game {
    world: World, // contains chunks
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            world: World::new(width, height),
        }
    }

    pub fn tick(&mut self) {
        // temp example 4real, im not sure its working
        MovementSystem::tick(&mut self.world.entity_manager);

        // TODO: movement, physics, some another things like apple spawn
    }
}
