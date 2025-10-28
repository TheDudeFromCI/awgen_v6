//! This module implements the style system for the UI framework.

use bevy::prelude::*;

mod data;

pub use data::*;

/// A plugin that sets up the style system.
pub struct StylePlugin;
impl Plugin for StylePlugin {
    fn build(&self, app_: &mut App) {}
}
