//! This module implements the systems used by the button widget.

use bevy::prelude::*;

use crate::button::AwgenButton;
use crate::style::Style;

/// System that runs when a button is spawned.
pub(super) fn on_spawn(
    trigger: On<Add, AwgenButton>,
    query: Query<(&AwgenButton, &Style, &Node)>,
    mut commands: Commands,
) {
    let entity = trigger.entity;
    let (button, style, node) = query.get(entity).unwrap();
    debug!("Spawning AwgenButton: {}", entity);

    commands
        .entity(entity)
        .insert(style.button.container.bundle(node.clone()))
        .with_children(|parent| {
            if let Some(icon) = button.icon() {
                parent.spawn((
                    Node {
                        width: Val::Px(style.dropdown.icon_size),
                        height: Val::Px(style.dropdown.icon_size),
                        ..default()
                    },
                    ImageNode::new(icon.clone()),
                ));
            }

            if let Some(text) = button.text() {
                parent.spawn((
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
                ));
            }
        });
}
