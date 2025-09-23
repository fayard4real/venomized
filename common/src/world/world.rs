use std::collections::HashMap;

use crate::world::{
    chunk::Chunk,
    types::{GridPos, HEIGHT, WIDTH},
};

#[derive(Debug)]
pub enum WorldError {
    NotMultipleOf16Error,
}

pub type ChunkId = u32; // Remove magic numbers

type ChunkMap = HashMap<ChunkId, Chunk>;

/// World provides chunk storage using a hash map,
/// which stores the key (ChunkId)
/// and the value (the chunk itself).
///
/// In this case, we have completely identical chunks,
/// but since each of them has a ChunkId,
/// we can calculate global coordinates,
/// which will be necessary in many cases
#[derive(Debug)]
pub struct World {
    /// Global width of the playing field
    pub width: u32,

    /// Global height of the playing field
    pub height: u32,

    pub chunks: ChunkMap,
}

impl World {
    /// #### Important! The dimensions of the world must be multiples of 16.
    ///
    /// This is because one chunk = 16x16, so if we make the `width` 256 and
    /// the `height` 128, everything will be fine, but if we make it 255 and
    /// 127, there will be an error.
    pub fn new(width: u32, height: u32) -> Result<World, WorldError> {
        if width % WIDTH == 0 && height % HEIGHT == 0 {
            let mut chunks: ChunkMap = HashMap::new();

            let chunks_total = (width / WIDTH) * (height / HEIGHT);

            for i in 0..chunks_total {
                let chunk = Chunk::new();
                chunks.insert(i, chunk);
            }
            Ok(World {
                width: width,
                height: height,
                chunks: chunks,
            })
        } else {
            Err(WorldError::NotMultipleOf16Error)
        }
    }

    /// Returns the `ChunkId` based on global coordinates.
    /// Note: This method now takes an immutable reference `&self`
    /// as it only needs to read the world's dimensions.
    pub fn chunk_at(&self, global_pos: &GridPos) -> ChunkId {
        let chunks_per_row = self.width / WIDTH;
        let chunk_x = global_pos.x / WIDTH;
        let chunk_y = global_pos.y / HEIGHT;

        chunk_y * chunks_per_row + chunk_x
    }

    // Provides the local location on the grid based on global coordinates
    /* pub fn get_local_pos(&mut self, global_pos: GridPos) -> (GridPos, Chunk) {
        // TODO: temp
    } */
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn world_new_success() -> Result<(), WorldError> {
        let world = World::new(256, 256)?;

        assert_eq!(world.chunks.len(), 256);

        Ok(())
    }

    #[test]
    fn world_new_fail_error() {
        let world = World::new(257, 256);

        assert_matches!(world, Err(WorldError::NotMultipleOf16Error));
    }
    #[test]
    fn test_chunk_at_logic() {
        // 1. Make the world 64x32 tiles.
        // This is grid 4x2 chunks (64/16=4, 32/16=2).
        // Total summary of chunks: 4 * 2 = 8 (ID from 0 to 7).
        //
        // It's look like:
        // [0][1][2][3]
        // [4][5][6][7]
        let world = World::new(64, 32).unwrap();

        // 2. Checking boundary and internal points for different chunks.

        // --- first chunk (ID 0) ---
        // Upper left corner of the world
        assert_eq!(world.chunk_at(&GridPos { x: 0, y: 0 }), 0);
        // Bottom right corner of the FIRST chunk
        assert_eq!(world.chunk_at(&GridPos { x: 15, y: 15 }), 0);
        // Random point inside
        assert_eq!(world.chunk_at(&GridPos { x: 5, y: 10 }), 0);

        // --- second chunk (ID 1) ---
        // Upper left corner of the second chunk
        assert_eq!(world.chunk_at(&GridPos { x: 16, y: 0 }), 1);
        // random point inside
        assert_eq!(world.chunk_at(&GridPos { x: 20, y: 5 }), 1);

        // ...
        assert_eq!(world.chunk_at(&GridPos { x: 0, y: 16 }), 4);
        assert_eq!(world.chunk_at(&GridPos { x: 10, y: 20 }), 4);

        assert_eq!(world.chunk_at(&GridPos { x: 30, y: 25 }), 5);

        assert_eq!(world.chunk_at(&GridPos { x: 63, y: 31 }), 7);
        assert_eq!(world.chunk_at(&GridPos { x: 48, y: 16 }), 7);
    }
}
