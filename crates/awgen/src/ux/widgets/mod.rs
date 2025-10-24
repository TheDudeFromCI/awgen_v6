//! This module implements common UI widgets.

use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;

pub mod dropdown;

/// A plugin that adds support for common UI widgets.
pub struct WidgetsPlugin;
impl Plugin for WidgetsPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins((UiWidgetsPlugins, dropdown::DropdownPlugin));
    }
}
