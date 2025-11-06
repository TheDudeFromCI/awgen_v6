//! This module implements the ChildList component for dynamic child list
//! bundles.

use bevy::prelude::*;

/// Content alignment options for a container.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentAlignment {
    /// Align content to the left.
    Left,

    /// Align content to the center.
    #[default]
    Center,

    /// Align content to the right.
    Right,
}

impl From<ContentAlignment> for AlignItems {
    fn from(alignment: ContentAlignment) -> Self {
        match alignment {
            ContentAlignment::Left => AlignItems::FlexStart,
            ContentAlignment::Center => AlignItems::Center,
            ContentAlignment::Right => AlignItems::FlexEnd,
        }
    }
}

/// Direction options for arranging content within a container.
#[derive(Debug, Clone)]
pub enum ContentDirection {
    /// Arrange content horizontally, with the specified spacing between items.
    Horizontal(f32),

    /// Arrange content vertically, with the specified spacing between items.
    Vertical(f32),
}

impl Default for ContentDirection {
    fn default() -> Self {
        ContentDirection::Horizontal(5.0)
    }
}

impl ContentDirection {
    /// Returns the flex direction for the container.
    pub fn flex_direction(&self) -> FlexDirection {
        match self {
            ContentDirection::Horizontal(_) => FlexDirection::Row,
            ContentDirection::Vertical(_) => FlexDirection::Column,
        }
    }

    /// Returns the content spacing as a [`Val`] for vertical arrangement.
    pub fn row_gap(&self) -> Val {
        match self {
            ContentDirection::Horizontal(_) => Val::Px(0.0),
            ContentDirection::Vertical(spacing) => Val::Px(*spacing),
        }
    }

    /// Returns the content spacing as a [`Val`] for horizontal arrangement.
    pub fn column_gap(&self) -> Val {
        match self {
            ContentDirection::Horizontal(spacing) => Val::Px(*spacing),
            ContentDirection::Vertical(_) => Val::Px(0.0),
        }
    }

    /// Converts the direction to horizontal, preserving spacing.
    pub fn as_horizontal(&self) -> ContentDirection {
        match self {
            ContentDirection::Horizontal(spacing) => ContentDirection::Horizontal(*spacing),
            ContentDirection::Vertical(spacing) => ContentDirection::Horizontal(*spacing),
        }
    }

    /// Converts the direction to vertical, preserving spacing.
    pub fn as_vertical(&self) -> ContentDirection {
        match self {
            ContentDirection::Horizontal(spacing) => ContentDirection::Vertical(*spacing),
            ContentDirection::Vertical(spacing) => ContentDirection::Vertical(*spacing),
        }
    }
}
