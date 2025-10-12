//! This module implements the diagnostics for world processing.

use bevy::diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use bevy::prelude::*;

use crate::map::chunk::ChunkModelPart;
use crate::map::messages::{ChunkCreated, ChunkMeshUpdated, ChunkRemoved};
use crate::map::{ChunkTable, VoxelChunk};

/// The name of the chunk count diagnostic.
pub const CHUNK_COUNT: DiagnosticPath = DiagnosticPath::const_new("map/chunk_count");

/// The name of the mesh count diagnostic.
pub const MESH_COUNT: DiagnosticPath = DiagnosticPath::const_new("map/mesh_count");

/// The name of the triangle count diagnostic.
pub const TRIANGLE_COUNT: DiagnosticPath = DiagnosticPath::const_new("map/triangle_count");

/// The plugin that adds map diagnostics to the application.
pub struct MapDiagnosticsPlugin;
impl Plugin for MapDiagnosticsPlugin {
    fn build(&self, app_: &mut App) {
        app_.register_diagnostic(Diagnostic::new(CHUNK_COUNT).with_max_history_length(1))
            .register_diagnostic(Diagnostic::new(MESH_COUNT).with_max_history_length(1))
            .register_diagnostic(Diagnostic::new(TRIANGLE_COUNT).with_max_history_length(1))
            .add_systems(Update, (mesh_updates, chunks_updated));
    }
}

/// Updates the map mesh diagnostics when a chunk mesh is updated.
fn mesh_updates(
    mut mesh_update_msg: MessageReader<ChunkMeshUpdated>,
    chunks: Query<&VoxelChunk>,
    model_parts: Query<&ChunkModelPart>,
    mut diagnostics: Diagnostics,
) {
    if mesh_update_msg.read().next().is_none() {
        return;
    }

    diagnostics.add_measurement(&MESH_COUNT, || {
        let mut mesh_count = 0;
        for chunk in chunks.iter() {
            if chunk.opaque_entity.is_some() {
                mesh_count += 1;
            }
        }

        mesh_count as f64
    });

    diagnostics.add_measurement(&TRIANGLE_COUNT, || {
        let mut triangles = 0;
        for chunk in chunks.iter() {
            if let Some(entity) = chunk.opaque_entity {
                if let Ok(part) = model_parts.get(entity) {
                    triangles += part.triangles;
                }
            }
        }

        triangles as f64
    });
}

/// Updates the chunk count diagnostic when chunks are created or removed.
fn chunks_updated(
    chunk_created_msg: MessageReader<ChunkCreated>,
    chunk_removed_msg: MessageReader<ChunkRemoved>,
    chunk_table: Res<ChunkTable>,
    mut diagnostics: Diagnostics,
) {
    if chunk_created_msg.is_empty() && chunk_removed_msg.is_empty() {
        return;
    }

    diagnostics.add_measurement(&CHUNK_COUNT, || chunk_table.len() as f64);
}
