use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowFocused, WindowMode};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

/// The title of the window in the title bar.
pub const WINDOW_TITLE: &str = "Awgen Editor";

/// The name of the window, visible to the operating system.
pub const WINDOW_NAME: &str = "awgen_editor";

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

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
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
        .add_plugins(FramepacePlugin)
        .add_systems(Update, window_framerate)
        .run()
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
