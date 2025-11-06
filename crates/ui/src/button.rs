//! This module provides the builder function for the `Button` UI component.

use bevy::ecs::relationship::RelatedSpawner;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::color::{InsetBorder, InteractiveColor};
use crate::prelude::InteractionSender;
use crate::theme::UiTheme;

/// Builder for a button UI component.
#[derive(Debug, Clone)]
pub struct ButtonBuilder {
    /// The default node component, if a custom layout is needed. Some fields
    /// may be overridden.
    pub node: Node,

    /// The content of the button (icon, label, or both).
    pub content: ButtonContent,

    /// The theme for the button.
    pub theme: UiTheme,
}

/// The content of the button.
#[derive(Debug, Clone)]
pub enum ButtonContent {
    /// An icon-only button.
    Icon(Handle<Image>),

    /// A label-only button.
    Label(String),

    /// A button with both an icon and a label.
    Both(Handle<Image>, String),
}

impl Default for ButtonContent {
    fn default() -> Self {
        ButtonContent::Label("Button".to_string())
    }
}

impl ButtonContent {
    /// Creates a label-only button content.
    ///
    /// This is a shortcut for creating a [`ButtonContent::Label`] with a static
    /// string.
    pub fn text<S: Into<String>>(text: S) -> Self {
        ButtonContent::Label(text.into())
    }
}

/// Creates a button UI component using the provided builder.
pub fn button(builder: ButtonBuilder) -> impl Bundle {
    (
        Button,
        Node {
            border: UiRect::all(px(builder.theme.button.border_thickness)),
            padding: UiRect::all(px(builder.theme.button.padding)),
            ..builder.node
        },
        BorderRadius::all(px(builder.theme.button.border_radius)),
        InteractiveColor::<BackgroundColor>::from(&builder.theme.button.background_color),
        InsetBorder::default(),
        InteractiveColor::<BorderColor>::from(&builder.theme.button.border_color),
        InteractionSender,
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            match builder.content {
                ButtonContent::Icon(handle) => {
                    parent.spawn(icon(handle, &builder.theme));
                }
                ButtonContent::Label(string) => {
                    parent.spawn(text(string, &builder.theme));
                }
                ButtonContent::Both(handle, string) => {
                    parent.spawn(icon(handle, &builder.theme));
                    parent.spawn(text(string, &builder.theme));
                }
            };
        })),
    )
}

/// Creates an icon node for the button.
fn icon(icon: Handle<Image>, theme: &UiTheme) -> impl Bundle {
    (
        Node {
            width: px(theme.icon_size),
            height: px(theme.icon_size),
            ..default()
        },
        ImageNode::new(icon.clone()),
        InteractiveColor::<ImageNode>::from(&theme.icon_color),
    )
}

/// Creates a text node for the button.
fn text(text: String, theme: &UiTheme) -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(text),
            TextFont {
                font: theme.text.font.clone(),
                font_size: theme.text.font_size,
                ..default()
            },
            InteractiveColor::<TextColor>::from(&theme.text.color),
        )],
    )
}
