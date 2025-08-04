//! This module defines the `PacketOut` enum, which is used to represent
//! different types of outgoing packets that may be sent to the script engine.
//!
//! *NOTE:* When adding new variants to this enum, newtype variants should not
//! be used. These will cause serde to fail to serialize the enum.

use serde::{Deserialize, Serialize};

/// The `PacketOut` enum, which is used to represent different types of
/// outgoing packets that may be sent to the script engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum PacketOut {
    /// A packet that contains the project file data. This packet is sent to
    /// the script engine on startup.
    Init {
        /// The absolute file path of the project file.
        project_folder: String,
    },

    /// A packet to request the script engine to shut down.
    Shutdown,
}
