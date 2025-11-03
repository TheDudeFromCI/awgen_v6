//! This module implements the data structure for the Style component.

use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

use crate::overlay::ScreenAnchor;

/// A component representing the style of a UI element.
#[derive(Debug, Default, Component, Clone)]
#[require(Node)]
pub struct Style {
    /// The style for button widgets.
    pub button: ButtonStyle,

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

    /// The spacing between elements in the dropdown menu.
    pub element_spacing: f32,
}

impl Default for DowndownMenuStyle {
    fn default() -> Self {
        Self {
            button: ContainerStyle::default(),
            options: ContainerStyle::default(),
            font_style: FontStyle::default(),
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
    /// Returns a bundled node with the container style applied.
    pub fn bundle(&self, node: Node) -> impl Bundle {
        (
            Node {
                border: self.border_thickness(),
                padding: self.padding(),
                ..node
            },
            self.background_color(),
            self.border_color(),
            self.border_radius(),
        )
    }

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

/// The style properties for a button widget.
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    /// Container style for the button.
    pub container: ContainerStyle,

    /// Font style for the button text.
    pub font: FontStyle,

    /// Content alignment within the button.
    pub alignment: ButtonAlignment,

    /// Position of the icon relative to the text.
    pub icon_position: IconPosition,

    /// The size of icon in the button.
    pub icon_size: f32,

    /// The spacing between content elements (icon and text).
    pub content_spacing: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            container: Default::default(),
            font: Default::default(),
            alignment: Default::default(),
            icon_position: Default::default(),
            icon_size: 32.0,
            content_spacing: 5.0,
        }
    }
}

impl ButtonStyle {
    /// Returns a bundled node with the button style applied.
    pub fn bundle(&self, node: Node) -> impl Bundle {
        self.container.bundle(Node {
            flex_direction: self.flex_direction(),
            row_gap: self.content_spacing(),
            align_items: self.align_items(),
            ..node
        })
    }

    /// Returns the alignment for the button content.
    pub fn align_items(&self) -> AlignItems {
        match (self.alignment, self.icon_position) {
            (ButtonAlignment::Left, IconPosition::Left) => AlignItems::FlexStart,
            (ButtonAlignment::Left, IconPosition::Right) => AlignItems::FlexEnd,
            (ButtonAlignment::Center, _) => AlignItems::Center,
            (ButtonAlignment::Right, IconPosition::Left) => AlignItems::FlexEnd,
            (ButtonAlignment::Right, IconPosition::Right) => AlignItems::FlexStart,
        }
    }

    /// Returns the flex direction based on the icon position.
    pub fn flex_direction(&self) -> FlexDirection {
        match self.icon_position {
            IconPosition::Left => FlexDirection::Row,
            IconPosition::Right => FlexDirection::RowReverse,
        }
    }

    /// Returns the content spacing as a [`Val`].
    pub fn content_spacing(&self) -> Val {
        Val::Px(self.content_spacing)
    }
}

/// Text alignment options for button text.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonAlignment {
    /// Align content to the left.
    Left,

    /// Align content to the center.
    #[default]
    Center,

    /// Align content to the right.
    Right,
}

/// Icon position options for button widgets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconPosition {
    /// The icon on the left side of the button text, if both are present.
    #[default]
    Left,

    /// The icon on the right side of the button text, if both are present.
    Right,
}

/// Layout options for UI widgets.
#[derive(Debug, Default, Clone)]
pub enum WidgetLayout {
    /// Automatic layout based on content size and parent constraints.
    #[default]
    Auto,

    /// A fixed layout with an anchored position, but a dynamic size.
    Anchored {
        /// The anchor position.
        position: ScreenAnchor,
    },

    /// Fixed layout with absolute position and size.
    Absolute {
        /// The anchor position.
        position: ScreenAnchor,

        /// The size of the widget.
        size: Val2,
    },

    /// A fixed widget size, with a relative position within its parent.
    Relative {
        /// The size of the widget.
        size: Val2,
    },
}

impl WidgetLayout {
    /// Applies the layout to the given node and returns the modified node.
    pub fn apply(&self, mut node: Node) -> Node {
        match self {
            WidgetLayout::Auto => {
                node.position_type = PositionType::Relative;
                node.width = Val::Auto;
                node.height = Val::Auto;
            }
            WidgetLayout::Anchored { position } => {
                node.width = Val::Auto;
                node.height = Val::Auto;
                position.set_node(&mut node);
            }
            WidgetLayout::Absolute { position, size } => {
                node.width = size.x;
                node.height = size.y;
                position.set_node(&mut node);
            }
            WidgetLayout::Relative { size } => {
                node.position_type = PositionType::Relative;
                node.width = size.x;
                node.height = size.y;
            }
        }

        node
    }
}
