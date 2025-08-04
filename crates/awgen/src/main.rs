#![doc = include_str!("../../../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use bevy::prelude::*;
use clap::Parser;

use crate::scripts::{PacketIn, PacketOut};

pub mod app;
pub mod scripts;

/// The arguments for the command line interface.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The project folder.
    #[arg(long, default_value = "project")]
    project: PathBuf,
}

/// Run the Awgen game engine.
fn main() -> AppExit {
    let args = Args::parse();

    let sockets = match scripts::start_script_engine(&args.project) {
        Ok(sockets) => sockets,
        Err(err) => {
            eprintln!("Failed to start script engine: {}", err);
            return AppExit::from_code(1);
        }
    };

    if let Err(err) = sockets.send(PacketOut::Init {
        project_folder: args.project.display().to_string(),
    }) {
        eprintln!(
            "Failed to send initialization packet to script engine: {}",
            err
        );
        return AppExit::from_code(1);
    }

    let Ok(PacketIn::Init { name, version }) = sockets.recv_blocking() else {
        eprintln!("Script Engine failed to properly initialize the game.");
        return AppExit::from_code(1);
    };

    let settings = app::GameInitSettings {
        name,
        version,
        debug: cfg!(debug_assertions),
        vsync: true,
        fullscreen: false,
    };

    app::run(settings, sockets)
}
