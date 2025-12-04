//! This module implements the theme data structures for the UI components.

use std::sync::Arc;

use bevy::prelude::*;

use crate::color::InteractiveColor;

/// The theme for the UI components.
///
/// This is a wrapper around the global theme to allow for easy cloning and
/// passing around.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct UiTheme(pub Arc<GlobalTheme>);

impl From<GlobalTheme> for UiTheme {
    fn from(theme: GlobalTheme) -> Self {
        UiTheme(Arc::new(theme))
    }
}

/// The global theme for all UI components.
#[derive(Debug, Clone)]
pub struct GlobalTheme {
    /// The theme for the outer window container.
    pub outer_window: ContainerTheme,

    /// The theme for the inner window container.
    pub inner_window: ContainerTheme,

    /// The theme for buttons.
    pub button: ButtonTheme,

    /// The theme for tree views.
    pub tree_view: TreeViewTheme,

    /// The theme for grid previews.
    pub grid_preview: GridPreviewTheme,
}

/// Theme for a generic container.
#[derive(Debug, Clone, Component)]
#[require(Node, BackgroundColor, BorderColor, BorderRadius)]
pub struct ContainerTheme {
    /// The background color of the container.
    pub background_color: ColorTheme,

    /// The border color of the container.
    pub border_color: ColorTheme,

    /// The border thickness of the container.
    pub border_thickness: f32,

    /// The border radius of the container.
    pub border_radius: f32,

    /// The padding inside the container.
    pub padding: UiRect,

    /// The theme for font rendering within the container.
    pub text: FontTheme,

    /// The size of icons used in the container.
    pub icon_size: f32,

    /// The color theme for icons used in the container.
    pub icon_color: ColorTheme,
}

/// The theme for font rendering.
#[derive(Debug, Clone, Component)]
#[require(Node, TextFont, TextColor)]
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
pub enum ColorTheme {
    /// The color changes based on interaction state.
    Interactive {
        /// The default color.
        default: Color,

        /// The color when hovered.
        hovered: Color,

        /// The color when pressed.
        pressed: Color,

        /// The color when disabled.
        disable: Color,

        /// The color when checked/selected, but not hovered or pressed.
        checked: Color,
    },

    /// A fixed color that does not change with interaction.
    Fixed(Color),
}

impl From<Color> for ColorTheme {
    fn from(color: Color) -> Self {
        ColorTheme::Fixed(color)
    }
}

/// Theme for the button widget.
#[derive(Debug, Clone)]
pub struct ButtonTheme {
    /// The theme for the button node.
    pub container: ContainerTheme,
}

/// Theme for the tree view.
#[derive(Debug, Clone)]
pub struct TreeViewTheme {
    /// The theme for the container of the tree view.
    pub container: ContainerTheme,

    /// The theme for the label of each tree node.
    pub label: ContainerTheme,

    /// The icon for a collapsed node.
    pub right_arrow_icon: Handle<Image>,

    /// The icon for an expanded node.
    pub down_arrow_icon: Handle<Image>,

    /// The icon for a spacer before a label.
    pub spacer_icon: Handle<Image>,
}

/// Theme for the grid preview widget.
#[derive(Debug, Clone)]
pub struct GridPreviewTheme {
    /// The theme for the container of the grid preview.
    pub container: ContainerTheme,

    /// The size of each cell in the grid.
    pub cell_size: Vec2,

    /// The spacing between each cell in the grid.
    pub cell_spacing: Vec2,

    /// The theme for each cell in the grid.
    pub cell: ContainerTheme,
}

pub(crate) fn style_container(
    trigger: On<Add, ContainerTheme>,
    mut query: Query<(
        &mut Node,
        &mut BackgroundColor,
        &mut BorderRadius,
        &mut BorderColor,
        &ContainerTheme,
    )>,
    mut commands: Commands,
) {
    let Ok((mut node, mut bg_color, mut border_radius, mut border_color, theme)) =
        query.get_mut(trigger.entity)
    else {
        warn!("UiTheme component missing on entity added trigger");
        return;
    };

    node.border = UiRect::all(px(theme.border_thickness));
    node.padding = theme.padding;
    *border_radius = BorderRadius::all(px(theme.border_radius));

    match theme.background_color {
        ColorTheme::Fixed(color) => {
            *bg_color = BackgroundColor(color);
        }
        ColorTheme::Interactive { .. } => {
            commands
                .entity(trigger.entity)
                .insert(InteractiveColor::<BackgroundColor>::from(
                    &theme.background_color,
                ));
        }
    }

    match theme.border_color {
        ColorTheme::Fixed(color) => {
            *border_color = BorderColor::all(color);
        }
        ColorTheme::Interactive { .. } => {
            commands
                .entity(trigger.entity)
                .insert(InteractiveColor::<BorderColor>::from(&theme.border_color));
        }
    }
}

/// Styles a text component when its font theme is added.
pub(crate) fn style_text(
    trigger: On<Add, FontTheme>,
    mut query: Query<(&mut TextFont, &mut TextColor, &FontTheme)>,
    mut commands: Commands,
) {
    let Ok((mut text_font, mut text_color, theme)) = query.get_mut(trigger.entity) else {
        warn!("FontTheme component missing on entity added trigger");
        return;
    };

    text_font.font = theme.font.clone();
    text_font.font_size = theme.font_size;

    match &theme.color {
        ColorTheme::Fixed(color) => {
            *text_color = TextColor(*color);
        }
        ColorTheme::Interactive { .. } => {
            commands
                .entity(trigger.entity)
                .insert(InteractiveColor::<TextColor>::from(&theme.color));
        }
    }
}
