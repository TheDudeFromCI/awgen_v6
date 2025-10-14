//! This plugin handles the overlay UI logic.

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// The plugin that adds an overlay to the application.
pub struct OverlayPlugin;
impl Plugin for OverlayPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_systems(Startup, setup)
            .add_systems(
                Update,
                update_3d_elements.in_set(OverlaySystems::Update3DPositions),
            )
            .add_observer(clear_3d_model);
    }
}

/// The different systems used by the overlay plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum OverlaySystems {
    /// Updates the 3D positions of overlay elements.
    Update3DPositions,
}

/// Marker component for the overlay root node.
#[derive(Component)]
pub struct OverlayRoot;

/// A component that marks a 3D element in the overlay. This component is placed
/// on the UI node.
///
/// Every frame, the entity indicated by this component will have its transform
/// updated to match the position of this UI node.
///
/// Destroying the UI node will also despawn the 3D entity.
///
/// That target entity should be set to [`RenderLayer`] 1 to be visible in the
/// overlay camera.
#[derive(Debug, Component)]
#[require(Transform)]
pub struct Node3D(pub Entity);

/// Sets up the overlay camera and root node.
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        RenderLayers::layer(1),
        Transform::default(),
        AmbientLight {
            color: Color::WHITE,
            brightness: 5000.0,
            affects_lightmapped_meshes: true,
        },
        Camera {
            order: 1,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scaling_mode: bevy::camera::ScalingMode::WindowSize,
            scale: 1.0,
            viewport_origin: Vec2::new(0.0, 0.0),
            area: Rect::new(0.0, 0.0, 1.0, 1.0),
        }),
    ));

    commands.spawn((
        OverlayRoot,
        Node {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Px(0.0)),
            padding: UiRect::all(Val::Px(0.0)),
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
    ));
}

/// This system updates the transforms of 3D elements to match their
/// corresponding UI nodes.
fn update_3d_elements(
    mut elements: Query<&mut Transform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    ui_nodes: Query<(&UiGlobalTransform, &Node3D)>,
) {
    let Ok(window) = windows.single() else {
        warn_once!("OverlayPlugin: No primary window found, cannot update 3D overlay elements");
        return;
    };

    let window_height = window.resolution.height();
    for (ui_transform, Node3D(entity)) in ui_nodes.iter() {
        if let Ok(mut transform) = elements.get_mut(*entity) {
            let mut position = ui_transform.transform_point2(Vec2::ZERO);
            position.y = window_height - position.y;
            transform.translation = Vec3::new(position.x, position.y, 0.0);
        }
    }
}

/// This system cleans up 3D models when their corresponding UI nodes are
/// removed.
fn clear_3d_model(trigger: On<Remove, Node3D>, nodes: Query<&Node3D>, mut commands: Commands) {
    let entity = trigger.event().entity;
    let node3d = nodes.get(entity).unwrap();
    commands.entity(node3d.0).despawn();
}
