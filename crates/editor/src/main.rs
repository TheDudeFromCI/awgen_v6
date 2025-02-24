#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowFocused, WindowMode};
use bevy::winit::WinitSettings;
use bevy_egui::EguiPlugin;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

mod file_picker;
mod project;
mod ui;

/// The title of the window in the title bar.
pub const WINDOW_TITLE: &str = "Awgen Editor";

/// The name of the window, visible to the operating system.
pub const WINDOW_NAME: &str = "Awgen Editor";

/// The window mode on startup.
pub const WINDOW_MODE: WindowMode = WindowMode::Windowed;

/// Whether or not to enable vsync.
pub const VSYNC: bool = true;

/// Whether or not to enable debug mode. If true, the editor will run in debug
/// mode, which enables additional features and logging.
pub const DEBUG: bool = true;

/// Whether or not to limit the framerate when the window is focused. If true,
/// the framerate will match the monitor refresh rate. If false, the framerate
/// will be unlimited.
pub const FRAME_LIMITER_FOCUSED: bool = true;

/// The framerate limit, in frames per second, when the window is unfocused. If
/// `None`, the framerate will be unlimited.
pub const FRAME_LIMITER_UNFOCUSED: Option<u32> = Some(5);

/// Run the editor.
fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WinitSettings::game())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!(
                            "{} {}{}",
                            WINDOW_TITLE,
                            env!("CARGO_PKG_VERSION"),
                            if DEBUG { " (Debug)" } else { "" }
                        ),
                        name: Some(WINDOW_NAME.to_string()),
                        mode: WINDOW_MODE,
                        present_mode: if VSYNC {
                            PresentMode::Fifo
                        } else {
                            PresentMode::Immediate
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: if DEBUG {
                        bevy::log::Level::DEBUG
                    } else {
                        bevy::log::Level::INFO
                    },
                    filter: "wgpu=error,naga=warn,calloop=debug,polling=debug".to_string(),
                    ..default()
                }),
        )
        .add_plugins((FramepacePlugin, EguiPlugin))
        .add_plugins((ui::EditorUiPlugin, project::ProjectPlugin))
        .add_systems(Startup, camera)
        .add_systems(Update, window_framerate)
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

/// A simple system that adjusts the framerate limit based on whether the window
/// is focused or unfocused.
fn window_framerate(
    mut settings: ResMut<FramepaceSettings>,
    mut window_focused: EventReader<WindowFocused>,
) {
    for ev in window_focused.read() {
        if ev.focused {
            if FRAME_LIMITER_FOCUSED {
                settings.limiter = Limiter::Auto;
                debug!("Window focused (Setting limiter to Auto).");
            } else {
                settings.limiter = Limiter::Off;
                debug!("Window focused (Setting limiter to Off).");
            }
        } else if let Some(framerate) = FRAME_LIMITER_UNFOCUSED {
            settings.limiter = Limiter::from_framerate(framerate as f64);
            debug!("Window unfocused (Setting limiter to {framerate} fps).");
        } else {
            settings.limiter = Limiter::Off;
            debug!("Window unfocused (Setting limiter to Off).");
        }
    }
}
