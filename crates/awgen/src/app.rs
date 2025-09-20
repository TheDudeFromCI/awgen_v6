//! This module prepares and launches the Bevy framework.

use std::path::{Path, PathBuf};

use bevy::asset::io::AssetSourceBuilder;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy::winit::WinitSettings;

use crate::camera::CameraPlugin;
use crate::map::{BlockModel, ChunkModelRoot, MapPlugin, QuadFace, VoxelChunk, WorldPos};
use crate::scripts::{ScriptEnginePlugin, ScriptSockets};
use crate::tiles::{TileRot, TilesetMaterial, TilesetPlugin};
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
                    filter: "wgpu=error,naga=warn,calloop=debug,polling=debug".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            bevy_framepace::FramepacePlugin,
            ScriptEnginePlugin::new(sockets),
            CameraPlugin,
            TilesetPlugin,
            MapPlugin,
            UxPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .run()
}

/// Spaces a simple scene with a cube and a light.
fn setup_scene(
    asset_server: Res<AssetServer>,
    mut tileset_materials: ResMut<Assets<TilesetMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let pos = WorldPos::new(0, 0, 0);
    let mut chunk = VoxelChunk::new(pos.as_chunk_pos());
    let mut block = chunk.get_block_mut(pos.as_block_pos());
    let block_model = block.model_mut();
    *block_model = BlockModel::Cube {
        up: Some(QuadFace {
            tile_index: 2,
            tile_rot: TileRot::default(),
        }),
        north: Some(QuadFace {
            tile_index: 3,
            tile_rot: TileRot::default().into_rotated(90.0),
        }),
        south: Some(QuadFace {
            tile_index: 4,
            tile_rot: TileRot::default().into_rotated(270.0),
        }),
        east: Some(QuadFace {
            tile_index: 5,
            tile_rot: TileRot::default(),
        }),
        west: Some(QuadFace {
            tile_index: 6,
            tile_rot: TileRot::default().into_rotated(180.0),
        }),
    };

    let tileset = asset_server.load("game://tilesets/terrain.tiles");
    let tileset_mat = TilesetMaterial {
        texture: tileset,
        alpha_mode: AlphaMode::Opaque,
    };

    commands.spawn((
        chunk,
        ChunkModelRoot {
            opaque_material: Some(tileset_materials.add(tileset_mat)),
            ..default()
        },
    ));
}
