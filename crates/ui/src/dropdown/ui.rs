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
use crate::style::Style;

/// Builds the UI hierarchy for a dropdown menu.
pub(super) fn build_menu(
    menu_id: Entity,
    menu: &DropdownMenu,
    style: &Style,
    commands: &mut Commands,
) {
    let mut menu_nodes = DropdownMenuNodes {
        content_node: Entity::PLACEHOLDER,
    };

    commands.entity(menu_id).with_children(|parent| {
        menu_button(menu_id, menu.main_button(), style, parent);

        menu_nodes.content_node = parent
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(style.dropdown.element_spacing),
                    padding: style.dropdown.options.padding(),
                    border: style.dropdown.options.border_thickness(),
                    top: Val::Px(style.button.icon_size),
                    display: Display::None,
                    ..default()
                },
                style.dropdown.options.background_color(),
                style.dropdown.options.border_color(),
                style.dropdown.options.border_radius(),
            ))
            .with_children(|parent| {
                for entry in menu.entries() {
                    menu_entry(menu_id, style, entry, parent);
                }
            })
            .id();
    });

    commands.entity(menu_id).insert(menu_nodes);
}

/// Builds the menu button UI entity.
fn menu_button<R: Relationship>(
    menu_id: Entity,
    entry: &DropdownMenuEntry,
    style: &Style,
    commands: &mut RelatedSpawnerCommands<R>,
) -> Entity {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                padding: style.dropdown.button.padding(),
                border: style.dropdown.button.border_thickness(),
                ..default()
            },
            DropdownMenuButton { menu: menu_id },
            style.dropdown.button.background_color(),
            style.dropdown.button.border_color(),
            style.dropdown.button.border_radius(),
        ))
        .with_children(|parent| {
            if let Some(icon) = &entry.icon {
                parent.spawn(menu_icon(style, icon));
            }

            if let Some(text) = &entry.text {
                parent.spawn(menu_text(style, text.to_string()));
            }
        })
        .id()
}

/// Builds a menu entry UI entity.
fn menu_entry<R: Relationship>(
    menu_id: Entity,
    style: &Style,
    entry: &DropdownMenuEntry,
    commands: &mut RelatedSpawnerCommands<R>,
) -> Entity {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                align_items: AlignItems::Center,
                height: Val::Px(style.button.icon_size),
                ..default()
            },
            DropdownEntryButton { menu: menu_id },
        ))
        .with_children(|parent| {
            if let Some(icon) = &entry.icon {
                parent.spawn(menu_icon(style, icon));
            }

            if let Some(text) = &entry.text {
                parent.spawn(menu_text(style, text.to_string()));
            }
        })
        .id()
}

/// Builds a menu icon UI entity.
fn menu_icon(style: &Style, icon: &Handle<Image>) -> impl Bundle {
    (
        Node {
            width: Val::Px(style.button.icon_size),
            height: Val::Px(style.button.icon_size),
            ..default()
        },
        ImageNode::new(icon.clone()),
    )
}

/// Builds a menu text UI entity.
fn menu_text(style: &Style, text: String) -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(text),
            TextFont {
                font: style.dropdown.font_style.font.clone(),
                font_size: style.dropdown.font_style.font_size,
                ..default()
            },
            TextColor(style.dropdown.font_style.font_color()),
        )],
    )
}
