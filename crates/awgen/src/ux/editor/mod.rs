//! This module implements various UX widgets for the editor.

use bevy::prelude::*;

pub mod toolbar;

/// Plugin that sets up the editor UX.
pub struct EditorUXPlugin;
impl Plugin for EditorUXPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins(toolbar::EditorToolbarPlugin);
    }
}
