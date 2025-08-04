//! This module defines the Bevy plugin for processing packets sent by the
//! script engine.

use std::sync::RwLock;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

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
    fn build(&self, app: &mut App) {
        let sockets = self.script_sockets.write().unwrap().take().unwrap();

        app.insert_resource(ScriptEngine(sockets))
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
}

impl PacketHandler<'_> {
    /// Handles the shutdown packet.
    fn shutdown(&mut self) {
        self.app_exit.send(AppExit::Success);
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
