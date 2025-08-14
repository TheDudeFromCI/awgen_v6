//! This module defines the Bevy plugin for processing packets sent by the
//! script engine.

use std::sync::RwLock;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use regex::Regex;

use crate::app::ProjectSettings;
use crate::scripts::{PacketIn, ScriptSockets};

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

/// A struct used for processing and handling packets from the script engine.
#[derive(SystemParam)]
struct PacketHandler<'w> {
    /// Send app_exit events.
    app_exit: EventWriter<'w, AppExit>,

    /// The project settings.
    project_settings: Res<'w, ProjectSettings>,
}

impl PacketHandler<'_> {
    /// Handles the shutdown packet by sending an [`AppExit`] event
    fn shutdown(&mut self) {
        self.app_exit.write(AppExit::Success);
    }

    /// Handles the import asset packet by copying the asset file into the
    /// project.
    fn import_asset(&mut self, file: String, asset_path: String) {
        let regex =
            Regex::new(r"^(game|editor)://(([A-Za-z0-9_]+/)*)([A-Za-z0-9_]+\.[A-Za-z0-9_]+)$")
                .unwrap();

        match regex.captures(&asset_path) {
            Some(caps) => {
                let asset_name = &caps[4];
                let dirs = &caps[2];
                let source = &caps[1];

                let mut dest_path = self.project_settings.project_folder().to_path_buf();
                if source == "editor" {
                    dest_path.push("editor");
                }

                dest_path.push("assets");
                if !dirs.is_empty() {
                    for part in dirs.trim_end_matches('/').split('/') {
                        dest_path.push(part);
                    }
                }

                if let Err(err) = std::fs::create_dir_all(&dest_path) {
                    error!(
                        "Failed to create directories for asset path {}: {}",
                        dest_path.display(),
                        err
                    );
                    return;
                };

                dest_path.push(asset_name);

                if let Err(err) = std::fs::copy(&file, &dest_path) {
                    error!(
                        "Failed to copy asset file from {} to {}: {}",
                        file,
                        dest_path.display(),
                        err
                    );
                }

                info!("Imported asset from {} as {}", file, asset_path);
            }
            None => {
                error!("Invalid asset path format: {}", asset_path);
            }
        }
    }
}

/// A Bevy system that receives packets from the script engine, if any, and
/// processes them.
fn recv(sockets: Res<ScriptEngine>, mut handler: PacketHandler) {
    while let Ok(Some(packet)) = sockets.recv() {
        info!("Received packet from script engine: {:?}", packet);
        handle(packet, &mut handler);
    }
}

/// Handles a single packet from the script engine.
fn handle(packet: PacketIn, handler: &mut PacketHandler) {
    match packet {
        PacketIn::Init { .. } => {
            warn!(
                "Received init packet, but this should only be sent by the script engine on startup."
            );
        }
        PacketIn::Set { packets } => {
            debug!("Received set packet with {} items.", packets.len());
            for packet in packets {
                handle(packet, handler);
            }
        }
        PacketIn::Shutdown => {
            debug!("Received shutdown packet, shutting down the app.");
            handler.shutdown();
        }
        PacketIn::ImportAsset { file, asset_path } => {
            debug!(
                "Received import asset packet: file = \"{}\", asset_path = \"{}\"",
                file, asset_path
            );
            handler.import_asset(file, asset_path);
        }
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
