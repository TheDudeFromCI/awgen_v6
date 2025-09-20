//! This module implements a singular rendered chunk unit.

use bevy::prelude::*;

use crate::map::chunk_model::ChunkModelRoot;
use crate::map::pos::BlockPos;
use crate::map::{Block, BlockModel, BlockMut, ChunkPos};

/// The size of a chunk in blocks along each axis.
pub const CHUNK_SIZE: usize = 1 << CHUNK_SIZE_BITS as usize;

/// The bit-shift used to convert world coordinates to chunk coordinates.
pub const CHUNK_SIZE_BITS: i32 = 4;

/// The mask used to convert world coordinates to block coordinates
pub const CHUNK_SIZE_MASK: i32 = (1 << CHUNK_SIZE_BITS) - 1;

/// Represents a finite cubic grid of blocks within a voxel world.
#[derive(Debug, Component)]
#[require(ChunkModelRoot)]
pub struct VoxelChunk {
    /// The position of this chunk in the world.
    pos: ChunkPos,

    /// The precomputed models for each block in this chunk.
    ///
    /// The length of this vector is always `CHUNK_SIZE^3`.
    models: Vec<BlockModel>,

    /// Whether or not this chunk is marked as dirty and needs to be redrawn.
    dirty: bool,
}

impl VoxelChunk {
    /// Creates a new [`VoxelChunk`] at the specified position.
    pub fn new(pos: ChunkPos) -> Self {
        let block_count = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
        Self {
            pos,
            models: (0 .. block_count).map(|_| BlockModel::default()).collect(),
            dirty: false,
        }
    }

    /// Gets the position of this chunk in the world.
    pub fn pos(&self) -> ChunkPos {
        self.pos
    }

    /// Gets the block at the specified relative position within the chunk.
    pub fn get_block(&self, pos: BlockPos) -> Block<'_> {
        Block { pos, chunk: self }
    }

    /// Gets a mutable reference to the block at the specified relative position
    /// within the chunk.
    pub fn get_block_mut(&mut self, pos: BlockPos) -> BlockMut<'_> {
        BlockMut { pos, chunk: self }
    }

    /// Gets the block model at the specified index within the chunk.
    pub(super) fn get_model(&self, index: usize) -> &BlockModel {
        &self.models[index]
    }

    /// Gets a mutable reference to the block model at the specified index
    pub(super) fn get_model_mut(&mut self, index: usize) -> &mut BlockModel {
        &mut self.models[index]
    }

    /// Clones the [`BlockModel`] data for this chunk into a
    /// [`VoxelChunkModel`].
    pub fn get_chunk_model(&self) -> VoxelChunkModel {
        VoxelChunkModel {
            models: self.models.clone(),
        }
    }

    /// Returns whether or not this chunk is marked as dirty and needs to be
    /// redrawn.
    ///
    /// A clean chunk does not guarantee that it has been redrawn, only that any
    /// redraw requests have already been processed.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Marks this chunk as dirty and needing to be redrawn.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
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

/// A container for the block models in a chunk, used for rendering. This data
/// is detached from the original [`VoxelChunk`] to allow for concurrent
/// processing.
#[derive(Debug, Clone)]
pub struct VoxelChunkModel {
    /// The precomputed models for each block in this chunk.
    models: Vec<BlockModel>,
}

impl VoxelChunkModel {
    /// Gets the block model at the specified relative position within the
    /// chunk.
    pub fn get_model(&self, pos: BlockPos) -> &BlockModel {
        self.models.get(pos.as_index()).unwrap()
    }
}
