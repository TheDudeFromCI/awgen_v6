//! This module implements the [`ActiveTilesets`] resource to Awgen.

use bevy::prelude::*;

use crate::map::VoxelChunk;
use crate::tiles::TilesetMaterial;

/// This resource contains the currently active tilesets in the application.
#[derive(Debug, Default, Resource)]
pub struct ActiveTilesets {
    /// The opaque tileset material handle.
    pub opaque: Handle<TilesetMaterial>,
}

/// System to update chunk models with the active tileset materials.
pub(super) fn update_chunk_models(
    tilesets: Res<ActiveTilesets>,
    chunks: Query<&VoxelChunk>,
    mut models: Query<&mut MeshMaterial3d<TilesetMaterial>>,
) {
    for chunk in chunks.iter() {
        let Some(opaque_entity) = chunk.opaque_entity else {
            continue;
        };

        if let Ok(mut model) = models.get_mut(opaque_entity) {
            *model = MeshMaterial3d(tilesets.opaque.clone());
        }
    }
}
