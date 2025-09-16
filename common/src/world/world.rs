use std::collections::HashMap;

use crate::world::chunk::Chunk;

type ChunkId = i32; // Remove magic numbers

pub struct World {
    pub chunks: HashMap<ChunkId, Chunk>, // TODO: add Rwlock from Tokio??? maybe... 
}