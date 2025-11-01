//! This module implements a tree view widget for hierarchical data display.

use bevy::prelude::*;

mod components;
mod systems;
mod ui;

pub use components::*;

/// A plugin that adds support for tree view widgets.
pub struct TreeViewPlugin;
impl Plugin for TreeViewPlugin {
    fn build(&self, app_: &mut App) {}
}
