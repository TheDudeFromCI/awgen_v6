//! This crate provides common UI widgets for Awgen.
//!
//! While designed for Awgen, these widgets can be used in any Bevy application.

use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;

pub mod dropdown;

/// A plugin that adds support for common UI widgets.
pub struct AwgenUiPlugin;
impl Plugin for AwgenUiPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins((UiWidgetsPlugins, dropdown::DropdownPlugin));
    }
}
