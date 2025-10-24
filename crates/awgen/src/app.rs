//! This module prepares and launches the Bevy framework.

use std::path::{Path, PathBuf};

use bevy::asset::io::AssetSourceBuilder;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy::winit::WinitSettings;

use crate::map::MapPlugin;
use crate::scripts::{ScriptEnginePlugin, ScriptSockets};
use crate::tiles::TilesetPlugin;
use crate::ux::UxPlugin;

/// Settings for initializing the game.
#[derive(Debug)]
pub struct GameInitSettings {
    /// The project folder where the game assets are located.
    pub project_folder: String,

    /// The name of the game.
    pub name: String,

    /// The version of the game.
    pub version: String,

    /// Whether or not to launch the game in debug mode.
    pub debug: bool,

    /// Whether or not to enable vsync.
    pub vsync: bool,

    /// Whether or not to launch the game in fullscreen mode.
    pub fullscreen: bool,

    /// Whether or not to launch the game in editor mode.
    pub editor: bool,
}

#[derive(Debug, Resource)]
pub struct ProjectSettings {
    /// The project folder.
    project_folder: PathBuf,
}

impl ProjectSettings {
    /// Gets the project folder path.
    pub fn project_folder(&self) -> &Path {
        self.project_folder.as_path()
    }
}

/// The current state of the Awgen application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum AwgenState {
    /// The application is initializing.
    ///
    /// Contains a boolean indicating whether initialization is into editor mode
    /// or not.
    Init(bool),

    /// The application is running the game.
    Game,

    /// The application is in editor mode.
    Editor,
}

/// Launch a new game window with the Bevy framework, setting up the
/// necessary plugins and resources.
pub fn run(settings: GameInitSettings, sockets: ScriptSockets) -> AppExit {
    let window_title = format!(
        "{} - {}{}",
        settings.name,
        settings.version,
        if settings.debug { " (Debug)" } else { "" }
    );

    let window_name = Some(settings.name.clone());

    let present_mode = if settings.vsync {
        PresentMode::Fifo
    } else {
        PresentMode::Immediate
    };

    let debug_level = if settings.debug {
        bevy::log::Level::DEBUG
    } else {
        bevy::log::Level::INFO
    };

    let window_mode = if settings.fullscreen {
        WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
    } else {
        WindowMode::Windowed
    };

    let project_settings = ProjectSettings {
        project_folder: PathBuf::from(settings.project_folder.clone()),
    };

    let game_assets = format!("{}/assets", settings.project_folder);
    let editor_assets = format!("{}/editor/assets", settings.project_folder,);

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WinitSettings::game())
        .insert_resource(project_settings)
        .register_asset_source(
            "game",
            AssetSourceBuilder::platform_default(&game_assets, None),
        )
        .register_asset_source(
            "editor",
            AssetSourceBuilder::platform_default(&editor_assets, None),
        )
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: window_title,
                        name: window_name,
                        mode: window_mode,
                        present_mode,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: debug_level,
                    filter: "wgpu=error,naga=warn,calloop=debug,polling=debug,cosmic_text=info"
                        .to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_state(AwgenState::Init(settings.editor))
        .add_plugins((
            ScriptEnginePlugin::new(sockets),
            TilesetPlugin,
            MapPlugin,
            UxPlugin,
        ))
        .add_systems(Last, finish_init)
        .run()
}

/// Finishes initialization and transitions to the next state.
fn finish_init(state: Res<State<AwgenState>>, mut next_state: ResMut<NextState<AwgenState>>) {
    match **state {
        AwgenState::Init(false) => next_state.set(AwgenState::Game),
        AwgenState::Init(true) => next_state.set(AwgenState::Editor),
        AwgenState::Game => {}
        AwgenState::Editor => {}
    }
}
