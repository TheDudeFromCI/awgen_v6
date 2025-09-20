//! Contains a data wrapper for a block on the map, allowing for easy access
//! to its model and other properties.

use crate::map::{BlockModel, BlockPos, VoxelChunk};

/// Contains the definition for a block on the map, and how it should be
/// rendered. This is a virtual definition that reads from a [`VoxelChunk`].
#[derive(Debug)]
pub struct Block<'c> {
    /// The position of this block within its chunk.
    pub(super) pos: BlockPos,

    /// A reference to the chunk containing this block.
    pub(super) chunk: &'c VoxelChunk,
}

impl Block<'_> {
    /// Gets the position of this block within its chunk.
    pub fn pos(&self) -> BlockPos {
        self.pos
    }

    /// Gets the model definition for this block.
    pub fn model(&self) -> &BlockModel {
        self.chunk.get_model(self.pos.as_index())
    }
}

/// Contains the definition for a block on the map, and how it should be
/// rendered. This is a virtual definition that reads and writes from a
/// [`VoxelChunk`].
#[derive(Debug)]
pub struct BlockMut<'c> {
    /// The position of this block within its chunk.
    pub(super) pos: BlockPos,

    /// A mutable reference to the chunk containing this block.
    pub(super) chunk: &'c mut VoxelChunk,
}

impl BlockMut<'_> {
    /// Gets the position of this block within its chunk.
    pub fn pos(&self) -> BlockPos {
        self.pos
    }

    /// Gets the model definition for this block.
    pub fn model(&self) -> &BlockModel {
        self.chunk.get_model(self.pos.as_index())
    }

    /// Gets a mutable reference to the model definition for this block. Calling
    /// this method will mark the chunk as dirty, requiring a redraw.
    pub fn model_mut(&mut self) -> &mut BlockModel {
        self.chunk.mark_dirty();
        self.chunk.get_model_mut(self.pos.as_index())
    }
}
