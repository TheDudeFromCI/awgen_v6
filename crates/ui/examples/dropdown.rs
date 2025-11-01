//! This example shows the dropdown menu component.

use awgen_ui::AwgenUiPlugin;
use awgen_ui::button::AwgenButton;
use awgen_ui::dropdown::{DropdownMenu, DropdownMenuEntry};
use awgen_ui::overlay::ScreenAnchor;
use awgen_ui::style::{
    BorderStyle,
    ButtonAlignment,
    ButtonStyle,
    ColorStyle,
    ContainerStyle,
    DowndownMenuStyle,
    FontStyle,
    IconPosition,
    Style,
};
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

    let black = Color::srgb(0.0, 0.0, 0.0);
    let white = Color::srgb(1.0, 1.0, 1.0);

    let container = ContainerStyle {
        background: ColorStyle {
            default: black,
            hovered: black,
            pressed: black,
        },
        border: BorderStyle {
            color: ColorStyle {
                default: white,
                hovered: white,
                pressed: white,
            },
            thickness: 2.0,
            radius: 8.0,
        },
        padding: 5.0,
    };

    let font = FontStyle {
        font: Handle::default(),
        font_size: 16.0,
        color: ColorStyle {
            default: white,
            hovered: white,
            pressed: white,
        },
    };

    let style = Style {
        button: ButtonStyle {
            container: container.clone(),
            font: font.clone(),
            alignment: ButtonAlignment::Center,
            icon_position: IconPosition::Left,
            content_spacing: 5.0,
        },
        dropdown: DowndownMenuStyle {
            button: container.clone(),
            options: container.clone(),
            font_style: font.clone(),
            icon_size: 32.0,
            element_spacing: 0.0,
        },
    };

    commands.spawn((
        style.clone(),
        ScreenAnchor::TopCenter,
        AwgenButton::with_text("Click Me!"),
    ));

    commands.spawn((
        style.clone(),
        ScreenAnchor::Center,
        DropdownMenu::new(
            DropdownMenuEntry {
                icon: None,
                text: Some("Select an option".to_string()),
            },
            vec![
                DropdownMenuEntry {
                    icon: None,
                    text: Some("Option 1".to_string()),
                },
                DropdownMenuEntry {
                    icon: None,
                    text: Some("Option 2".to_string()),
                },
                DropdownMenuEntry {
                    icon: None,
                    text: Some("Option 3".to_string()),
                },
            ],
        ),
    ));
}
