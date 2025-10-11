//! This module implements camera functionality to the game engine.

use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

/// This plugin implements camera functionality to the game engine.
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_systems(Startup, setup_camera.in_set(CameraSystems::Setup))
            .add_systems(
                Update,
                (
                    lerp_camera.in_set(CameraSystems::UpdatePosition),
                    rotate_camera.in_set(CameraSystems::Controls),
                    zoom_camera_mouse.in_set(CameraSystems::Controls),
                    pan_camera_mouse.in_set(CameraSystems::Controls),
                ),
            )
            .configure_sets(
                Update,
                CameraSystems::Controls.before(CameraSystems::UpdatePosition),
            );
    }
}

/// The system sets for the camera plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum CameraSystems {
    /// The system set for the camera setup.
    Setup,

    /// The system set for the camera controls.
    ///
    /// This set is executed before the [`CameraSystemSet::UpdatePosition`] set.
    Controls,

    /// The system set for the camera positioning.
    ///
    /// This set is executed after the [`CameraSystemSet::Controls`] set.
    UpdatePosition,
}

/// This component is used to control the camera position, rotation, scale, and
/// distance.
#[derive(Debug, Component)]
pub struct CameraController {
    /// Target position of the camera.
    pub target_pos: Vec3,

    /// Target rotation of the camera in Euler angles (in radians).
    pub target_rot: Vec3,

    /// Target distance of the camera from the origin.
    pub target_dist: f32,

    /// Current position of the camera.
    ///
    /// In most situations, this value should not be modified directly. It is
    /// recommended to modify the `target_pos` instead, letting the camera
    /// smoothly interpolate to the new position.
    pos: Vec3,

    /// Current rotation of the camera in Euler angles (in radians).
    ///
    /// In most situations, this value should not be modified directly. It is
    /// recommended to modify the `target_rot` instead, letting the camera
    /// smoothly interpolate to the new rotation.
    rot: Vec3,

    /// Current distance of the camera from the origin.
    ///
    /// In most situations, this value should not be modified directly. It is
    /// recommended to modify the `targetDist` instead, letting the camera
    /// smoothly interpolate to the new distance.
    dist: f32,

    /// Smoothing factor for camera position.
    pub pos_smoothing: f32,

    /// Smoothing factor for camera rotation.
    pub rot_smoothing: f32,

    /// Smoothing factor for camera distance.
    pub dist_smoothing: f32,

    /// Whether or not the camera controls are active.
    pub active: bool,

    /// The minimum zoom level for the camera.
    pub min_zoom: f32,

    /// The maximum zoom level for the camera.
    pub max_zoom: f32,

    /// Sensitivity for zooming the camera with the mouse wheel.
    pub zoom_sensitivity: f32,

    /// Sensitivity for rotating the camera with the mouse.
    pub pan_sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            target_pos: Vec3::ZERO,
            target_rot: Vec3::new(45.0, 45.0, 0.0),
            target_dist: 16.0,

            pos: Vec3::ZERO,
            rot: Vec3::new(45.0, 45.0, 0.0),
            dist: 16.0,

            pos_smoothing: 0.01,
            rot_smoothing: 0.02,
            dist_smoothing: 0.0025,

            active: true,

            min_zoom: 4.0,
            max_zoom: 256.0,

            zoom_sensitivity: 1.0,
            pan_sensitivity: 1.0,
        }
    }
}

impl CameraController {
    /// Updates the camera's position, rotation, scale, and distance to the
    /// target values. This should be called every frame to smoothly.
    pub fn update(&mut self, delta: f32) {
        let pos_t = (1.0 - self.pos_smoothing.powf(10.0 * delta)).clamp(0.0, 1.0);
        self.pos = self.pos.lerp(self.target_pos, pos_t);

        let rot_t = (1.0 - self.rot_smoothing.powf(delta)).clamp(0.0, 1.0);
        self.rot = self.rot.lerp(self.target_rot, rot_t);

        let dist_t = (1.0 - self.dist_smoothing.powf(2.0 * delta)).clamp(0.0, 1.0);
        self.dist = self.dist.lerp(self.target_dist, dist_t);
    }

    /// Gets the current rotation of the camera as a quaternion.
    pub fn rotation(&self) -> Quat {
        Quat::from_euler(
            EulerRot::YXZ,
            self.rot.y.to_radians(),
            self.rot.x.to_radians(),
            self.rot.z.to_radians(),
        )
    }

    /// Gets the current true position of the camera, accounting for
    /// rotation and distance.
    pub fn translation(&self) -> Vec3 {
        self.pos + self.rotation() * Vec3::new(0.0, 0.0, -self.dist)
    }

    /// Gets the origin point of the camera, which is the position
    /// without any rotation or distance applied. The camera will always look
    /// at this location (not counting camera shake).
    pub fn origin(&self) -> Vec3 {
        self.pos
    }

    /// Gets the current up vector of the camera.
    pub fn up(&self) -> Vec3 {
        self.rotation() * Vec3::Y
    }

    /// Gets the current right vector of the camera, ignoring the Y component to
    /// ensure it is horizontal.
    pub fn right_plane(&self) -> Vec3 {
        (self.rotation() * Vec3::X * Vec3::new(1.0, 0.0, 1.0)).normalize()
    }

    /// Gets the current forward vector of the camera, ignoring the Y component
    /// to ensure it is horizontal.
    pub fn forward_plane(&self) -> Vec3 {
        (self.rotation() * Vec3::Z * Vec3::new(1.0, 0.0, 1.0)).normalize()
    }

    /// Zooms the camera in or out based on the given delta value.
    ///
    /// Value is clamped between `min_zoom` and `max_zoom`.
    pub fn zoom(&mut self, delta: f32) {
        self.target_dist =
            (self.target_dist * 1.25f32.powf(-delta)).clamp(self.min_zoom, self.max_zoom);
    }

    /// Rotates the camera clockwise by 90 degrees around the Y-axis.
    pub fn rotate_cw(&mut self) {
        self.target_rot.y += 90.0;
    }

    /// Rotates the camera counter-clockwise by 90 degrees around the Y-axis.
    pub fn rotate_ccw(&mut self) {
        self.target_rot.y -= 90.0;
    }
}

/// Creates the main camera on startup.
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        CameraController::default(),
        Transform::default(),
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 1.0,
            },
            scale: 1.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        }),
    ));
}

/// Smoothly moves the camera to the target position, rotation, scale, and
/// distance based on the `CameraController` component.
fn lerp_camera(
    mut query: Query<(&mut CameraController, &mut Transform, &mut Projection)>,
    time: Res<Time>,
) {
    for (mut controller, mut transform, mut projection) in query.iter_mut() {
        controller.update(time.delta_secs());
        transform.translation = controller.translation();
        transform.rotation = controller.rotation();
        transform.look_at(controller.origin(), controller.up());

        if let Projection::Orthographic(ortho) = &mut *projection {
            ortho.scale = controller.dist;
        }
    }
}

/// Rotates the camera direction based on keyboard input.
fn rotate_camera(
    mut camera_controllers: Query<&mut CameraController>,
    mut key_presses: MessageReader<KeyboardInput>,
) {
    for key_ev in key_presses.read() {
        if !key_ev.state.is_pressed() {
            continue;
        }

        if key_ev.key_code == KeyCode::KeyQ {
            for mut controller in camera_controllers.iter_mut() {
                if controller.active {
                    controller.rotate_ccw();
                }
            }
        }

        if key_ev.key_code == KeyCode::KeyE {
            for mut controller in camera_controllers.iter_mut() {
                if controller.active {
                    controller.rotate_cw();
                }
            }
        }
    }
}

/// Zooms the camera in and out based on mouse wheel input.
fn zoom_camera_mouse(
    mut camera_controllers: Query<&mut CameraController>,
    mut scroll: MessageReader<MouseWheel>,
) {
    let delta = scroll.read().map(|e| e.y).sum::<f32>();
    for mut controller in camera_controllers.iter_mut() {
        if controller.active {
            let offset = delta * controller.zoom_sensitivity;
            controller.zoom(offset);
        }
    }
}

/// Pans the camera based on mouse movement while the middle mouse button is
/// pressed.
fn pan_camera_mouse(
    mut last_mouse_pos: Local<Vec2>,
    mut camera_controllers: Query<&mut CameraController>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let height = window.height();
    let pos = window.cursor_position().unwrap_or(*last_mouse_pos);
    let delta = pos - *last_mouse_pos;
    *last_mouse_pos = pos;

    if !buttons.pressed(MouseButton::Middle) {
        return;
    }

    for mut controller in camera_controllers.iter_mut() {
        if controller.active {
            let mut offset = Vec3::ZERO;
            offset += controller.right_plane() * delta.x;
            offset += controller.forward_plane() * delta.y * 2f32.sqrt();
            offset *= controller.dist * controller.pan_sensitivity / height;
            offset.y = 0.0;
            controller.target_pos += offset;
        }
    }
}
