//! This module defines the Bevy plugin for processing packets sent by the
//! script engine.

use std::sync::RwLock;

use bevy::prelude::*;

use crate::scripts::ScriptSockets;
use crate::scripts::handler::PacketHandler;

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
fn recv(sockets: Res<ScriptEngine>, mut handler: PacketHandler) {
    handler.check_tasks();

    while let Ok(Some(packet)) = sockets.recv() {
        info!("Received packet from script engine: {:?}", packet);
        handler.handle(packet);
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
