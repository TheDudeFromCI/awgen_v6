//! This module implements the color components used in the UI.

use std::marker::PhantomData;

use bevy::ecs::component::Mutable;
use bevy::prelude::*;

use crate::prelude::InteractionReceiver;
use crate::theme::ColorTheme;

/// A plugin that adds color interaction support to the UI.
pub struct ColorPlugin;
impl Plugin for ColorPlugin {
    fn build(&self, app_: &mut App) {
        app_.register_colorable::<BackgroundColor>()
            .register_colorable::<BorderColor>()
            .register_colorable::<TextColor>()
            .register_colorable::<ImageNode>();
    }
}

/// Extension trait for registering colorable components.
pub trait RegisterColorable {
    /// Registers a colorable component with the app.
    fn register_colorable<C>(&mut self) -> &mut Self
    where
        C: Component<Mutability = Mutable> + Colorable,
        ColorTarget<C>: Default;
}

impl RegisterColorable for App {
    fn register_colorable<C>(&mut self) -> &mut Self
    where
        C: Component<Mutability = Mutable> + Colorable,
        ColorTarget<C>: Default,
    {
        self.add_systems(
            Update,
            (on_interaction_changed::<C>, update_smooth_color::<C>).chain(),
        );
        self
    }
}

/// A theme for different colors based on interaction state.
#[derive(Debug, Component, Clone)]
#[require(InteractionReceiver, ColorTarget<C> = ColorTarget(PhantomData))]
pub struct InteractiveColor<C>
where
    C: Component<Mutability = Mutable> + Colorable,
{
    /// The color when the entity is in its default state.
    pub default: Color,

    /// The color when the entity is hovered.
    pub hovered: Color,

    /// The color when the entity is pressed.
    pub pressed: Color,

    /// The color when the entity is disabled.
    pub disable: Color,

    /// Marker.
    pub _marker: PhantomData<C>,
}

impl<C> From<&ColorTheme> for InteractiveColor<C>
where
    C: Component<Mutability = Mutable> + Colorable,
{
    fn from(theme: &ColorTheme) -> Self {
        InteractiveColor {
            default: theme.default,
            hovered: theme.hovered,
            pressed: theme.pressed,
            disable: theme.disable,
            _marker: PhantomData,
        }
    }
}

/// A component for smoothly transitioning an entity's color.
#[derive(Debug, Default, Component, Clone)]
#[require(ColorTarget<C> = ColorTarget(PhantomData))]
pub struct SmoothColor<C>
where
    C: Component<Mutability = Mutable> + Colorable,
{
    /// The target color to transition to.
    pub color: Option<Color>,

    /// Marker.
    pub _marker: PhantomData<C>,
}

/// A component that indicates which component's color should be targeted.
#[derive(Debug, Default, Component)]
pub struct ColorTarget<C>(PhantomData<C>)
where
    C: Component<Mutability = Mutable> + Colorable;

/// A trait for components that have a color.
pub trait Colorable {
    /// Gets the current color of the component.
    fn get_color(&self) -> Color;

    /// Sets the color of the component.
    fn set_color(&mut self, color: Color);
}

impl Colorable for BackgroundColor {
    fn get_color(&self) -> Color {
        self.0
    }

    fn set_color(&mut self, color: Color) {
        self.0 = color;
    }
}

impl Colorable for BorderColor {
    fn get_color(&self) -> Color {
        self.top
    }

    fn set_color(&mut self, color: Color) {
        self.top = color;
        self.right = color;
        self.bottom = color;
        self.left = color;
    }
}

impl Colorable for TextColor {
    fn get_color(&self) -> Color {
        self.0
    }

    fn set_color(&mut self, color: Color) {
        self.0 = color;
    }
}

impl Colorable for ImageNode {
    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

/// System that updates smooth color transitions.
fn update_smooth_color<C>(
    time: Res<Time>,
    mut query: Query<(&mut C, &SmoothColor<C>), With<ColorTarget<C>>>,
) where
    C: Component<Mutability = Mutable> + Colorable,
{
    let delta = time.delta_secs();
    let t = (1.0 - 0.01f32.powf(4.0 * delta)).clamp(0.0, 1.0);

    for (mut colorable, maybe_smooth) in query.iter_mut() {
        let Some(target_color) = maybe_smooth.color else {
            continue;
        };

        let old_color = colorable.get_color();
        let new_color = old_color.mix(&target_color, t);
        colorable.set_color(new_color);
    }
}

/// System that handles interaction change events and updates colors
/// accordingly.
#[allow(clippy::type_complexity)]
fn on_interaction_changed<C>(
    mut query: Query<
        (
            &mut C,
            Option<&mut SmoothColor<C>>,
            &InteractiveColor<C>,
            &InteractionReceiver,
        ),
        (Changed<InteractionReceiver>, With<ColorTarget<C>>),
    >,
) where
    C: Component<Mutability = Mutable> + Colorable,
{
    for (mut colorable, maybe_smooth, color, interaction) in query.iter_mut() {
        let color = match interaction {
            InteractionReceiver::Disable => color.disable,
            InteractionReceiver::Pressed => color.pressed,
            InteractionReceiver::Hovered => color.hovered,
            InteractionReceiver::Default => color.default,
        };

        match maybe_smooth {
            Some(mut smooth) => smooth.color = Some(color),
            None => colorable.set_color(color),
        }
    }
}
