//! This module defines the `PacketIn` enum, which is used to represent
//! different types of incoming packets that may be received from the script
//! engine.
//!
//! *NOTE:* When adding new variants to this enum, newtype variants should not
//! be used. These will cause serde to fail to serialize the enum.

use serde::{Deserialize, Serialize};

use crate::map::{BlockModel, WorldPos};

/// The `PacketIn` enum, which is used to represent different types of
/// incoming packets that may be received from the script engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum PacketIn {
    /// A packet that contains the initial game state settings, which is sent
    /// when the script engine starts up. This packet must always be the first
    /// packet sent by the script engine. Subsequent packets of this type are
    /// ignored.
    Init {
        /// The name of the game.
        name: String,

        /// The game version.
        version: String,
    },

    /// A packet that contains a collection of packets from the
    /// script engine that should be processed on the same frame.
    Set {
        /// The packets that should be processed.
        packets: Vec<PacketIn>,
    },

    /// Requests for the app to shutdown safely.
    Shutdown,

    /// A packet that indicates that the script engine has crashed.
    ///
    /// This packet should never be sent under normal operation, and is
    /// generated automatically if the script engine panics or encounters
    /// an unrecoverable error.
    Crashed {
        /// The error message associated with the crash.
        error: String,
    },

    /// Import an asset file into the project directory.
    ImportAsset {
        /// The OS filepath of the asset file to import.
        file: String,

        /// The local asset path to use within the project.
        asset_path: String,
    },

    /// Creates a new tileset from a list of tile asset paths.
    ///
    /// This packet will fail if the tiles cannot be loaded or if they are not
    /// valid tile assets of equal size.
    CreateTileset {
        /// The list of asset paths for the corresponding tiles.
        tile_paths: Vec<String>,

        /// The output asset path for the tileset.
        output_path: String,
    },

    /// Sets the tilesets currently in use for the world.
    SetTilesets {
        /// The asset path of the tileset to use for the world.
        opaque_tileset_path: String,
    },

    /// Sets the block model at the specified world position.
    SetBlock {
        /// The world position.
        pos: WorldPos,

        /// The block model.
        model: Box<BlockModel>,
    },
}
