//! This module implements a singular rendered chunk unit.

use bevy::prelude::*;

use crate::map::ChunkPos;
use crate::map::model::ChunkModels;

/// The size of a chunk in blocks along each axis.
pub const CHUNK_SIZE: usize = 1 << CHUNK_SIZE_BITS as usize;

/// The bit-shift used to convert world coordinates to chunk coordinates.
pub const CHUNK_SIZE_BITS: i32 = 4;

/// The mask used to convert world coordinates to block coordinates
pub const CHUNK_SIZE_MASK: i32 = (1 << CHUNK_SIZE_BITS) - 1;

/// The total number of blocks in a single chunk.
pub const TOTAL_BLOCKS: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

/// Represents a finite cubic grid of blocks within a voxel world.
#[derive(Debug, Component)]
#[require(Transform, Visibility)]
pub struct VoxelChunk {
    /// The position of this chunk in the world.
    pos: ChunkPos,

    /// The models for each block in this chunk.
    models: ChunkModels,

    /// Whether or not this chunk is marked as dirty and needs to be redrawn.
    dirty: bool,

    /// Entity for the opaque model entity of this chunk.
    pub opaque_entity: Option<Entity>,
}

impl VoxelChunk {
    /// Creates a new [`VoxelChunk`] at the specified position.
    pub fn new(pos: ChunkPos) -> Self {
        Self {
            pos,
            models: ChunkModels::default(),
            dirty: false,
            opaque_entity: None,
        }
    }

    /// Gets the position of this chunk in the world.
    pub fn pos(&self) -> ChunkPos {
        self.pos
    }

    /// Gets a slice of all block models in this chunk.
    pub fn get_models(&self) -> &ChunkModels {
        &self.models
    }

    /// Gets a mutable slice of all block models in this chunk.
    ///
    /// Calling this method will automatically mark the chunk as dirty.
    pub fn get_models_mut(&mut self) -> &mut ChunkModels {
        self.dirty = true;
        &mut self.models
    }

    /// Returns whether or not this chunk is marked as dirty and needs to be
    /// redrawn.
    ///
    /// A clean chunk does not guarantee that it has been redrawn, only that any
    /// redraw requests have already been processed.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Marks this chunk as clean and not needing to be redrawn.
    ///
    /// This method is usually called after a redraw has been scheduled. Note
    /// that this does not guarantee that the chunk has been redrawn, only that
    /// it has been scheduled for redraw.
    pub(super) fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

/// A component that stores diagnostic information about a chunk's model.
#[derive(Debug, Component)]
pub struct ChunkModelPart {
    /// The number of triangles in this model part.
    pub triangles: u32,
}
