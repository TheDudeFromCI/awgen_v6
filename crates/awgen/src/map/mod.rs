//! This module implements map rendering functionality for Awgen.

use bevy::prelude::*;

mod chunk;
mod chunk_table;
mod diagnostics;
mod mesher;
mod messages;
mod model;
mod occlusion;
mod pos;
mod systems;

pub use chunk::{CHUNK_SIZE, TOTAL_BLOCKS, VoxelChunk};
pub use chunk_table::ChunkTable;
pub use diagnostics::{CHUNK_COUNT, MESH_COUNT, TRIANGLE_COUNT};
pub use model::BlockModel;
pub use occlusion::Occlusion;
pub use pos::{ChunkPos, WorldPos};

/// This plugin is responsible for rendering the map in the Awgen application.
pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins(diagnostics::MapDiagnosticsPlugin)
            .init_resource::<chunk_table::ChunkTable>()
            .add_message::<messages::ChunkMeshUpdated>()
            .add_message::<messages::ChunkCreated>()
            .add_message::<messages::ChunkRemoved>()
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
