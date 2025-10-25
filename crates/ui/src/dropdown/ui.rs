//! This module contains builder functions for the UI hierarchy of the dropdown
//! widget.

use bevy::ecs::relationship::{RelatedSpawnerCommands, Relationship};
use bevy::prelude::*;

use crate::dropdown::{
    DropdownEntryButton,
    DropdownMenu,
    DropdownMenuButton,
    DropdownMenuEntry,
    DropdownMenuNodes,
};

/// The size of icons in the dropdown menu.
const ICON_SIZE: f32 = 32.0;

/// The font used for dropdown text.
const TEXT_FONT: &str = "editor://fonts/pixel_arial.ttf";

/// The size of dropdown text.
const TEXT_SIZE: f32 = 16.0;

/// The color of dropdown text.
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// The background color of dropdown buttons.
const BACKGROUND_COLOR: Color = Color::srgba(0.3, 0.225, 0.225, 1.0);

/// The border color of dropdown buttons.
const BORDER_COLOR: Color = Color::srgba(0.7, 0.32, 0.39, 1.0);

/// The background color of dropdown buttons with an alpha of 0 (hidden).
const BACKGROUND_COLOR_HIDDEN: Color = Color::srgba(0.6, 0.45, 0.45, 0.0);

/// The border color of dropdown buttons with an alpha of 0 (hidden).
const BORDER_COLOR_HIDDEN: Color = Color::srgba(0.94, 0.42, 0.49, 0.0);

/// The number of pixels between menu entries.
const MENU_ENTRY_SPACING: f32 = 5.0;

/// Builds the UI hierarchy for a dropdown menu.
pub(super) fn build_menu(
    asset_server: &Res<AssetServer>,
    menu_id: Entity,
    menu: &DropdownMenu,
    commands: &mut Commands,
) {
    let mut menu_nodes = DropdownMenuNodes {
        content_node: Entity::PLACEHOLDER,
    };

    commands.entity(menu_id).with_children(|parent| {
        menu_button(menu_id, asset_server, menu.main_button(), parent);

        menu_nodes.content_node = parent
            .spawn(Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(MENU_ENTRY_SPACING),
                padding: UiRect::top(Val::Px(MENU_ENTRY_SPACING)),
                top: Val::Px(ICON_SIZE),
                display: Display::None,
                ..default()
            })
            .with_children(|parent| {
                for entry in menu.entries() {
                    menu_entry(menu_id, asset_server, entry, parent);
                }
            })
            .id();
    });

    commands.entity(menu_id).insert(menu_nodes);
}

/// Builds the menu button UI entity.
fn menu_button<R: Relationship>(
    menu_id: Entity,
    asset_server: &Res<AssetServer>,
    entry: &DropdownMenuEntry,
    commands: &mut RelatedSpawnerCommands<R>,
) -> Entity {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            DropdownMenuButton { menu: menu_id },
            BackgroundColor(BACKGROUND_COLOR_HIDDEN),
            BorderColor::all(BORDER_COLOR_HIDDEN),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|parent| {
            if let Some(icon) = &entry.icon {
                parent.spawn(menu_icon(icon));
            }

            if let Some(text) = &entry.text {
                parent.spawn(menu_text(text, asset_server));
            }
        })
        .id()
}

/// Builds a menu entry UI entity.
fn menu_entry<R: Relationship>(
    menu_id: Entity,
    asset_server: &Res<AssetServer>,
    entry: &DropdownMenuEntry,
    commands: &mut RelatedSpawnerCommands<R>,
) -> Entity {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            DropdownEntryButton { menu: menu_id },
            BackgroundColor(BACKGROUND_COLOR),
            BorderColor::all(BORDER_COLOR),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|parent| {
            if let Some(icon) = &entry.icon {
                parent.spawn(menu_icon(icon));
            }

            if let Some(text) = &entry.text {
                parent.spawn(menu_text(text, asset_server));
            }
        })
        .id()
}

/// Builds a menu icon UI entity.
fn menu_icon(icon: &Handle<Image>) -> impl Bundle {
    (
        Node {
            width: Val::Px(ICON_SIZE),
            height: Val::Px(ICON_SIZE),
            ..default()
        },
        ImageNode::new(icon.clone()),
    )
}

/// Builds a menu text UI entity.
fn menu_text(text: &str, asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        Node {
            height: Val::Px(ICON_SIZE),
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: TEXT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    )
}
