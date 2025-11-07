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
            .add_observer(clear_3d_model)
            .add_observer(replace_anchor);
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

/// An enum representing the different screen anchor positions.
///
/// Adding this component to a UI node will automatically position it
/// according to the specified anchor as a child of the [`OverlayRoot`] at the
/// specified position and then remove this component.
///
/// This component will automatically overwrite the node's position type to
/// `Absolute` and set the appropriate margin and top/bottom/left/right values.
///
/// Relative margin values will be preserved.
#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
#[component(storage = "SparseSet")]
#[require(Node)]
pub enum ScreenAnchor {
    /// Top left corner of the screen.
    TopLeft,

    /// Top center of the screen.
    TopCenter,

    /// Top right corner of the screen.
    TopRight,

    /// Center left of the screen.
    CenterLeft,

    /// Center of the screen.
    Center,

    /// Center right of the screen.
    CenterRight,

    /// Bottom left corner of the screen.
    BottomLeft,

    /// Bottom center of the screen.
    BottomCenter,

    /// Bottom right corner of the screen.
    BottomRight,
}

impl ScreenAnchor {
    /// Sets the given node's position and margin according to this anchor.
    pub fn set_node(&self, node: &mut Node) {
        node.position_type = PositionType::Absolute;
        node.top = Val::Auto;
        node.bottom = Val::Auto;
        node.left = Val::Auto;
        node.right = Val::Auto;

        node.margin = UiRect {
            left: match node.margin.left {
                Val::Auto => Val::Px(0.0),
                value => value,
            },
            right: match node.margin.right {
                Val::Auto => Val::Px(0.0),
                value => value,
            },
            top: match node.margin.top {
                Val::Auto => Val::Px(0.0),
                value => value,
            },
            bottom: match node.margin.bottom {
                Val::Auto => Val::Px(0.0),
                value => value,
            },
        };

        match self {
            ScreenAnchor::TopLeft => {
                node.top = Val::Px(0.0);
                node.left = Val::Px(0.0);
            }
            ScreenAnchor::TopCenter => {
                node.top = Val::Px(0.0);
                node.margin = UiRect::AUTO
                    .with_top(node.margin.top)
                    .with_bottom(node.margin.bottom);
            }
            ScreenAnchor::TopRight => {
                node.top = Val::Px(0.0);
                node.right = Val::Px(0.0);
            }
            ScreenAnchor::CenterLeft => {
                node.left = Val::Px(0.0);
                node.margin = UiRect::AUTO
                    .with_left(node.margin.left)
                    .with_right(node.margin.right);
            }
            ScreenAnchor::Center => {
                node.margin = UiRect::AUTO;
            }
            ScreenAnchor::CenterRight => {
                node.right = Val::Px(0.0);
                node.margin = UiRect::AUTO
                    .with_right(node.margin.right)
                    .with_left(node.margin.left);
            }
            ScreenAnchor::BottomLeft => {
                node.bottom = Val::Px(0.0);
                node.left = Val::Px(0.0);
            }
            ScreenAnchor::BottomCenter => {
                node.bottom = Val::Px(0.0);
                node.margin = UiRect::AUTO
                    .with_bottom(node.margin.bottom)
                    .with_top(node.margin.top);
            }
            ScreenAnchor::BottomRight => {
                node.bottom = Val::Px(0.0);
                node.right = Val::Px(0.0);
            }
        }
    }
}

/// Replaces the ScreenAnchor component with appropriate positioning and
/// parenting.
fn replace_anchor(
    trigger: On<Add, ScreenAnchor>,
    overlay: Query<Entity, With<OverlayRoot>>,
    mut query: Query<(&mut Node, &ScreenAnchor)>,
    mut commands: Commands,
) {
    let entity = trigger.event().entity;
    let Ok((mut node, anchor)) = query.get_mut(entity) else {
        error!("Failed to replace ScreenAnchor: could not get Node component");
        return;
    };

    let Ok(overlay) = overlay.single() else {
        error!("Failed to replace ScreenAnchor: no OverlayRoot found");
        return;
    };

    anchor.set_node(&mut node);

    commands
        .entity(entity)
        .remove::<ScreenAnchor>()
        .insert(ChildOf(overlay));
}
