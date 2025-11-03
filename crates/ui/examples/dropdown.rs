//! This example shows the dropdown menu component.

use awgen_ui::prelude::*;
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

    let button_style = ButtonStyle {
        container: container.clone(),
        font: font.clone(),
        alignment: ButtonAlignment::Center,
        icon_position: IconPosition::Left,
        icon_size: 32.0,
        content_spacing: 5.0,
    };

    let style = Style {
        button: button_style.clone(),
        dropdown: DowndownMenuStyle {
            button: container.clone(),
            options: container.clone(),
            font_style: font.clone(),
            element_spacing: 0.0,
        },
    };

    commands.spawn((
        ScreenAnchor::TopCenter,
        button(
            ButtonBuilder::default()
                .with_layout(WidgetLayout::Anchored {
                    position: ScreenAnchor::TopCenter,
                })
                .with_text("Click Me!")
                .with_style(button_style.clone()),
        ),
        observe(|_: On<Activate>| {
            info!("Button clicked!");
        }),
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
