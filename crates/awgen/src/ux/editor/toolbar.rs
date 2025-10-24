//! This module implements the toolbar for the editor UX.

use bevy::prelude::*;

use crate::app::AwgenState;
use crate::ux::OverlayRoot;
use crate::ux::widgets::dropdown::{DropdownMenu, DropdownMenuEntry};

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
fn setup(
    asset_server: Res<AssetServer>,
    overlay: Query<Entity, With<OverlayRoot>>,
    mut commands: Commands,
) {
    let Ok(overlay) = overlay.single() else {
        error!("No overlay root found! Cannot initialize toolbar.");
        return;
    };

    commands.spawn((
        EditorToolbar,
        ChildOf(overlay),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
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
    ));
}

/// Cleans up the editor toolbar.
fn cleanup(toolbar: Query<Entity, With<EditorToolbar>>, mut commands: Commands) {
    for entity in toolbar.iter() {
        commands.entity(entity).despawn();
    }
}
