//! This module implements the toolbar for the editor UX.

use awgen_ui::dropdown::{DropdownMenu, DropdownMenuEntry};
use awgen_ui::overlay::ScreenAnchor;
use bevy::prelude::*;

use crate::app::AwgenState;

/// Plugin that sets up the editor toolbar.
pub struct EditorToolbarPlugin;
impl Plugin for EditorToolbarPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_systems(OnEnter(AwgenState::Editor), setup)
            .add_systems(OnExit(AwgenState::Editor), cleanup);
    }
}

/// A marker component for the editor toolbar.
#[derive(Debug, Component)]
pub struct EditorToolbar;

/// Sets up the editor toolbar.
fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        EditorToolbar,
        ScreenAnchor::TopLeft,
        Node {
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(5.0),
            ..default()
        },
        children![
            DropdownMenu::new(
                DropdownMenuEntry {
                    icon: Some(asset_server.load("editor://icons/settings.png")),
                    text: None,
                },
                vec![
                    DropdownMenuEntry {
                        icon: Some(asset_server.load("editor://icons/settings.png")),
                        text: Some("Project".to_string()),
                    },
                    DropdownMenuEntry {
                        icon: Some(asset_server.load("editor://icons/settings.png")),
                        text: Some("Assets".to_string()),
                    },
                    DropdownMenuEntry {
                        icon: Some(asset_server.load("editor://icons/settings.png")),
                        text: Some("Help".to_string()),
                    },
                ],
            ),
            DropdownMenu::new(
                DropdownMenuEntry {
                    icon: Some(asset_server.load("editor://icons/map.png")),
                    text: None,
                },
                vec![DropdownMenuEntry {
                    icon: Some(asset_server.load("editor://icons/map.png")),
                    text: Some("Open Map".to_string()),
                }],
            ),
        ],
    ));
}

/// Cleans up the editor toolbar.
fn cleanup(toolbar: Query<Entity, With<EditorToolbar>>, mut commands: Commands) {
    for entity in toolbar.iter() {
        commands.entity(entity).despawn();
    }
}
