//! This module implements tileset functionality to Awgen.

use bevy::asset::embedded_asset;
use bevy::prelude::*;

mod asset;
mod material;
mod mesh;
mod systems;

pub use asset::Tileset;
pub use material::TilesetMaterial;
pub use mesh::{TerrainMesh, TerrainQuad, TerrainTriangle, TerrainVertex};

/// TilesetPlugin is a Bevy plugin that provides tileset functionality. This
/// includes the loading and processing of texture arrays.
pub struct TilesetPlugin;
impl Plugin for TilesetPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins(MaterialPlugin::<TilesetMaterial>::default())
            .init_asset::<Tileset>()
            .add_systems(
                Update,
                systems::load_pending_tiles.in_set(TilesetSystemSet::TryLoadPendingTiles),
            );

        embedded_asset!(app_, "shader.wgsl");
    }
}

/// The system sets for the camera plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum TilesetSystemSet {
    /// This system set is used to load pending tiles in the tileset.
    TryLoadPendingTiles,
}
