//! This module prepares and launches the Bevy framework.

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy::winit::WinitSettings;

use crate::scripts::{ScriptEnginePlugin, ScriptSockets};

/// Settings for initializing the game.
#[derive(Debug)]
pub struct GameInitSettings {
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
        WindowMode::Fullscreen(MonitorSelection::Primary)
    } else {
        WindowMode::Windowed
    };

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WinitSettings::game())
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
                }),
        )
        .add_plugins(ScriptEnginePlugin::new(sockets))
        .add_systems(Startup, camera)
        .run()
}

/// Spaces a simple scene with a camera and a light.
fn camera(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform = Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((Camera3d::default(), camera_transform));
}
