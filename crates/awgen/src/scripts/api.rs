//! This module contains API that can be used by the script engine to interact
//! with the game client.

use std::pin::Pin;
use std::sync::Arc;

use bevy::log::debug;
use rustyscript::Runtime;
use smol::channel::{Receiver, Sender};

use crate::scripts::{PacketIn, PacketOut};

/// Registers the API functions with the script engine runtime.
pub fn register(
    runtime: &mut Runtime,
    socket: Arc<Receiver<PacketOut>>,
    send_to_client: Sender<PacketIn>,
) -> Result<(), rustyscript::Error> {
    // Register sockets
    runtime.register_async_function(
        "fetchPacket",
        move |_: Vec<serde_json::Value>| -> Pin<
            Box<dyn std::future::Future<Output = Result<serde_json::Value, rustyscript::Error>>>,
        > {
            let local = socket.clone();
            Box::pin(async move {
                let packet = local.recv().await.map_err(|_| {
                    rustyscript::Error::Runtime("Failed to receive packet".to_string())
                })?;
                serde_json::to_value(packet).map_err(|e| {
                    rustyscript::Error::Runtime(format!("Failed to parse packet: {e}"))
                })
            })
        },
    )?;
    runtime.register_function(
        "sendPackets",
        move |args: &[serde_json::Value]| -> Result<serde_json::Value, rustyscript::Error> {
            debug!("Sending packets to client: {:?}", args);
            for arg in args {
                let packet = serde_json::from_value::<PacketIn>(arg.clone()).map_err(|e| {
                    rustyscript::Error::Runtime(format!("Failed to parse packet: {e}"))
                })?;
                send_to_client.send_blocking(packet).map_err(|_| {
                    rustyscript::Error::Runtime("Failed to send packet".to_string())
                })?;
            }

            Ok(serde_json::Value::Null)
        },
    )?;

    Ok(())
}
