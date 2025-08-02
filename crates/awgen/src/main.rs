#![doc = include_str!("../../../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use bevy::prelude::*;
use clap::Parser;

use crate::scripts::PacketOut;

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

    for i in 0 .. 3 {
        sockets.send(PacketOut::Count { value: i }).unwrap();
    }

    // let app_out = app::run();
    let app_out = AppExit::Success;

    sockets.shutdown();
    if let Err(err) = sockets.join() {
        eprintln!("Script engine thread panicked: {}", err);
        return AppExit::from_code(1);
    }

    app_out
}
