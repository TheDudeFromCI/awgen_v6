//! This module implements the ProjectPlugin, which handles the project
//! management for the editor.

use bevy::prelude::*;

/// This plugin provides the project management for the editor.
pub struct ProjectPlugin;
impl Plugin for ProjectPlugin {
    fn build(&self, app_: &mut App) {}
}

/// This resource stores metadata about the currently loaded project.
#[derive(Default, Resource)]
pub struct ProjectSettings {
    /// The path to the project file.
    pub path: String,
}
