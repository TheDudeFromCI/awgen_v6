//! This module defines the Bevy plugin for processing packets sent by the
//! script engine.

use std::path::{Path, PathBuf};
use std::sync::RwLock;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use lazy_static::lazy_static;
use regex::Regex;

use crate::app::ProjectSettings;
use crate::map::{ChunkTable, VoxelChunk};
use crate::scripts::{PacketIn, ScriptSockets};
use crate::tiles::{ActiveTilesets, GeneratingTilesets, TilesetMaterial};

lazy_static! {
    static ref ASSET_PATH_REGEX: Regex =
        Regex::new(r"^(game|editor)://(([A-Za-z0-9_]+/)*)([A-Za-z0-9_]+\.[A-Za-z0-9_]+)$").unwrap();
}

/// The Bevy plugin for processing packets sent by the script engine.
pub struct ScriptEnginePlugin {
    /// The sockets used to communicate with the script engine.
    script_sockets: RwLock<Option<ScriptSockets>>,
}

impl ScriptEnginePlugin {
    /// Creates a new `ScriptEnginePlugin` with the given script sockets.
    pub fn new(script_sockets: ScriptSockets) -> Self {
        Self {
            script_sockets: RwLock::new(Some(script_sockets)),
        }
    }
}

impl Plugin for ScriptEnginePlugin {
    fn build(&self, app_: &mut App) {
        let sockets = self.script_sockets.write().unwrap().take().unwrap();

        app_.insert_resource(ScriptEngine(sockets))
            .add_systems(PreUpdate, recv)
            .add_systems(Last, cleanup);
    }
}

/// A resource that holds the script engine sockets, allowing systems to
/// send and receive packets from the script engine.
#[derive(Resource, Deref, DerefMut)]
pub struct ScriptEngine(ScriptSockets);

/// A Bevy system that receives packets from the script engine, if any, and
/// processes them.
#[allow(clippy::type_complexity)]
fn recv(world: &mut World) {
    while let Ok(Some(packet)) = world.resource::<ScriptEngine>().recv() {
        let _ = handle(world, packet);
    }
}

/// Cleans up the script engine sockets when the application exits, joining the
/// thread and handling any errors that may occur during shutdown.
fn cleanup(mut app_exit: ResMut<Events<AppExit>>, mut sockets: ResMut<ScriptEngine>) {
    if !app_exit.is_empty() {
        info!("Cleaning up script engine sockets.");

        sockets.shutdown();
        if let Err(err) = sockets.join() {
            error!("Script engine thread panicked: {}", err);
            app_exit.send(AppExit::from_code(1));
        }
    }
}

/// Handles incoming packets from the script engine.
fn handle(world: &mut World, packet: PacketIn) -> Result<(), ()> {
    match packet {
        PacketIn::Init { .. } => {
            warn!(
                "Received init packet, but this should only be sent by the script engine on startup."
            );
        }
        PacketIn::Set { packets } => {
            debug!("Received set packet with {} items.", packets.len());
            for packet in packets {
                let _ = handle(world, packet);
            }
        }
        PacketIn::Shutdown => {
            info!("Shutting down the game engine.");
            world.send_event(AppExit::Success);
        }
        PacketIn::Crashed { error } => {
            error!("The script engine has crashed: {}", error);
            world.send_event(AppExit::from_code(1));
        }
        PacketIn::ImportAsset { file, asset_path } => {
            info!("Importing file \"{}\" as \"{}\"", file, asset_path);

            let project_folder = world.resource::<ProjectSettings>().project_folder();
            let dest_path = parse_asset_path(project_folder, &asset_path)?;

            if let Err(err) = std::fs::copy(&file, &dest_path) {
                error!(
                    "Failed to copy asset file from {} to {}: {}",
                    file,
                    dest_path.display(),
                    err
                );
                return Err(());
            }

            debug!("Imported asset from {} as {}", file, asset_path);
        }
        PacketIn::CreateTileset {
            tile_paths,
            output_path,
        } => {
            info!(
                "Received create tileset packet: tile_paths = {:?}, asset_path = {}",
                tile_paths, output_path
            );

            if !output_path.ends_with(".tiles") {
                error!(
                    "Tilesets must have a '.tiles' extension. Found: {}",
                    output_path
                );
                return Err(());
            }

            let project_folder = world.resource::<ProjectSettings>().project_folder();
            let tile_paths = tile_paths
                .iter()
                .map(|path| parse_asset_path(project_folder, path))
                .collect::<Result<Vec<PathBuf>, ()>>()?;
            let asset_path = parse_asset_path(project_folder, &output_path)?;

            let handle = world
                .resource::<AssetServer>()
                .get_handle(output_path)
                .unwrap_or_else(|| world.resource_mut::<Assets<Image>>().reserve_handle());

            let thread_pool = AsyncComputeTaskPool::get();
            let task = thread_pool.spawn(async move {
                (
                    handle,
                    crate::tiles::builder::create_tileset(tile_paths, asset_path),
                )
            });
            world.resource_mut::<GeneratingTilesets>().add_task(task);
        }
        PacketIn::SetTilesets {
            opaque_tileset_path,
        } => {
            info!(
                "Received set tilesets packet: opaque_tileset_path = {}",
                opaque_tileset_path
            );

            let asset_server = world.resource::<AssetServer>();
            let opaque_img_handle = asset_server.load(&opaque_tileset_path);

            let mut materials = world.resource_mut::<Assets<TilesetMaterial>>();
            let opaque_mat_handle = materials.add(TilesetMaterial {
                texture: opaque_img_handle,
                alpha_mode: AlphaMode::Opaque,
            });

            let mut active_tilesets = world.resource_mut::<ActiveTilesets>();
            active_tilesets.opaque = opaque_mat_handle;
        }
        PacketIn::SetBlock { pos, model } => {
            let chunk_pos = pos.as_chunk_pos();
            match world.resource::<ChunkTable>().get_chunk(chunk_pos) {
                Some(chunk_id) => {
                    if let Some(mut chunk) = world.get_mut::<VoxelChunk>(chunk_id) {
                        *chunk.get_models_mut().get_mut(pos) = *model;
                    } else {
                        error!("Failed to get chunk at position {chunk_pos} to set block at {pos}");
                    }
                }
                None => {
                    let mut chunk = VoxelChunk::new(chunk_pos);
                    *chunk.get_models_mut().get_mut(pos) = *model;
                    let chunk_id = world.spawn(chunk).id();
                    world
                        .resource_mut::<ChunkTable>()
                        .add_chunk(chunk_pos, chunk_id);

                    info!("Created new chunk at position {chunk_pos}");
                }
            };
        }
    };
    Ok(())
}

/// Attempts to parse the given string as an asset path. This function will also
/// automatically create the necessary directories for the asset if they do not
/// already exist.
///
/// Errors are logged if the asset path is invalid or if there are issues
/// creating directories or copying the asset file.
///
/// If the string is a valid asset path, it returns a `PathBuf` representing the
/// file path that asset is located at. If the string does not match the
/// expected format, it returns an error.
fn parse_asset_path(project_folder: &Path, asset_path: &str) -> Result<PathBuf, ()> {
    match ASSET_PATH_REGEX.captures(asset_path) {
        Some(caps) => {
            let asset_name = &caps[4];
            let dirs = &caps[2];
            let source = &caps[1];

            let mut file_path = project_folder.to_path_buf();
            if source == "editor" {
                file_path.push("editor");
            }

            file_path.push("assets");
            if !dirs.is_empty() {
                for part in dirs.trim_end_matches('/').split('/') {
                    file_path.push(part);
                }
            }

            if let Err(err) = std::fs::create_dir_all(&file_path) {
                error!(
                    "Failed to create directories for asset path {}: {}",
                    file_path.display(),
                    err
                );
                return Err(());
            };

            file_path.push(asset_name);
            Ok(file_path)
        }
        None => {
            error!("Invalid asset path format: {}", asset_path);
            Err(())
        }
    }
}
