//! This module implements map rendering functionality for Awgen.

use bevy::prelude::*;

mod block;
mod chunk;
mod chunk_model;
mod chunk_table;
mod mesher;
mod model;
mod pos;
mod systems;

pub use block::{Block, BlockMut};
pub use chunk::{CHUNK_SIZE, VoxelChunk, VoxelChunkModel};
pub use chunk_model::ChunkModelRoot;
pub use model::{BlockModel, QuadFace};
pub use pos::{BlockPos, ChunkPos, WorldPos};

/// This plugin is responsible for rendering the map in the Awgen application.
pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app_: &mut App) {
        app_.init_resource::<chunk_table::ChunkTable>()
            .add_systems(
                Update,
                systems::redraw_chunks.in_set(MapSystemSets::RedrawChunks),
            )
            .add_observer(systems::on_chunk_spawn)
            .add_observer(systems::on_chunk_despawn);
    }
}

/// This enum defines the system sets used in the map plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum MapSystemSets {
    /// System set for redrawing chunks in the map.
    RedrawChunks,
}
