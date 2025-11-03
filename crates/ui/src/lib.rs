//! This crate provides common UI widgets for Awgen.
//!
//! While designed for Awgen, these widgets can be used in any Bevy application.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;

pub mod button;
pub mod child_list;
pub mod dropdown;
pub mod overlay;
pub mod style;
pub mod tree;

/// A prelude module for easy importing of common types.
pub mod prelude {
    pub use bevy::ui_widgets::{Activate, observe};

    pub use super::button::*;
    pub use super::child_list::*;
    pub use super::dropdown::*;
    pub use super::overlay::*;
    pub use super::style::*;
    pub use super::tree::*;
    pub use super::*;
}

/// A plugin that adds support for common UI widgets.
pub struct AwgenUiPlugin;
impl Plugin for AwgenUiPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins((
            UiWidgetsPlugins,
            overlay::OverlayPlugin,
            style::StylePlugin,
            dropdown::DropdownPlugin,
            tree::TreeViewPlugin,
        ))
        .add_observer(child_list::on_spawn);
    }
}
