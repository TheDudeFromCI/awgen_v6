//! Systems for managing the map in the game.

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on};

use crate::map::chunk_model::{ChunkMesh, ChunkModelRoot};
use crate::map::chunk_table::ChunkTable;
use crate::map::mesher::build_mesh;
use crate::map::{ChunkPos, VoxelChunk};
use crate::tiles::TilesetMaterial;

/// This system updates every frame to redraw all chunks that have been marked
/// for redraw.
pub(super) fn redraw_chunks(
    mut active_tasks: Local<Vec<Task<(ChunkPos, ChunkMesh)>>>,
    chunk_table: Res<ChunkTable>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<(&mut VoxelChunk, &mut ChunkModelRoot)>,
    mut chunk_models: Query<(&mut Mesh3d, &mut MeshMaterial3d<TilesetMaterial>)>,
    mut commands: Commands,
) {
    // Wait on all pending redraw tasks to avoid flickering.
    let finished_tasks = block_on(futures::future::join_all(active_tasks.drain(..)));

    for (pos, chunk_mesh) in finished_tasks {
        let Some(chunk_id) = chunk_table.get_chunk(pos) else {
            continue;
        };

        let Ok((_, mut model_root)) = chunks.get_mut(chunk_id) else {
            continue;
        };

        match (model_root.opaque, chunk_mesh.opaque) {
            (None, None) => {}
            (None, Some(mesh)) => {
                let material = model_root.opaque_material.clone().unwrap_or_default();
                let entity = commands
                    .spawn((
                        ChildOf(chunk_id),
                        Mesh3d::from(meshes.add(mesh)),
                        MeshMaterial3d::from(material),
                    ))
                    .id();
                model_root.opaque = Some(entity);
            }
            (Some(old_entity), None) => {
                commands.entity(old_entity).despawn();
            }
            (Some(old_entity), Some(mesh)) => {
                if let Ok((mut mesh_handle, _)) = chunk_models.get_mut(old_entity) {
                    *mesh_handle = Mesh3d::from(meshes.add(mesh));
                }
            }
        }
    }

    let pool = AsyncComputeTaskPool::get();
    for (mut chunk, _) in chunks.iter_mut() {
        if !chunk.is_dirty() {
            continue;
        }
        chunk.mark_clean();

        let position = chunk.pos();
        let chunk_model = chunk.get_chunk_model();
        active_tasks.push(pool.spawn(async move { (position, build_mesh(&chunk_model)) }));
    }
}

/// This observer is triggered whenever a new [`VoxelChunk`] is added to the
/// world, and adds it to the [`ChunkTable`].
pub(super) fn on_chunk_spawn(
    trigger: Trigger<OnAdd, VoxelChunk>,
    chunks: Query<&VoxelChunk>,
    mut chunk_table: ResMut<ChunkTable>,
) {
    let entity = trigger.target();
    let chunk = chunks.get(entity).unwrap();
    let pos = chunk.pos();

    if chunk_table.get_chunk(pos).is_some() {
        error!("ChunkTable already has a chunk at position {pos}");
        return;
    }

    chunk_table.add_chunk(pos, entity);
}

/// This observer is triggered whenever a [`VoxelChunk`] is removed from the
/// world, and removes it from the [`ChunkTable`].
pub(super) fn on_chunk_despawn(
    trigger: Trigger<OnRemove, VoxelChunk>,
    chunks: Query<&VoxelChunk>,
    mut chunk_table: ResMut<ChunkTable>,
) {
    let entity = trigger.target();
    let chunk = chunks.get(entity).unwrap();
    let pos = chunk.pos();
    chunk_table.remove_chunk(pos);
}
