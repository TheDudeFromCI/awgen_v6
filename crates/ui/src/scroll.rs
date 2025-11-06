//! This module forwards scrolling events through the UI hierarchy.

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;

/// The multiplier for line-based scrolling.
const LINE_HEIGHT: f32 = 21.0;

/// A plugin that adds scrolling support to the UI.
pub struct ScrollPlugin;
impl Plugin for ScrollPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_systems(
            Update,
            (send_scroll_events, update_smooth_scroll_positions).chain(),
        )
        .add_observer(on_scroll_handler);
    }
}

/// UI scrolling event.
#[derive(Debug, EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct Scroll {
    /// The entity that received the scroll event.
    pub entity: Entity,

    /// Scroll delta in logical coordinates.
    pub delta: Vec2,
}

/// Smooth scroll position component.
#[derive(Debug, Default, Component, Clone, Copy, Deref, DerefMut)]
#[require(ScrollPosition)]
pub struct SmoothScrollPosition(pub Vec2);

/// Injects scroll events into the UI hierarchy.
fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

/// Handles scroll events and updates scroll positions.
fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(
        &mut ScrollPosition,
        Option<&mut SmoothScrollPosition>,
        &Node,
        &ComputedNode,
    )>,
) {
    let Ok((mut scroll_position, mut smooth_scroll, node, computed)) = query.get_mut(scroll.entity)
    else {
        return;
    };

    let pos = if let Some(smooth_scroll) = smooth_scroll.as_deref_mut() {
        &mut smooth_scroll.0
    } else {
        &mut scroll_position.0
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0.0 {
        let max = if delta.x > 0.0 {
            pos.x >= max_offset.x
        } else {
            pos.x <= 0.0
        };

        if !max {
            pos.x += delta.x;
            delta.x = 0.0;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0.0 {
        let max = if delta.y > 0.0 {
            pos.y >= max_offset.y
        } else {
            pos.y <= 0.0
        };

        if !max {
            pos.y += delta.y;
            delta.y = 0.0;
        }
    }

    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}

/// Updates smooth scroll positions.
fn update_smooth_scroll_positions(
    time: Res<Time>,
    mut query: Query<(&mut ScrollPosition, &SmoothScrollPosition)>,
) {
    let delta = time.delta_secs();
    let t = (1.0 - 0.01f32.powf(2.0 * delta)).clamp(0.0, 1.0);

    for (mut scroll_position, smooth_scroll) in query.iter_mut() {
        let src = scroll_position.0;
        let dst = smooth_scroll.0;
        scroll_position.0 = src.lerp(dst, t);
    }
}
