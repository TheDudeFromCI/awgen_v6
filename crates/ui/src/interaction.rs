//! This module extends the widget interaction systems,

use bevy::app::{HierarchyPropagatePlugin, Propagate};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::reflect::Is;
use bevy::ui::{InteractionDisabled, Pressed};

/// A plugin that adds improved interaction support to the UI.
pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app_: &mut App) {
        app_.add_plugins(HierarchyPropagatePlugin::<InteractionReceiver>::new(
            PreUpdate,
        ))
        .add_observer(update_interaction::<Insert, Hovered>)
        .add_observer(update_interaction::<Add, Pressed>)
        .add_observer(update_interaction::<Remove, Pressed>)
        .add_observer(update_interaction::<Add, InteractionDisabled>)
        .add_observer(update_interaction::<Remove, InteractionDisabled>);
    }
}

/// The interaction state of a UI component. This component receives interaction
/// events sent by an [`InteractionSender`], and can be used to determine the
/// current interaction state of the component for visual updates.
///
/// Adding a [`Propagate(InteractionReceiver)`] component to a parent entity
/// will forward the interaction state to all descendants.
#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionReceiver {
    /// The default state.
    #[default]
    Default,

    /// The widget is being hovered over.
    Hovered,

    /// The widget is being pressed.
    Pressed,

    /// The widget is disabled.
    Disable,
}

/// A component that listens for user interaction and forwards that too all
/// entities via the [`InteractionReceiver`] component.
///
/// Adding this component to an entity will automatically add the
/// [`InteractionReceiver`] component to it as well.
#[derive(Debug, Default, Component)]
#[require(Hovered, Propagate<InteractionReceiver> = Propagate(InteractionReceiver::Default))]
pub struct InteractionSender;

/// System that updates and forwards interaction events to receivers based on
/// user input.
#[allow(clippy::type_complexity)]
fn update_interaction<E, A>(
    trigger: On<E, A>,
    mut query: Query<(
        Option<&mut InteractionReceiver>,
        Option<&mut Propagate<InteractionReceiver>>,
        Has<Pressed>,
        Has<InteractionDisabled>,
        &Hovered,
    )>,
) where
    E: EntityEvent,
    A: Component,
{
    let Ok((maybe_interact, maybe_propagate, pressed, disabled, hovered)) =
        query.get_mut(trigger.event_target())
    else {
        return;
    };

    let pressed = pressed && !(E::is::<Remove>() && A::is::<Pressed>());
    let disabled = disabled && !(E::is::<Remove>() && A::is::<InteractionDisabled>());

    let state = match (disabled, hovered.0, pressed) {
        (true, _, _) => InteractionReceiver::Disable,
        (false, _, true) => InteractionReceiver::Pressed,
        (false, true, false) => InteractionReceiver::Hovered,
        (false, false, false) => InteractionReceiver::Default,
    };

    match (maybe_propagate, maybe_interact) {
        (Some(mut propagate), _) => {
            propagate.0 = state;
        }
        (None, Some(mut interact)) => {
            *interact = state;
        }
        _ => {}
    }
}
