use crate::world::types::{GridPos, HEIGHT, WIDTH};

/// Enumeration for the deterministic designation of a cell and its state.
/// An excellent solution for ensuring that the client knows
/// how to render using any of the possible chars/methods.
#[derive(Clone, Copy, Debug)]
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
#[derive(Debug)]
pub struct Chunk {
    /// We use a one-dimensional vector to store chunk tiles
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
    // Required in order to correctly iterate through a one-dimensional array
    // pub width: u32, <- deprecated field, we already know is 16
}

// TODO: refactoring cuz new system
impl Chunk {
    /// Generate game field with Tile::Wall along the edges of the map
    /// TODO: apple?
    pub fn new() -> Chunk {
        let grid = vec![Tile::Empty; (WIDTH * HEIGHT) as usize];
        Chunk {
            grid: grid,
            // width: WIDTH, deprecated
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
        if pos.x < WIDTH && pos.y < HEIGHT {
            Some((pos.y * WIDTH + pos.x) as usize)
        } else {
            None
        }
    }
    // -- Internal magic end --
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn chunk_new_success_size() {
        let chunk = Chunk::new();

        assert_eq!(chunk.grid.len(), (HEIGHT * WIDTH) as usize)
    }

    // TODO all
    #[test]
    fn chunk_get_tile_success() {}

    #[test]
    fn chunk_set_tile_success() {}

    #[test]
    fn test_pos_to_index_conversion() {
        let chunk = Chunk::new();

        assert_eq!(chunk.pos(GridPos { x: 0, y: 0 }), Some(0));
        assert_eq!(chunk.pos(GridPos { x: 9, y: 0 }), Some(9));
    }

    #[test]
    fn chunk_pos_fail() {
        let chunk = Chunk::new();

        assert_eq!(chunk.pos(GridPos { x: 17, y: 5 }), None);
    }
}
