//! This module implements the button UI component.

use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;

/// A plugin that adds button widget.
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_observer(systems::on_spawn);
    }
}
