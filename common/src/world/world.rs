use std::collections::HashMap;

use crate::world::{
    chunk::Chunk,
    types::{HEIGHT, WIDTH},
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

    pub chunks: ChunkMap, // TODO: add Rwlock from Tokio??? maybe...
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
}
