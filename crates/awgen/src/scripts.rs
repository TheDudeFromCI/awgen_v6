//! The scripting plugin for the Awgen game engine.

use std::fs;
use std::path::PathBuf;

use bevy::prelude::*;
use rustyscript::{Module, ModuleHandle, Runtime, RuntimeOptions, Undefined, json_args};

use crate::awgen_ext;

/// This plugin adds support for the Javascript scripting language background
/// server.
pub struct ScriptsPlugin {
    /// The folder where the scripts are located.
    pub script_folder: PathBuf,
}

impl Plugin for ScriptsPlugin {
    fn build(&self, app_: &mut App) {
        app_.insert_resource(ScriptsSettings::new(self.script_folder.clone()))
            .add_systems(Startup, run);
    }
}

/// The settings for the Scripts plugin.
#[derive(Debug, Clone, Resource)]
pub struct ScriptsSettings {
    /// The folder where the scripts are located.
    script_folder: PathBuf,
}

impl ScriptsSettings {
    /// Creates a new instance of the settings.
    fn new(script_folder: PathBuf) -> Self {
        Self { script_folder }
    }

    /// Gets the folder where the scripts are located.
    pub fn script_folder(&self) -> &PathBuf {
        &self.script_folder
    }
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

    debug!("Loading modules from folder: {:?}", folder);

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
fn prepare_script_engine(folder: &PathBuf) -> Result<(Runtime, ModuleHandle), ScriptEngineError> {
    let index = Module::load(folder.join("index.ts"))?;

    let mut modules = vec![];
    find_modules(folder, &mut modules, &index)?;

    let mut runtime = Runtime::new(RuntimeOptions {
        default_entrypoint: Some("main".to_string()),
        extensions: vec![awgen_ext::awgen::init_ops_and_esm()],
        ..default()
    })?;

    let mod_ref = modules.iter().collect::<Vec<_>>();
    let mod_handle = runtime.load_modules(&index, mod_ref)?;
    runtime.set_current_dir(folder)?;

    Ok((runtime, mod_handle))
}

/// Loads and executes the script engine within the given script folder.
///
/// This function blocks the current thread until the script engine is finished
/// running.
fn run_script_engine(folder: &PathBuf) -> Result<(), ScriptEngineError> {
    let (mut runtime, mod_handle) = prepare_script_engine(folder)?;
    runtime.call_entrypoint::<Undefined>(&mod_handle, json_args!())?;
    Ok(())
}

/// Spawns a new thread to run the script engine.
fn run(mut app_exit: EventWriter<AppExit>, settings: Res<ScriptsSettings>) {
    let folder = settings.script_folder().clone();

    // TODO: Switch thread to async task so it can return errors
    let _thread = match std::thread::Builder::new()
        .name("script_engine".to_string())
        .spawn(move || {
            if let Err(e) = run_script_engine(&folder) {
                error!("Error running script engine: {:?}", e);
            }
        }) {
        Ok(thread) => thread,
        Err(e) => {
            error!("Failed to spawn script engine thread: {}", e);
            app_exit.send(AppExit::from_code(1));
            return;
        }
    };
}

/// An error that can occur while loading and executing scripts.
#[derive(Debug, thiserror::Error)]
enum ScriptEngineError {
    /// An error that can occur while loading a module or a directory.
    #[error("Failed to load modules: {0}")]
    Io(#[from] std::io::Error),

    /// An error that can occur while executing a script.
    #[error("Failed to execute script: {0}")]
    RuntimeError(#[from] rustyscript::error::Error),
}
