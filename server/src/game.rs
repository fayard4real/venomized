use crate::{systems::{movement::{MovementEvent, MovementSystem}, physics::{PhysicsEvent, PhysicsSystem}, presence::{PresenceEvent, PresenceSystem}}, world::World};

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
        
        // just init presence system
        let mut presence_system = PresenceSystem::new();

        // events
        let mut movement_events: Vec<MovementEvent> = Vec::new();
        let mut presence_events: Vec<PresenceEvent> = Vec::new();
        let mut physics_events: Vec<PhysicsEvent> = Vec::new();
        
        loop {
            MovementSystem::tick(&mut self.world.entity_manager, &mut movement_events);
            presence_system.tick(&mut self.world.world, &movement_events[..], &mut presence_events);
            PhysicsSystem::tick(&mut presence_system, &mut self.world.entity_manager, &mut self.world.world, &mut physics_events);
        }

    }
}
