//! This module implements tileset functionality to Awgen.

use bevy::asset::embedded_asset;
use bevy::prelude::*;

mod asset_loader;
pub mod builder;
mod material;
mod mesh;
mod resource;
mod tileset;

pub use material::TilesetMaterial;
pub use mesh::{TerrainMesh, TerrainPoly, TerrainQuad};
pub use resource::{ActiveTilesets, GeneratingTilesets};

use crate::tiles::asset_loader::TilesetAssetLoader;

/// TilesetPlugin is a Bevy plugin that provides tileset functionality. This
/// includes the loading and processing of texture arrays.
pub struct TilesetPlugin;
impl Plugin for TilesetPlugin {
    fn build(&self, app_: &mut App) {
        app_.init_asset_loader::<TilesetAssetLoader>()
            .init_resource::<ActiveTilesets>()
            .init_resource::<GeneratingTilesets>()
            .add_plugins(MaterialPlugin::<TilesetMaterial>::default())
            .add_systems(
                Update,
                (
                    resource::update_chunk_models
                        .in_set(TilesetSystemSets::UpdateActiveTilesets)
                        .run_if(resource_changed::<ActiveTilesets>),
                    resource::finish_tileset_tasks.in_set(TilesetSystemSets::FinishTasks),
                ),
            );

        embedded_asset!(app_, "shader.wgsl");
    }
}

/// System sets for tileset-related systems.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TilesetSystemSets {
    /// System set for updating active tilesets.
    UpdateActiveTilesets,

    /// System set for polling the task process of tilesets actively being
    /// generated.
    FinishTasks,
}
