//! This module implements the component for the button widget.

use bevy::prelude::*;

use crate::style::Style;

/// A component for button widgets.
#[derive(Debug, Default, Component)]
#[require(Node, Interaction, Style)]
pub struct AwgenButton {
    /// Optional icon for the button.
    icon: Option<Handle<Image>>,

    /// Optional text for the button.
    text: Option<String>,
}

impl AwgenButton {
    /// Creates a new empty [`AwgenButton`] with an icon only.
    pub fn with_icon(icon: Handle<Image>) -> Self {
        Self {
            icon: Some(icon),
            text: None,
        }
    }

    /// Creates a new empty [`AwgenButton`] with text only.
    pub fn with_text<S: Into<String>>(text: S) -> Self {
        Self {
            icon: None,
            text: Some(text.into()),
        }
    }

    /// Creates a new empty [`AwgenButton`] with both an icon and text.
    pub fn with_icon_and_text<S: Into<String>>(icon: Handle<Image>, text: S) -> Self {
        Self {
            icon: Some(icon),
            text: Some(text.into()),
        }
    }

    /// Returns a reference to the icon handle, if it exists.
    pub fn icon(&self) -> Option<&Handle<Image>> {
        self.icon.as_ref()
    }

    /// Returns a reference to the button text, if it exists.
    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}
