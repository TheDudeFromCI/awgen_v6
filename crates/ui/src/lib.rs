//! This crate provides common UI widgets for Awgen.
//!
//! While designed for Awgen, these widgets can be used in any Bevy application.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;

pub mod button;
pub mod color;
pub mod interaction;
pub mod overlay;
pub mod scroll;
pub mod theme;
pub mod tree;
pub mod util;

/// A prelude module for easy importing of common types.
pub mod prelude {
    pub use bevy::ui_widgets::{Activate, observe};

    pub use super::AwgenUiPlugin;
    pub use super::button::*;
    pub use super::color::*;
    pub use super::interaction::*;
    pub use super::overlay::*;
    pub use super::scroll::*;
    pub use super::theme::*;
    pub use super::tree::*;
    pub use super::util::*;
}

/// A plugin that adds support for common UI widgets.
pub struct AwgenUiPlugin;
impl Plugin for AwgenUiPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins((
            UiWidgetsPlugins,
            interaction::InteractionPlugin,
            overlay::OverlayPlugin,
            scroll::ScrollPlugin,
            color::ColorPlugin,
        ));
    }
}
