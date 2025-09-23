//! This module handles packets from the script engine, processing them and
//! executing the appropriate actions based on the packet type.

use std::path::{Path, PathBuf};

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on};
use lazy_static::lazy_static;
use regex::Regex;
use smol::future;

use crate::app::ProjectSettings;
use crate::map::{BlockModel, ChunkTable, VoxelChunk, WorldPos};
use crate::scripts::PacketIn;
use crate::tiles::builder::TilesetBuilderError;
use crate::tiles::{ActiveTilesets, TilesetMaterial};

lazy_static! {
    static ref ASSET_PATH_REGEX: Regex =
        Regex::new(r"^(game|editor)://(([A-Za-z0-9_]+/)*)([A-Za-z0-9_]+\.[A-Za-z0-9_]+)$").unwrap();
}

/// A struct used for processing and handling packets from the script engine.
#[derive(SystemParam)]
pub struct PacketHandler<'w, 's> {
    /// A list of potential tileset building tasks that may be running in the
    /// background.
    #[allow(clippy::type_complexity)]
    tileset_tasks: Local<'s, Vec<Task<(Handle<Image>, Result<Image, TilesetBuilderError>)>>>,

    /// Send app_exit events.
    app_exit: EventWriter<'w, AppExit>,

    /// The project settings.
    project_settings: Res<'w, ProjectSettings>,

    /// The asset server.
    asset_server: Res<'w, AssetServer>,

    /// The resource containing all image assets.
    images: ResMut<'w, Assets<Image>>,

    /// The resource containing all tileset materials.
    tileset_materials: ResMut<'w, Assets<TilesetMaterial>>,

    /// The commands to modify the world.
    commands: Commands<'w, 's>,

    /// The chunk table resource.
    chunk_table: Res<'w, ChunkTable>,

    /// A query of all voxel chunks.
    chunks: Query<'w, 's, &'static mut VoxelChunk>,

    /// The resource containing the currently active tilesets.
    active_tilesets: ResMut<'w, ActiveTilesets>,
}

impl PacketHandler<'_, '_> {
    /// Processes the given packet from the script engine and executes the
    /// appropriate action based on the packet type.
    pub fn handle(&mut self, packet: PacketIn) {
        let _ = handle(self, packet);
    }

    /// Checks all (system local) background tasks that might be running and
    /// finishes them if they are done.
    ///
    /// This method should be called every frame.
    pub fn check_tasks(&mut self) {
        check_tasks(self);
    }
}

/// Checks all background tasks that might be running and finishes them if they
/// are done.
fn check_tasks(handler: &mut PacketHandler) {
    handler.tileset_tasks.retain_mut(|task| {
        if let Some((handle, result)) = block_on(future::poll_once(task)) {
            match result {
                Ok(image) => {
                    info!("Tileset creation task completed successfully.");

                    if let Some(img_asset) = handler.images.get_mut(&handle) {
                        *img_asset = image;

                        // iter_mut() will force all materials to be updated
                        for _ in handler.tileset_materials.iter_mut() {}
                    };
                }
                Err(err) => {
                    error!("Failed to create tileset: {}", err);
                }
            }

            return false;
        }

        true
    });
}

/// Handles incoming packets from the script engine.
fn handle(handler: &mut PacketHandler, packet: PacketIn) -> Result<(), ()> {
    match packet {
        PacketIn::Init { .. } => init(),
        PacketIn::Set { packets } => set(handler, packets),
        PacketIn::Shutdown => shutdown(handler),
        PacketIn::ImportAsset { file, asset_path } => import_asset(handler, file, asset_path)?,
        PacketIn::CreateTileset {
            tile_paths,
            output_path,
        } => create_tileset(handler, tile_paths, output_path)?,
        PacketIn::SetTilesets {
            opaque_tileset_path,
        } => set_tilesets(handler, opaque_tileset_path),
        PacketIn::SetBlock { pos, model } => set_block(handler, pos, *model),
    };
    Ok(())
}

/// Handles the initialization packet from the script engine.
fn init() {
    warn!("Received init packet, but this should only be sent by the script engine on startup.");
}

/// Handles the set packet from the script engine.
fn set(handler: &mut PacketHandler, packets: Vec<PacketIn>) {
    debug!("Received set packet with {} items.", packets.len());
    for packet in packets {
        let _ = handle(handler, packet);
    }
}

/// Handles the shutdown packet from the script engine.
fn shutdown(handler: &mut PacketHandler) {
    info!("Shutting down the game engine.");
    handler.app_exit.write(AppExit::Success);
}

/// Handles the import asset packet from the script engine.
fn import_asset(handler: &mut PacketHandler, file: String, asset_path: String) -> Result<(), ()> {
    info!("Importing file \"{}\" as \"{}\"", file, asset_path);

    let dest_path = parse_asset_path(handler.project_settings.project_folder(), &asset_path)?;

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
    Ok(())
}

/// Handles the create tileset packet from the script engine.
fn create_tileset(
    handler: &mut PacketHandler,
    tile_paths: Vec<String>,
    asset_path: String,
) -> Result<(), ()> {
    info!(
        "Received create tileset packet: tile_paths = {:?}, asset_path = {}",
        tile_paths, asset_path
    );

    if !asset_path.ends_with(".tiles") {
        error!(
            "Tilesets must have a '.tiles' extension. Found: {}",
            asset_path
        );
        return Err(());
    }

    let project_folder = handler.project_settings.project_folder();
    let tile_paths = tile_paths
        .iter()
        .map(|path| parse_asset_path(project_folder, path))
        .collect::<Result<Vec<PathBuf>, ()>>()?;
    let output_path = parse_asset_path(project_folder, &asset_path)?;

    let handle = handler
        .asset_server
        .get_handle(&asset_path)
        .unwrap_or_else(|| handler.images.reserve_handle());

    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move {
        (
            handle,
            crate::tiles::builder::create_tileset(tile_paths, output_path),
        )
    });
    handler.tileset_tasks.push(task);

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

/// Handles the set tilesets packet from the script engine.
fn set_tilesets(handler: &mut PacketHandler, opaque_tileset_path: String) {
    info!(
        "Received set tilesets packet: opaque_tileset_path = {}",
        opaque_tileset_path
    );

    handler.active_tilesets.opaque = handler.tileset_materials.add(TilesetMaterial {
        texture: handler.asset_server.load(&opaque_tileset_path),
        alpha_mode: AlphaMode::Opaque,
    });
}

/// Handles the set block packet from the script engine.
fn set_block(handler: &mut PacketHandler, pos: WorldPos, model: BlockModel) {
    let chunk_pos = pos.as_chunk_pos();

    match handler.chunk_table.get_chunk(chunk_pos) {
        Some(chunk_id) => {
            if let Ok(mut chunk) = handler.chunks.get_mut(chunk_id) {
                *chunk.get_models_mut().get_mut(pos) = model;
            } else {
                error!("Failed to get chunk at position {chunk_pos} to set block at {pos}");
            }
        }
        None => {
            let mut chunk = VoxelChunk::new(chunk_pos);
            *chunk.get_models_mut().get_mut(pos) = model;
            handler.commands.spawn(chunk);

            info!("Created new chunk at position {chunk_pos}");
        }
    };
}
