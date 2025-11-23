//! This module implements the toolbar for the editor UX.

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
fn setup() {}

/// Cleans up the editor toolbar.
fn cleanup(toolbar: Query<Entity, With<EditorToolbar>>, mut commands: Commands) {
    for entity in toolbar.iter() {
        commands.entity(entity).despawn();
    }
}
