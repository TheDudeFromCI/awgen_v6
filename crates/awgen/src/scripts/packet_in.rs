//! This module defines the `PacketIn` enum, which is used to represent
//! different types of incoming packets that may be received from the script
//! engine.
//!
//! *NOTE:* When adding new variants to this enum, newtype variants should not
//! be used. These will cause serde to fail to serialize the enum.

use serde::{Deserialize, Serialize};

/// The `PacketIn` enum, which is used to represent different types of
/// incoming packets that may be received from the script engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
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
}
