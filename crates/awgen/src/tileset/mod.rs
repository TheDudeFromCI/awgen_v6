//! This module implements tileset functionality to Awgen.

use bevy::asset::embedded_asset;
use bevy::prelude::*;

mod asset_loader;
pub mod builder;
mod material;
mod mesh;
mod tileset;

pub use material::TilesetMaterial;
pub use mesh::{TerrainMesh, TerrainQuad, TerrainTriangle, TerrainVertex};

use crate::tileset::asset_loader::TilesetAssetLoader;

/// TilesetPlugin is a Bevy plugin that provides tileset functionality. This
/// includes the loading and processing of texture arrays.
pub struct TilesetPlugin;
impl Plugin for TilesetPlugin {
    fn build(&self, app_: &mut App) {
        app_.init_asset_loader::<TilesetAssetLoader>()
            .add_plugins(MaterialPlugin::<TilesetMaterial>::default());

        embedded_asset!(app_, "shader.wgsl");
    }
}
