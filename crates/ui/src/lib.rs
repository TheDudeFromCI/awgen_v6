//! This crate provides common UI widgets for Awgen.
//!
//! While designed for Awgen, these widgets can be used in any Bevy application.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;

pub mod color;
pub mod interaction;
pub mod menus;
pub mod scroll;
pub mod theme;
pub mod themes;
pub mod util;
pub mod widgets;

/// The path to the default Awgen UI font: "Quiver".
#[cfg(feature = "editor")]
pub const QUIVER_FONT: &str = "embedded://awgen_ui/fonts/quiver.ttf";

/// The path to the right arrow icon used in tree views.
#[cfg(feature = "editor")]
pub const RIGHT_ARROW_ICON: &str = "embedded://awgen_ui/icons/right_arrow.png";

/// The path to the down arrow icon used in tree views.
#[cfg(feature = "editor")]
pub const DOWN_ARROW_ICON: &str = "embedded://awgen_ui/icons/down_arrow.png";

/// The path to the vertical spacer icon used in tree views.
#[cfg(feature = "editor")]
pub const SPACER_ICON: &str = "embedded://awgen_ui/icons/vert_spacer.png";

/// The path to the folder icon used in tree views.
#[cfg(feature = "editor")]
pub const FOLDER_ICON: &str = "embedded://awgen_ui/icons/folder.png";

/// A prelude module for easy importing of common types.
pub mod prelude {
    pub use bevy::ui_widgets::{Activate, observe};

    pub use super::AwgenUiPlugin;
    pub use super::color::*;
    pub use super::interaction::*;
    pub use super::menus::overlay::*;
    pub use super::scroll::*;
    pub use super::theme::*;
    pub use super::util::*;
    pub use super::widgets::button::*;
    pub use super::widgets::grid_preview::*;
    pub use super::widgets::tree_view::*;
}

/// A plugin that adds support for common UI widgets.
pub struct AwgenUiPlugin;
impl Plugin for AwgenUiPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins((
            UiWidgetsPlugins,
            interaction::InteractionPlugin,
            menus::overlay::OverlayPlugin,
            scroll::ScrollPlugin,
            color::ColorPlugin,
        ))
        .add_observer(theme::style_container)
        .add_observer(theme::style_text)
        .add_observer(widgets::tree_view::on_tree_added)
        .add_observer(widgets::grid_preview::on_grid_add);

        #[cfg(feature = "editor")]
        {
            use bevy::asset::embedded_asset;

            embedded_asset!(app_, "crates/ui/src", "fonts/quiver.ttf");
            embedded_asset!(app_, "crates/ui/src", "icons/right_arrow.png");
            embedded_asset!(app_, "crates/ui/src", "icons/down_arrow.png");
            embedded_asset!(app_, "crates/ui/src", "icons/vert_spacer.png");
            embedded_asset!(app_, "crates/ui/src", "icons/folder.png");
        }
    }
}
