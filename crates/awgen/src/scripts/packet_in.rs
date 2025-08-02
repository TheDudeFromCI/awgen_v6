//! This module defines the `PacketIn` enum, which is used to represent
//! different types of incoming packets that may be received from the script
//! engine.

use serde::{Deserialize, Serialize};

/// The `PacketIn` enum, which is used to represent different types of
/// incoming packets that may be received from the script engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketIn {}
