//! This example shows the dropdown menu component.

use awgen_ui::AwgenUiPlugin;
use awgen_ui::dropdown::{DropdownMenu, DropdownMenuEntry, DropdownMenuEntryText};
use awgen_ui::overlay::ScreenAnchor;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AwgenUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        ScreenAnchor::Center,
        DropdownMenu::new(
            option("Select an option"),
            vec![option("Option 1"), option("Option 2"), option("Option 3")],
        ),
    ));
}

fn option(text: &str) -> DropdownMenuEntry {
    DropdownMenuEntry {
        icon: None,
        text: Some(DropdownMenuEntryText {
            content: text.to_string(),
            font: Handle::default(),
        }),
    }
}
