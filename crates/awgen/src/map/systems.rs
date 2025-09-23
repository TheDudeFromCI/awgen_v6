//! Systems for managing the map in the game.

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on};

use crate::map::chunk_table::ChunkTable;
use crate::map::mesher::{ChunkMesh, build_mesh};
use crate::map::{ChunkPos, VoxelChunk};
use crate::tiles::{ActiveTilesets, TilesetMaterial};

/// This system updates every frame to redraw all chunks that have been marked
/// for redraw.
pub(super) fn redraw_chunks(
    mut active_tasks: Local<Vec<Task<(ChunkPos, ChunkMesh)>>>,
    chunk_table: Res<ChunkTable>,
    active_tilesets: Res<ActiveTilesets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<&mut VoxelChunk>,
    mut chunk_models: Query<(&mut Mesh3d, &mut MeshMaterial3d<TilesetMaterial>)>,
    mut commands: Commands,
) {
    // Wait on all pending redraw tasks to avoid flickering.
    let finished_tasks = block_on(futures::future::join_all(active_tasks.drain(..)));

    for (pos, chunk_mesh) in finished_tasks {
        let Some(chunk_id) = chunk_table.get_chunk(pos) else {
            continue;
        };

        let Ok(mut chunk) = chunks.get_mut(chunk_id) else {
            continue;
        };

        match (chunk.opaque_entity, chunk_mesh.opaque) {
            (None, None) => {}
            (None, Some(mesh)) => {
                let entity = commands
                    .spawn((
                        ChildOf(chunk_id),
                        Mesh3d(meshes.add(mesh)),
                        MeshMaterial3d(active_tilesets.opaque.clone()),
                    ))
                    .id();
                chunk.opaque_entity = Some(entity);
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
    for mut chunk in chunks.iter_mut() {
        if !chunk.is_dirty() {
            continue;
        }
        chunk.mark_clean();

        let position = chunk.pos();
        let chunk_model = chunk.get_models().clone();
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

    if let Some(existing_chunk) = chunk_table.get_chunk(pos) {
        if existing_chunk != entity {
            error!("ChunkTable already has a chunk at position {pos}");
        }
    } else {
        chunk_table.add_chunk(pos, entity);
    }
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
