/// Enumeration for the deterministic designation of a cell and its state.
/// An excellent solution for ensuring that the client knows
/// how to render using any of the possible chars/methods.
#[derive(Clone, Copy)]
pub enum Tile {
    /// An empty tile with nothing in it
    Empty,

    /// The boundaries of the world,
    /// upon colliding with which
    /// the snake instantly perishes.
    Wall,

    /// In a standard snake game,
    /// the snake grows as it eats apples,
    /// and it's the same here.
    Apple,
}

/// A structure to have associative меthods (without &mut self)
/// and methods that facilitate access to the API world
/// for both the future client and server implementation.
pub struct World {
    /// We use a one-dimensional vector to store world tiles
    /// and use index arithmetic to move around it for our operations.
    /// This is necessary because Vec<Vec<Tile>>
    /// is slower and will cause cache misses.
    ///
    /// Using a static array doesn't make sense either,
    /// because then we wouldn't be able to dynamically
    /// specify the size of the world when creating the server.
    /// Plus, Vec<Tile> won't be reallocated.
    /// It'll be created once and will be like a regular array, which is perfect
    pub grid: Vec<Tile>,

    /// Required in order to correctly iterate through a one-dimensional array
    pub width: u32,
}

pub struct GridPos {
    pub x: u32,
    pub y: u32,
}

impl World {
    /// Generate game field with Tile::Wall along the edges of the map
    /// TODO: apple?
    pub fn new(width: u32, height: u32) -> World {
        let grid = vec![Tile::Empty; (width * height) as usize];
        World {
            grid: grid,
            width: width,
        }
        // TODO: some another logic, walls, basic apple's
    }

    pub fn get_tile(&self, pos: GridPos) -> Option<&Tile> {
        let index = self.pos(pos)?;
        self.grid.get(index)
    }

    pub fn set_tile(&mut self, pos: GridPos, tile: Tile) -> Option<()> {
        let index = self.pos(pos)?;
        let target_tile = self.grid.get_mut(index)?;
        *target_tile = tile;
        Some(())
    }
    // -- Internal magic --
    fn pos(&self, pos: GridPos) -> Option<usize> {
        if pos.x < self.width && pos.y < self.height() {
            Some((pos.y * self.width + pos.x) as usize)
        } else {
            None
        }
    }

    fn height(&self) -> u32 {
        (self.grid.len() as u32 / self.width) as u32
    }
    // -- Internal magic end --
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn world_new_success_size() {
        let world = World::new(1000, 1000);

        assert_eq!(world.grid.len(), (1000 * 1000) as usize)
    }

    #[test]
    fn world_height_success() {
        let world = World::new(1000, 1000);
        assert_eq!(world.height(), 1000)
    }
}
