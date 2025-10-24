//! This module implements the dropdown widget.

use bevy::prelude::*;

mod components;
mod systems;
mod ui;

pub use components::*;

/// A plugin that adds support for dropdown widgets.
pub struct DropdownPlugin;
impl Plugin for DropdownPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_systems(Update, systems::update_menu_visibility)
            .add_observer(systems::on_spawn)
            .add_observer(systems::on_menu_click)
            .add_observer(systems::on_menu_entry_click);
    }
}
