//! This module implements block model types for the terrain mesh generation.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map::occlusion::Occluder;
use crate::map::pos::LocalPos;
use crate::map::{Occlusion, TOTAL_BLOCKS};
use crate::tiles::TerrainMesh;

mod cube;

pub use cube::Cube;

/// Contains the definition for a block on the map, and how it should be
/// rendered.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum BlockModel {
    /// An empty block, which does not render anything.
    #[default]
    Empty,

    /// A unit cube.
    Cube(Cube),
}

impl BlockModel {
    /// Draws the block into the provided mesh at the specified transform.
    pub fn draw(&self, mesh: &mut TerrainMesh, transform: Transform, occlusion: Occlusion) {
        match self {
            BlockModel::Empty => {}
            BlockModel::Cube(cube) => cube.draw(mesh, transform, occlusion),
        }
    }

    /// Gets the occluder flags for this block model.
    pub fn get_occluder_flags(&self) -> Occluder {
        match self {
            BlockModel::Empty => Occluder::empty(),
            BlockModel::Cube(_) => Occluder::all(),
        }
    }
}

/// Represents a face of a block, which contains tile information for rendering.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct TileFace {
    /// The tile index for the block face.
    pub tile_index: u32,

    /// The rotation matrix for the tile.
    pub rotation: Mat2,
}

/// A data container for all block models within a chunk.
#[derive(Debug, Clone)]
pub struct ChunkModels(Vec<BlockModel>);

impl ChunkModels {
    /// Gets the block model at the specified local position within the chunk.
    pub fn get<P: Into<LocalPos>>(&self, pos: P) -> &BlockModel {
        &self.0[pos.into().as_index()]
    }

    /// Gets a mutable reference to the block model at the specified local
    /// position within the chunk.
    pub fn get_mut<P: Into<LocalPos>>(&mut self, pos: P) -> &mut BlockModel {
        &mut self.0[pos.into().as_index()]
    }
}

impl Default for ChunkModels {
    fn default() -> Self {
        Self(vec![BlockModel::Empty; TOTAL_BLOCKS])
    }
}
