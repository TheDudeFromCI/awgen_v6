//! This module handles user interface interactions for the Awgen game engine.

use awgen_ui::AwgenUiPlugin;
use bevy::prelude::*;

mod camera;
mod diagnostics;
mod editor;
mod filedrop;
mod overlay;

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
            AwgenUiPlugin,
            editor::EditorUXPlugin,
        ))
        .add_systems(Update, filedrop::handle_file_drop);
    }
}
