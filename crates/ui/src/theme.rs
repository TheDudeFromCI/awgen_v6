//! This module implements the theme data structures for the UI components.

use bevy::prelude::*;

/// The theme for the UI components.
#[derive(Debug, Clone)]
pub struct UiTheme {
    /// The theme for the outer window container.
    pub outer_window: ContainerTheme,

    /// The theme for the inner window container.
    pub inner_window: ContainerTheme,

    /// The theme for buttons.
    pub button: InteractiveTheme,

    /// The theme for asset display containers.
    pub asset: InteractiveTheme,

    /// The color theme for icons used in the UI.
    pub icon_color: ColorTheme,

    /// The size of icons used in the UI.
    pub icon_size: f32,

    /// The theme for font rendering.
    pub text: FontTheme,
}

/// Theme for a generic container.
#[derive(Debug, Clone)]
pub struct ContainerTheme {
    /// The background color of the container.
    pub background_color: Color,

    /// The border color of the container.
    pub border_color: Color,

    /// The border thickness of the container.
    pub border_thickness: f32,

    /// The border radius of the container.
    pub border_radius: f32,

    /// The padding inside the container.
    pub padding: f32,
}

/// Theme for a generic container.
#[derive(Debug, Clone)]
pub struct InteractiveTheme {
    /// The background color of the container.
    pub background_color: ColorTheme,

    /// The border color of the container.
    pub border_color: ColorTheme,

    /// The border thickness of the container.
    pub border_thickness: f32,

    /// The border radius of the container.
    pub border_radius: f32,

    /// The padding inside the container.
    pub padding: f32,
}

/// The theme for font rendering.
#[derive(Debug, Clone)]
pub struct FontTheme {
    /// The font handle.
    pub font: Handle<Font>,

    /// The font size.
    pub font_size: f32,

    /// The default color of the font.
    pub color: ColorTheme,
}

/// Theme for different colors based on interaction state.
#[derive(Debug, Clone)]
pub struct ColorTheme {
    /// The default color.
    pub default: Color,

    /// The color when hovered.
    pub hovered: Color,

    /// The color when pressed.
    pub pressed: Color,

    /// The color when disabled.
    pub disable: Color,
}
