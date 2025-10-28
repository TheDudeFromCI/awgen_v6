//! This module implements the data structure for the Style component.

use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

/// A component representing the style of a UI element.
#[derive(Debug, Default, Component, Clone)]
pub struct Style {
    /// The style for dropdown menus.
    pub dropdown: DowndownMenuStyle,
}

/// The style properties for a dropdown menu widget.
#[derive(Debug, Clone)]
pub struct DowndownMenuStyle {
    /// The style for the primary container of the dropdown menu, i.e., the part
    /// that is always visible.
    pub button: ContainerStyle,

    /// The style for the option container of the dropdown menu. This is the
    /// part that appears when the menu is expanded.
    pub options: ContainerStyle,

    /// The style for the text in the dropdown menu.
    pub font_style: FontStyle,

    /// The size of icons in the dropdown menu.
    pub icon_size: f32,

    /// The spacing between elements in the dropdown menu.
    pub element_spacing: f32,
}

impl Default for DowndownMenuStyle {
    fn default() -> Self {
        Self {
            button: ContainerStyle::default(),
            options: ContainerStyle::default(),
            font_style: FontStyle::default(),
            icon_size: 32.0,
            element_spacing: 5.0,
        }
    }
}

/// The style for font rendering.
#[derive(Debug, Clone)]
pub struct FontStyle {
    /// The font handle.
    pub font: Handle<Font>,

    /// The font size.
    pub font_size: f32,

    /// The color style for the font.
    pub color: ColorStyle,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            font: Handle::default(),
            font_size: 16.0,
            color: ColorStyle::all(Color::WHITE),
        }
    }
}

impl FontStyle {
    /// Returns the default font color.
    pub fn font_color(&self) -> Color {
        self.color.default
    }
}

/// The style for container elements.
#[derive(Debug, Clone)]
pub struct ContainerStyle {
    /// The background color style.
    pub background: ColorStyle,

    /// The border color style.
    pub border: BorderStyle,

    /// The padding inside the container.
    pub padding: f32,
}

impl Default for ContainerStyle {
    fn default() -> Self {
        Self {
            background: ColorStyle::all(WHITE.into()),
            border: Default::default(),
            padding: 2.0,
        }
    }
}

impl ContainerStyle {
    /// Returns the default background color as a [`BackgroundColor`].
    pub fn background_color(&self) -> BackgroundColor {
        BackgroundColor(self.background.default)
    }

    /// Returns the border thickness as a [`UiRect`].
    pub fn border_thickness(&self) -> UiRect {
        self.border.border_thickness()
    }

    /// Returns the border color as a [`BorderColor`].
    pub fn border_color(&self) -> BorderColor {
        self.border.border_color()
    }

    /// Returns the border radius as a [`BorderRadius`].
    pub fn border_radius(&self) -> BorderRadius {
        self.border.border_radius()
    }

    /// Returns the padding as a [`UiRect`].
    pub fn padding(&self) -> UiRect {
        UiRect::all(Val::Px(self.padding))
    }
}

/// The style for borders.
#[derive(Debug, Clone)]
pub struct BorderStyle {
    /// The color style of the border.
    pub color: ColorStyle,

    /// The thickness of the border on each side.
    pub thickness: f32,

    /// The radius of the border on each corner.
    pub radius: f32,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            color: ColorStyle::all(WHITE.into()),
            thickness: 0.0,
            radius: 0.0,
        }
    }
}

impl BorderStyle {
    /// Returns the border thickness as a [`UiRect`].
    pub fn border_thickness(&self) -> UiRect {
        UiRect::all(Val::Px(self.thickness))
    }

    /// Returns the border color as a [`BorderColor`].
    pub fn border_color(&self) -> BorderColor {
        BorderColor::all(self.color.default)
    }

    /// Returns the border radius as a [`BorderRadius`].
    pub fn border_radius(&self) -> BorderRadius {
        BorderRadius::all(Val::Px(self.radius))
    }
}

/// Color styles for different UI states.
#[derive(Debug, Clone)]
pub struct ColorStyle {
    /// The default color.
    pub default: Color,

    /// The color when hovered.
    pub hovered: Color,

    /// The color when pressed.
    pub pressed: Color,
}

impl ColorStyle {
    /// Creates a new `ColorStyle` with the same color for all states.
    pub const fn all(color: Color) -> Self {
        Self {
            default: color,
            hovered: color,
            pressed: color,
        }
    }
}
