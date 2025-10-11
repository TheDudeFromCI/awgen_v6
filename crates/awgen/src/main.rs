#![doc = include_str!("../../../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Arc;

use bevy::prelude::*;
use clap::Parser;

use crate::database::Database;
use crate::scripts::PacketIn;

mod app;
mod database;
mod map;
mod scripts;
mod tiles;
mod ux;

/// The arguments for the command line interface.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The project folder.
    #[arg(long, default_value = "project")]
    project: PathBuf,

    /// Whether to run the game in editor mode.
    #[arg(long, default_value_t = false)]
    editor: bool,
}

/// Run the Awgen game engine.
fn main() -> AppExit {
    let args = Args::parse();

    let db = Arc::new(Database::new(&args.project).unwrap_or_else(|err| {
        eprintln!("Failed to open database: {}", err);
        std::process::exit(1);
    }));

    let script_path = if args.editor {
        args.project.join("editor/scripts")
    } else {
        args.project.join("scripts")
    };

    let mut sockets = match scripts::start_script_engine(script_path, db) {
        Ok(sockets) => sockets,
        Err(err) => {
            eprintln!("Failed to start script engine: {}", err);
            return AppExit::from_code(1);
        }
    };

    let init_packet = match sockets.recv_blocking() {
        Ok(packet) => packet,
        Err(err) => {
            eprintln!(
                "Failed to receive initialization packet from script engine: {}",
                err
            );
            if let Err(err2) = sockets.shutdown_blocking() {
                eprintln!("The script engine has crashed: {}", err2);
            }
            return AppExit::from_code(1);
        }
    };

    let PacketIn::Init { name, version } = init_packet else {
        eprintln!("Script Engine failed to properly initialize the game.");
        if let Err(err2) = sockets.shutdown_blocking() {
            eprintln!("The script engine has crashed: {}", err2);
        }
        return AppExit::from_code(1);
    };

    let settings = app::GameInitSettings {
        project_folder: args.project.to_string_lossy().to_string(),
        name,
        version,
        debug: cfg!(debug_assertions),
        vsync: true,
        fullscreen: false,
    };

    app::run(settings, sockets)
}
