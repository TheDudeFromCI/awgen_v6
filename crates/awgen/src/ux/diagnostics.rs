//! This module implements the diagnostics overlay for the Awgen game engine.

use bevy::camera::visibility::RenderLayers;
use bevy::diagnostic::{
    DiagnosticsStore,
    EntityCountDiagnosticsPlugin,
    FrameTimeDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use lazy_static::lazy_static;

use crate::ux::{CameraController, Node3D, OverlayRoot};

/// The length of the axis indicator in the overlay.
const AXIS_INDICATOR_LEN: f32 = 20.0;

/// The thickness of the axis indicator in the overlay.
const AXIS_INDICATOR_WIDTH: f32 = 2.0;

lazy_static! {
    /// The number of CPU cores on the system.
    static ref CORE_COUNT: u32 = sys_info::cpu_num().unwrap_or(1);

    /// The CPU frequency in GHz.
    static ref CPU_FREQUENCY: f32 = sys_info::cpu_speed().unwrap_or(1000) as f32 / 1000.0;

    /// The system OS.
    static ref OS: String = format!(
        "{} {}",
        sys_info::os_type().unwrap_or_else(|_| "Unknown".into()),
        sys_info::os_release().unwrap_or_else(|_| "Unknown".into())
    );

    /// The maximum memory in MB.
    static ref MAX_MEMORY: u64 = sys_info::mem_info().map(|m| m.total / 1024 / 1024).unwrap_or(0);
}

/// The plugin that adds a diagnostics overlay to the application.
pub struct DiagnosticsOverlayPlugin;
impl Plugin for DiagnosticsOverlayPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin,
            RenderDiagnosticsPlugin,
        ))
        .init_resource::<DiagnosticsOverlay>()
        .init_resource::<DiagnosticsOverlayTimer>()
        .add_systems(
            Update,
            (
                toggle_diagnostics_overlay.in_set(DiagnosticsOverlaySystems::Toggle),
                build_diagnostics_overlay
                    .in_set(DiagnosticsOverlaySystems::BuildUI)
                    .run_if(resource_changed::<DiagnosticsOverlay>),
                update_text
                    .in_set(DiagnosticsOverlaySystems::UpdateText)
                    .run_if(not(resource_changed::<DiagnosticsOverlay>)),
                update_axis_indicator.in_set(DiagnosticsOverlaySystems::UpdateAxisIndicator),
            ),
        )
        .configure_sets(
            Update,
            (
                DiagnosticsOverlaySystems::BuildUI.after(DiagnosticsOverlaySystems::Toggle),
                DiagnosticsOverlaySystems::UpdateText.after(DiagnosticsOverlaySystems::Toggle),
                DiagnosticsOverlaySystems::UpdateAxisIndicator
                    .after(DiagnosticsOverlaySystems::Toggle),
            ),
        );
    }
}

/// The SystemSets for the diagnostics overlay.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum DiagnosticsOverlaySystems {
    /// The system set for toggling the diagnostics overlay.
    Toggle,

    /// The system set for building (or destroying) the diagnostics overlay UI.
    BuildUI,

    /// The system set for updating the diagnostics overlay text.
    UpdateText,

    /// The system set for updating the world axis indicator.
    UpdateAxisIndicator,
}

/// The resource which contains the settings for the diagnostics overlay.
#[derive(Debug, Default, Resource)]
pub struct DiagnosticsOverlay {
    /// The font used for the overlay text.
    pub font: Handle<Font>,

    /// Whether the overlay is visible.
    pub visible: bool,
}

/// A timer resource used to control the update rate of the diagnostics overlay.
#[derive(Debug, Resource)]
pub struct DiagnosticsOverlayTimer(Timer);

impl Default for DiagnosticsOverlayTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

/// A component used to identify a diagnostics overlay UI entity.
#[derive(Debug, Default, Component)]
pub struct DiagnosticsText;

/// A component used to identify the world axis indicator entity.
#[derive(Debug, Default, Component)]
pub struct WorldAxisIndicator;

/// This system toggles the visibility of the diagnostics overlay when the F3
/// key is pressed.
fn toggle_diagnostics_overlay(
    mut diagnostics_overlay: ResMut<DiagnosticsOverlay>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        diagnostics_overlay.visible = !diagnostics_overlay.visible;
    }
}

/// This system builds or destroys the diagnostics overlay UI based on the
/// `DiagnosticsOverlay.visible` flag.
fn build_diagnostics_overlay(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    diagnostics_overlay: Res<DiagnosticsOverlay>,
    diagnostics_store: Res<DiagnosticsStore>,
    overlay_ui: Query<Entity, With<DiagnosticsText>>,
    overlay_root: Query<Entity, With<OverlayRoot>>,
    mut commands: Commands,
) {
    // destroy any existing debug overlay
    for entity in overlay_ui.iter() {
        commands.entity(entity).despawn();
    }

    if !diagnostics_overlay.visible {
        return;
    }

    let Ok(overlay_root) = overlay_root.single() else {
        error!("No OverlayRoot found when trying to build diagnostics overlay");
        return;
    };

    let axis_indicator = commands
        .spawn((
            WorldAxisIndicator,
            Transform::default(),
            InheritedVisibility::default(),
            children![
                (
                    RenderLayers::layer(1),
                    Mesh3d(meshes.add(Cuboid::new(
                        AXIS_INDICATOR_LEN,
                        AXIS_INDICATOR_WIDTH,
                        AXIS_INDICATOR_WIDTH
                    ))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                    Transform::from_translation(Vec3::new(-AXIS_INDICATOR_LEN / 2.0, 0.0, 0.0))
                ),
                (
                    RenderLayers::layer(1),
                    Mesh3d(meshes.add(Cuboid::new(
                        AXIS_INDICATOR_WIDTH,
                        AXIS_INDICATOR_LEN,
                        AXIS_INDICATOR_WIDTH
                    ))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
                    Transform::from_translation(Vec3::new(0.0, AXIS_INDICATOR_LEN / 2.0, 0.0))
                ),
                (
                    RenderLayers::layer(1),
                    Mesh3d(meshes.add(Cuboid::new(
                        AXIS_INDICATOR_WIDTH,
                        AXIS_INDICATOR_WIDTH,
                        AXIS_INDICATOR_LEN
                    ))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
                    Transform::from_translation(Vec3::new(0.0, 0.0, -AXIS_INDICATOR_LEN / 2.0))
                ),
            ],
        ))
        .id();

    commands.spawn((
        ChildOf(overlay_root),
        DiagnosticsText,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
        Text::new(compute_text(&diagnostics_store)),
        TextLayout::new_with_justify(Justify::Left),
        TextColor::from(Color::WHITE),
        TextBackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
        TextFont {
            font: diagnostics_overlay.font.clone(),
            font_size: 14.0,
            ..default()
        },
    ));

    let axis_radius = AXIS_INDICATOR_LEN + 2.0;
    commands.spawn((
        ChildOf(overlay_root),
        DiagnosticsText,
        Node {
            position_type: PositionType::Absolute,
            margin: Val::Auto.into(),
            width: Val::Px(axis_radius * 2.0),
            height: Val::Px(axis_radius * 2.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        BorderRadius::all(Val::Px(axis_radius)),
        Node3D(axis_indicator),
    ));
}

/// This system updates the diagnostics overlay text each frame.
fn update_text(
    time: Res<Time>,
    diagnostics_store: Res<DiagnosticsStore>,
    mut timer: ResMut<DiagnosticsOverlayTimer>,
    mut query: Query<&mut Text, With<DiagnosticsText>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for mut text_component in query.iter_mut() {
        text_component.0 = compute_text(&diagnostics_store);
    }
}

/// Builds the diagnostics overlay text from the diagnostics store.
fn compute_text(store: &Res<DiagnosticsStore>) -> String {
    let system = format!(
        "System {} / Cpu: {:.1}% ({:.1}x{} Ghz) / Mem: {:.0}/{} MB",
        &*OS,
        store
            .get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE)
            .and_then(|cpu| cpu.smoothed())
            .unwrap_or(0.0),
        &*CPU_FREQUENCY,
        &*CORE_COUNT,
        store
            .get(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE)
            .and_then(|memory| memory.smoothed())
            .map(|mem| mem * 1024.0)
            .unwrap_or(0.0),
        &*MAX_MEMORY
    );

    let fps = format!(
        "FPS: {:.0} avg / {:.0} min / {:.0} max ({:.1}ms)",
        store
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
            .unwrap_or(0.0),
        store
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .map(|fps| fps.values().copied().fold(f64::INFINITY, f64::min))
            .unwrap_or(0.0),
        store
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .map(|fps| fps.values().copied().fold(f64::NEG_INFINITY, f64::max))
            .unwrap_or(0.0),
        store
            .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .and_then(|frame_time| frame_time.smoothed())
            .unwrap_or(0.0)
    );

    let geometry = format!(
        "Geometry:\n - Map: {} chunks / {} meshes / {} triangles\n",
        store
            .get(&crate::map::CHUNK_COUNT)
            .and_then(|chunk_count| chunk_count.value())
            .map(|v| v as u32)
            .unwrap_or(0),
        store
            .get(&crate::map::MESH_COUNT)
            .and_then(|mesh_count| mesh_count.value())
            .map(|v| v as u32)
            .unwrap_or(0),
        store
            .get(&crate::map::TRIANGLE_COUNT)
            .and_then(|triangle_count| triangle_count.value())
            .map(|v| v as u32)
            .unwrap_or(0)
    );

    format!("{system}\n{fps}\n{geometry}")
}

/// This system updates the rotation of the world axis indicator to reflect the
/// camera's orientation.
fn update_axis_indicator(
    camera: Query<&CameraController>,
    mut indicator: Query<&mut Transform, With<WorldAxisIndicator>>,
) {
    let Ok(controller) = camera.single() else {
        warn_once!("No CameraController found when trying to update world axis indicator");
        return;
    };

    for mut transform in indicator.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            controller.rot.x.to_radians(),
            (-controller.rot.y).to_radians(),
            controller.rot.z.to_radians(),
        );
    }
}
