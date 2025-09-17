use crate::world::World;

pub struct Game {
    world: World, // contains chunks
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            world: World::new(width, height),
        }
    }

    pub fn tick() {
        // TODO: movement, physics, some another things like apple spawn
    }
}
