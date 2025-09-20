//! This module implements the chunk model component, containing references to
//! the child components for the chunk used for rendering.

use bevy::prelude::*;

use crate::tiles::TilesetMaterial;

/// Component for the root entity of a chunk model, containing references to
/// the child entities.
#[derive(Debug, Default, Component)]
pub struct ChunkModelRoot {
    /// Entity for the opaque mesh.
    pub opaque: Option<Entity>,

    /// Handle to the material used for the opaque mesh, if any.
    pub opaque_material: Option<Handle<TilesetMaterial>>,
}

/// A multi-part mesh generated from a voxel chunk.
#[derive(Debug, Default)]
pub struct ChunkMesh {
    /// The opaque part of the mesh, if it exists.
    pub opaque: Option<Mesh>,
}
