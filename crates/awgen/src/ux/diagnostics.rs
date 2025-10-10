//! This module implements the diagnostics overlay for the Awgen game engine.

use bevy::diagnostic::{
    DiagnosticsStore,
    EntityCountDiagnosticsPlugin,
    FrameTimeDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;

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
            ),
        )
        .configure_sets(
            Update,
            (
                DiagnosticsOverlaySystems::BuildUI.after(DiagnosticsOverlaySystems::Toggle),
                DiagnosticsOverlaySystems::UpdateText.after(DiagnosticsOverlaySystems::Toggle),
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

/// A component used to identify the diagnostics overlay UI entity.
#[derive(Debug, Default, Component)]
pub struct DiagnosticsOverlayUI;

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
    diagnostics_overlay: Res<DiagnosticsOverlay>,
    diagnostics_store: Res<DiagnosticsStore>,
    overlay_ui: Query<Entity, With<DiagnosticsOverlayUI>>,
    mut commands: Commands,
) {
    // destroy any existing debug overlay
    for entity in overlay_ui.iter() {
        commands.entity(entity).despawn();
    }

    if !diagnostics_overlay.visible {
        return;
    }

    commands.spawn((
        DiagnosticsOverlayUI,
        Text::new(compute_text(&diagnostics_store)),
        TextLayout::new_with_justify(Justify::Left),
        TextColor::from(Color::WHITE),
        TextFont {
            font: diagnostics_overlay.font.clone(),
            font_size: 14.0,
            ..default()
        },
    ));
}

/// This system updates the diagnostics overlay text each frame.
fn update_text(
    time: Res<Time>,
    diagnostics_store: Res<DiagnosticsStore>,
    mut timer: ResMut<DiagnosticsOverlayTimer>,
    mut query: Query<&mut Text, With<DiagnosticsOverlayUI>>,
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
    let render_times = format!(
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

    render_times
}
