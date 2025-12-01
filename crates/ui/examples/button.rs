//! This example shows the dropdown menu component.

use awgen_ui::QUIVER_FONT;
use awgen_ui::prelude::*;
use awgen_ui::themes::hearth_theme;
use bevy::prelude::*;

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
            theme: hearth_theme(asset_server.load(QUIVER_FONT)),
        }),
        observe(|_: On<Activate>| {
            info!("Button clicked!");
        }),
    ));
}
