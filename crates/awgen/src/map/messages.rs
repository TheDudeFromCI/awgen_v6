//! Messages related to map and chunk updates.

use bevy::prelude::*;

/// A message sent when a chunk's mesh has been updated.
#[derive(Debug, Message)]
pub struct ChunkMeshUpdated;

/// A message sent when a new chunk has been created.
#[derive(Debug, Message)]
pub struct ChunkCreated;

/// A message sent when a chunk has been removed.
#[derive(Debug, Message)]
pub struct ChunkRemoved;
