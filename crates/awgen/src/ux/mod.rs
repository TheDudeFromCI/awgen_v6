//! This module handles user interface interactions for the Awgen game engine.

use bevy::prelude::*;

mod filedrop;

/// The plugin that manages user interface interactions.
pub struct UxPlugin;
impl Plugin for UxPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_systems(Update, filedrop::handle_file_drop);
    }
}
