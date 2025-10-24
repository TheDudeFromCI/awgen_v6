//! This module handles user interface interactions for the Awgen game engine.

use bevy::prelude::*;

mod camera;
mod diagnostics;
mod editor;
mod filedrop;
mod overlay;
mod widgets;

pub use camera::CameraController;
pub use overlay::{Node3D, OverlayRoot};

/// The plugin that manages user interface interactions.
pub struct UxPlugin;
impl Plugin for UxPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins((
            diagnostics::DiagnosticsOverlayPlugin,
            camera::CameraPlugin,
            overlay::OverlayPlugin,
            widgets::WidgetsPlugin,
            editor::EditorUXPlugin,
        ))
        .add_systems(Update, filedrop::handle_file_drop);
    }
}
