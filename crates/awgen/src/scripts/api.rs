//! This module contains API that can be used by the script engine to interact
//! with the game client.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use rustyscript::{Error, Runtime};
use serde_json::Value;
use smol::channel::{Receiver, Sender};

use crate::database::Database;
use crate::scripts::{PacketIn, PacketOut};

/// Registers the API functions with the script engine runtime.
pub fn register(
    runtime: &mut Runtime,
    socket: Arc<Receiver<PacketOut>>,
    send_to_client: Sender<PacketIn>,
    database: Arc<Database>,
) -> Result<(), rustyscript::Error> {
    // Register sockets functions

    runtime.register_async_function(
        "fetchPacket",
        move |args: Vec<Value>| -> Pin<Box<dyn Future<Output = Result<Value, Error>>>> {
            let local = socket.clone();
            Box::pin(async move {
                if !args.is_empty() {
                    return Err(Error::Runtime("Expected: fetchPacket()".to_string()));
                }

                let packet = local
                    .recv()
                    .await
                    .map_err(|_| Error::Runtime("Failed to receive packet".to_string()))?;
                serde_json::to_value(packet)
                    .map_err(|e| Error::Runtime(format!("Failed to parse packet: {e}")))
            })
        },
    )?;

    runtime.register_function(
        "sendPackets",
        move |args: &[Value]| -> Result<Value, Error> {
            if args.is_empty() {
                return Ok(Value::Null);
            }

            let mut packets = vec![];

            for arg in args {
                let packet = serde_json::from_value::<PacketIn>(arg.clone())
                    .map_err(|e| Error::Runtime(format!("Failed to parse packet: {e}")))?;
                packets.push(packet);
            }

            if packets.len() == 1 {
                send_to_client
                    .send_blocking(packets.into_iter().next().unwrap())
                    .map_err(|_| Error::Runtime("Failed to send packet".to_string()))?;
            } else {
                let compound = PacketIn::Set { packets };
                send_to_client
                    .send_blocking(compound)
                    .map_err(|_| Error::Runtime("Failed to send packet".to_string()))?;
            }

            Ok(Value::Null)
        },
    )?;

    // Register database functions

    let db1 = database.clone();
    runtime.register_function(
        "getSetting",
        move |args: &[Value]| -> Result<Value, Error> {
            if args.len() != 1 {
                return Err(Error::Runtime("Expected: getSetting(key)".to_string()));
            }

            let key = args[0]
                .as_str()
                .ok_or_else(|| Error::Runtime("Key must be a string".to_string()))?;

            let value = db1
                .get_setting(key)
                .map_err(|e| Error::Runtime(format!("Failed to get setting: {e}")))?;

            let value = serde_json::to_value(value)
                .map_err(|e| Error::Runtime(format!("Failed to serialize setting: {e}")))?;

            Ok(value)
        },
    )?;

    let db2 = database.clone();
    runtime.register_function(
        "setSetting",
        move |args: &[Value]| -> Result<Value, Error> {
            if args.len() != 2 {
                return Err(Error::Runtime(
                    "Expected: setSetting(key, value)".to_string(),
                ));
            }

            let key = args[0]
                .as_str()
                .ok_or_else(|| Error::Runtime("Key must be a string".to_string()))?;

            if args[1].is_null() {
                db2.clear_setting(key)
                    .map_err(|e| Error::Runtime(format!("Failed to clear setting: {e}")))?;
            } else {
                let value = args[1]
                    .as_str()
                    .ok_or_else(|| Error::Runtime("Value must be a string".to_string()))?;

                db2.set_setting(key, value)
                    .map_err(|e| Error::Runtime(format!("Failed to set setting: {e}")))?;
            }

            Ok(Value::Null)
        },
    )?;

    Ok(())
}
