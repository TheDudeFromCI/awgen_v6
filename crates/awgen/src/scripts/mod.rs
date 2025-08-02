//! The scripting plugin for the Awgen game engine.

use std::fs;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::thread::JoinHandle;

use rustyscript::{Module, ModuleHandle, Runtime, RuntimeOptions, Undefined, json_args};
use smol::channel::{Receiver, Sender};

mod awgen_ext;
mod packet_in;
mod packet_out;

pub use packet_in::PacketIn;
pub use packet_out::PacketOut;

/// Spawns a new thread to run the script engine.
pub fn start_script_engine(project_folder: &Path) -> Result<ScriptSockets, ScriptEngineError> {
    let folder = project_folder.join("scripts");

    let (send_to_engine, get_from_client) = smol::channel::unbounded();
    let (send_to_client, get_from_engine) = smol::channel::unbounded();

    let thread = std::thread::Builder::new()
        .name("script_engine".to_string())
        .spawn(move || -> Result<(), ScriptEngineError> {
            let (mut runtime, mod_handle) =
                prepare_script_engine(&folder, send_to_client, get_from_client)?;
            runtime.call_entrypoint::<Undefined>(&mod_handle, json_args!())?;
            Ok(())
        })?;

    Ok(ScriptSockets::new(thread, send_to_engine, get_from_engine))
}

/// A recursive function that finds and loads all TypeScript modules in a folder
/// and its subfolders.
fn find_modules(
    folder: &PathBuf,
    list: &mut Vec<Module>,
    index: &Module,
) -> Result<(), std::io::Error> {
    if !folder.is_dir() {
        return Ok(());
    }

    list.extend(
        Module::load_dir(folder)?
            .into_iter()
            .filter(|module| module != index),
    );

    for entry in fs::read_dir(folder)?.flatten() {
        let path = entry.path();
        find_modules(&path, list, index)?;
    }

    Ok(())
}

/// Loads and prepares the script engine within the given script folder.
fn prepare_script_engine(
    folder: &PathBuf,
    send_to_client: Sender<PacketIn>,
    get_from_client: Receiver<PacketOut>,
) -> Result<(Runtime, ModuleHandle), ScriptEngineError> {
    let index = Module::load(folder.join("index.ts"))?;

    let mut modules = vec![];
    find_modules(folder, &mut modules, &index)?;

    let mut runtime = Runtime::new(RuntimeOptions {
        default_entrypoint: Some("main".to_string()),
        extensions: vec![awgen_ext::awgen::init_ops_and_esm()],
        ..Default::default()
    })?;

    let socket = Arc::new(get_from_client);
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

    let mod_ref = modules.iter().collect::<Vec<_>>();
    let mod_handle = runtime.load_modules(&index, mod_ref)?;
    runtime.set_current_dir(folder)?;

    Ok((runtime, mod_handle))
}

/// An error that can occur while loading, executing, or interacting with
/// scripts.
#[derive(Debug, thiserror::Error)]
pub enum ScriptEngineError {
    /// An error that can occur while loading a module or a directory.
    #[error("Failed to load: {0}")]
    Io(#[from] std::io::Error),

    /// An error that can occur while executing a script.
    #[error("Failed to execute script: {0}")]
    Runtime(#[from] rustyscript::error::Error),

    /// The script engine encountered an unexpected error.
    #[error("Script engine encountered an unexpected error: {0:?}")]
    Crash(Box<dyn std::any::Any + Send>),

    /// An error that can occur when trying to send a packet to the script
    /// engine without an open socket.
    #[error("Failed to send packet: Socket closed")]
    SocketClosed,
}

/// A container for the sockets between Bevy and the script engine.
#[derive()]
pub struct ScriptSockets {
    /// The thread handle for the script engine.
    thread: JoinHandle<Result<(), ScriptEngineError>>,

    /// The outgoing packets that can be sent to the script engine.
    outgoing: Sender<PacketOut>,

    /// The incoming packets that can be received from the script engine.
    incoming: Receiver<PacketIn>,
}

impl ScriptSockets {
    /// Creates a new `ScriptSockets` instance with the given thread handle.
    fn new(
        thread: JoinHandle<Result<(), ScriptEngineError>>,
        outgoing: Sender<PacketOut>,
        incoming: Receiver<PacketIn>,
    ) -> Self {
        Self {
            thread,
            outgoing,
            incoming,
        }
    }

    /// Joins the script engine thread, waiting for it to finish execution.
    pub fn join(self) -> Result<(), ScriptEngineError> {
        self.thread.join().map_err(ScriptEngineError::Crash)?
    }

    /// Sends a packet to the script engine.
    ///
    /// Returns an error if the packet cannot be sent.
    pub fn send(&self, packet: PacketOut) -> Result<(), ScriptEngineError> {
        self.outgoing
            .send_blocking(packet)
            .map_err(|_| ScriptEngineError::SocketClosed)
    }

    /// Sends a shutdown request to the script engine, if the socket is open.
    pub fn shutdown(&self) {
        let _ = self.send(PacketOut::Shutdown);
    }
}
