//! This module generates a renderable mesh from a voxel chunk.

use bevy::prelude::*;

use crate::map::model::ChunkModels;
use crate::map::{CHUNK_SIZE, WorldPos};
use crate::tiles::TerrainMesh;

/// Generates a mesh from the given chunk.
pub fn build_mesh(chunk: &ChunkModels) -> ChunkMesh {
    let mut mesh = TerrainMesh::new();

    for x in 0 .. CHUNK_SIZE as i32 {
        for y in 0 .. CHUNK_SIZE as i32 {
            for z in 0 .. CHUNK_SIZE as i32 {
                let model = &chunk.get(WorldPos::new(x, y, z));
                let transform = Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5);
                model.draw(&mut mesh, transform);
            }
        }
    }

    let mut chunk_mesh = ChunkMesh::default();

    if !mesh.is_empty() {
        chunk_mesh.opaque = Some(mesh.into());
    }

    chunk_mesh
}

/// A multi-part mesh generated from a voxel chunk.
#[derive(Debug, Default)]
pub struct ChunkMesh {
    /// The opaque part of the mesh, if it exists.
    pub opaque: Option<Mesh>,
}
