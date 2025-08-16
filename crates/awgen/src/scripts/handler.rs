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
use crate::scripts::PacketIn;
use crate::tileset::builder::TilesetBuilderError;

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
    tileset_tasks: Local<'s, Vec<Task<Result<(), TilesetBuilderError>>>>,

    /// Send app_exit events.
    app_exit: EventWriter<'w, AppExit>,

    /// The project settings.
    project_settings: Res<'w, ProjectSettings>,
}

impl PacketHandler<'_, '_> {
    /// Processes the given packet from the script engine and executes the
    /// appropriate action based on the packet type.
    pub fn handle(&mut self, packet: PacketIn) {
        handle(self, packet);
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
        if let Some(result) = block_on(future::poll_once(task)) {
            match result {
                Ok(_) => {
                    info!("Tileset creation task completed successfully.");
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
fn handle(handler: &mut PacketHandler, packet: PacketIn) {
    match packet {
        PacketIn::Init { .. } => init(),
        PacketIn::Set { packets } => set(handler, packets),
        PacketIn::Shutdown => shutdown(handler),
        PacketIn::ImportAsset { file, asset_path } => {
            let _ = import_asset(handler, file, asset_path);
        }
        PacketIn::CreateTileset {
            tile_paths,
            output_path,
        } => {
            let _ = create_tileset(handler, tile_paths, output_path);
        }
    }
}

/// Handles the initialization packet from the script engine.
fn init() {
    warn!("Received init packet, but this should only be sent by the script engine on startup.");
}

/// Handles the set packet from the script engine.
fn set(handler: &mut PacketHandler, packets: Vec<PacketIn>) {
    debug!("Received set packet with {} items.", packets.len());
    for packet in packets {
        handle(handler, packet);
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

    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool
        .spawn(async move { crate::tileset::builder::create_tileset(tile_paths, output_path) });
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
