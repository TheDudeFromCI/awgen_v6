//! This module defines the `Occlusion` and `Occluder` bitflags, which represent
//! the occlusion state of block faces and adjacent blocks in a voxel terrain.

use bitflags::bitflags;

use crate::map::CHUNK_SIZE;
use crate::map::model::ChunkModels;
use crate::map::pos::{Dir, LocalPos};

bitflags! {
    /// Represents what faces of a block are occluded by adjacent blocks.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Occlusion: u8 {
        /// The upward face is occluded by another block.
        const PosY = 0b00000001;

        /// The downward face is occluded by another block.
        const NegY = 0b00000010;

        /// The northern face is occluded by another block.
        const PosZ = 0b00000100;

        /// The southern face is occluded by another block.
        const NegZ = 0b00001000;

        /// The eastern face is occluded by another block.
        const PosX = 0b00010000;

        /// The western face is occluded by another block.
        const NegX = 0b00100000;
    }

    /// Represents what adjacent blocks are occluded by this block.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Occluder: u8 {
        /// The upward block is occluded by this block.
        const PosY = 0b00000001;

        /// The downward block is occluded by this block.
        const NegY = 0b00000010;

        /// The northern block is occluded by this block.
        const PosZ = 0b00000100;

        /// The southern block is occluded by this block.
        const NegZ = 0b00001000;

        /// The eastern block is occluded by this block.
        const PosX  = 0b00010000;

        /// The western block is occluded by this block.
        const NegX  = 0b00100000;
    }
}

impl Occlusion {
    /// Calculates the occlusion data for a block as the given position based on
    /// the surrounding block models in the chunk.
    pub fn from_chunk_models(models: &ChunkModels, pos: LocalPos) -> Self {
        const CHUNK_MAX: i32 = (CHUNK_SIZE - 1) as i32;
        let mut block_occ = Occlusion::empty();

        if pos.y < CHUNK_MAX
            && models
                .get(pos + Dir::POS_Y)
                .get_occluder_flags()
                .contains(Occluder::NegY)
        {
            block_occ |= Occlusion::PosY;
        }

        if pos.y > 0
            && models
                .get(pos + Dir::NEG_Y)
                .get_occluder_flags()
                .contains(Occluder::PosY)
        {
            block_occ |= Occlusion::NegY;
        }

        if pos.z < CHUNK_MAX
            && models
                .get(pos + Dir::POS_Z)
                .get_occluder_flags()
                .contains(Occluder::NegZ)
        {
            block_occ |= Occlusion::PosZ;
        }

        if pos.z > 0
            && models
                .get(pos + Dir::NEG_Z)
                .get_occluder_flags()
                .contains(Occluder::PosZ)
        {
            block_occ |= Occlusion::NegZ;
        }

        if pos.x < CHUNK_MAX
            && models
                .get(pos + Dir::POS_X)
                .get_occluder_flags()
                .contains(Occluder::NegX)
        {
            block_occ |= Occlusion::PosX;
        }

        if pos.x > 0
            && models
                .get(pos + Dir::NEG_X)
                .get_occluder_flags()
                .contains(Occluder::PosX)
        {
            block_occ |= Occlusion::NegX;
        }

        block_occ
    }
}
