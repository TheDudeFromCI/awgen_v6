//! This module implements the [`ChunkTable`] resource for quickly looking up
//! chunks by their position.

use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use crate::map::ChunkPos;

/// A resource that maps chunk positions to their corresponding entities.
#[derive(Debug, Default, Resource)]
pub struct ChunkTable {
    /// The internal hash map storing the chunk positions and their entities.
    table: HashMap<ChunkPos, Entity>,
}

impl ChunkTable {
    /// Gets the chunk at the given position, if it exists.
    pub fn get_chunk(&self, pos: ChunkPos) -> Option<Entity> {
        self.table.get(&pos).copied()
    }

    /// Initializes a chunk at the given position with the given entity.
    pub fn add_chunk(&mut self, pos: ChunkPos, entity: Entity) {
        self.table.insert(pos, entity);
    }

    /// Removes the chunk at the given position.
    pub fn remove_chunk(&mut self, pos: ChunkPos) {
        self.table.remove(&pos);
    }

    /// Returns the number of chunks currently stored in the table.
    pub fn len(&self) -> usize {
        self.table.len()
    }
}
