//! This module implements system for loading tilesets.

use bevy::prelude::*;

use crate::tileset::asset::TilesetEditorError;
use crate::tileset::{Tileset, TilesetMaterial};

/// This method will check all tilesets for pending tiles that need to be loaded
/// and attempt to load them if possible.
///
/// This method will only load up to one tile per tileset per frame.
pub(super) fn load_pending_tiles(
    mut tilesets: ResMut<Assets<Tileset>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<TilesetMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (_, tileset) in tilesets.iter_mut() {
        let Some(next) = tileset.add_when_loaded.first() else {
            continue;
        };
        let source = next.clone_weak();

        let mut editor = tileset.edit(&mut images, &asset_server);
        match editor.add_tile(&source) {
            Ok(_) => {
                // No error, tile was added successfully.
                // Mutably reference all materials to ensure they are updated.
                for (_, _) in materials.iter_mut() {}
            }
            Err(TilesetEditorError::TileLoading(..)) => {
                // Still loading.
                continue;
            }
            Err(err) => {
                // Print error and remove the tile from the list.
                error!("Failed to add tile ({:?}) to tileset: {}", source, err);
            }
        }

        tileset.add_when_loaded.remove(0);
    }
}
