//! This module provides the builder function for the `Button` UI component.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::child_list::ChildList;
use crate::style::{ButtonStyle, FontStyle, WidgetLayout};

/// Builder for a button UI component.
#[derive(Debug, Default, Clone)]
pub struct ButtonBuilder {
    /// The layout of the button within its parent container.
    pub layout: WidgetLayout,

    /// The content of the button (icon, label, or both).
    pub content: ButtonContent,

    /// The style of the button.
    pub style: ButtonStyle,
}

impl ButtonBuilder {
    /// Sets the layout of the button.
    pub fn with_layout(mut self, layout: WidgetLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the text label of the button.
    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        let text = text.into();
        self.content = match self.content {
            ButtonContent::Icon(icon) => ButtonContent::Both(icon, text),
            _ => ButtonContent::Label(text),
        };
        self
    }

    /// Sets the icon of the button.
    pub fn with_icon(mut self, icon: Handle<Image>) -> Self {
        self.content = match self.content {
            ButtonContent::Label(label) => ButtonContent::Both(icon, label),
            _ => ButtonContent::Icon(icon),
        };
        self
    }

    /// Sets the style of the button.
    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }
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

/// Creates a button UI component using the provided builder.
pub fn button(builder: ButtonBuilder) -> impl Bundle {
    (
        builder.style.bundle(builder.layout.apply(Node::default())),
        Button,
        Hovered::default(),
        ChildList::from(|parent| match builder.content {
            ButtonContent::Icon(handle) => {
                parent.add_child(icon(handle, &builder.style));
            }
            ButtonContent::Label(string) => {
                parent.add_child(text(string, &builder.style.font));
            }
            ButtonContent::Both(handle, string) => {
                parent.add_child(icon(handle, &builder.style));
                parent.add_child(text(string, &builder.style.font));
            }
        }),
    )
}

/// Creates an icon node for the button.
fn icon(icon: Handle<Image>, style: &ButtonStyle) -> impl Bundle {
    (
        Node {
            width: Val::Px(style.icon_size),
            height: Val::Px(style.icon_size),
            ..default()
        },
        ImageNode::new(icon.clone()),
    )
}

/// Creates a text node for the button.
fn text(text: String, style: &FontStyle) -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(text),
            TextFont {
                font: style.font.clone(),
                font_size: style.font_size,
                ..default()
            },
            TextColor(style.font_color()),
        )],
    )
}
