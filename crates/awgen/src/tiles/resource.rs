//! This module implements the [`ActiveTilesets`] resource to Awgen.

use bevy::prelude::*;
use bevy::tasks::{Task, block_on, poll_once};

use crate::map::VoxelChunk;
use crate::tiles::TilesetMaterial;
use crate::tiles::builder::TilesetBuilderError;

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

/// This resource tracks tilesets that are currently being generated.
#[derive(Debug, Default, Resource)]
pub struct GeneratingTilesets {
    /// The tasks that are currently being processed to generate tilesets.
    #[allow(clippy::type_complexity)]
    tasks: Vec<Task<(Handle<Image>, Result<Image, TilesetBuilderError>)>>,
}

impl GeneratingTilesets {
    /// Add a new tileset generation task.
    pub fn add_task(&mut self, task: Task<(Handle<Image>, Result<Image, TilesetBuilderError>)>) {
        self.tasks.push(task);
    }
}

/// System to poll and finish tileset generation tasks.
pub(super) fn finish_tileset_tasks(
    mut generating: ResMut<GeneratingTilesets>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<TilesetMaterial>>,
) {
    generating.tasks.retain_mut(|task| {
        if let Some((handle, result)) = block_on(poll_once(task)) {
            match result {
                Ok(image) => {
                    info!("Tileset creation task completed successfully.");

                    if let Some(img_asset) = images.get_mut(&handle) {
                        *img_asset = image;

                        // iter_mut() will force all materials to be updated
                        for _ in materials.iter_mut() {}
                    };
                }
                Err(err) => {
                    error!("Failed to create tileset: {}", err);
                }
            }

            return false;
        }

        true
    });
}
