//! This example shows the dropdown menu component.

use awgen_ui::prelude::*;
use bevy::prelude::*;

mod theme;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AwgenUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        ScreenAnchor::Center,
        button(ButtonBuilder {
            node: Node::default(),
            content: ButtonContent::text("Click Me"),
            theme: theme::heath_theme(&asset_server),
        }),
        observe(|_: On<Activate>| {
            info!("Button clicked!");
        }),
    ));
}
