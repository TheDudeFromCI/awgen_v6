//! This example shows the tree widget in action.

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
        tree_view(TreeBuilder {
            node: Node {
                width: px(250.0),
                height: px(500.0),
                ..default()
            },
            root_elements: build_tree(),
            theme: hearth_theme(asset_server.load(QUIVER_FONT)),
        }),
    ));
}

fn build_tree() -> Vec<TreeNode> {
    let mut root_nodes = Vec::new();

    for i in 0 .. 10 {
        let name = format!("Node {}", i);
        root_nodes.push(build_node(name, 0));
    }

    root_nodes
}

fn build_node(name: String, depth: u32) -> TreeNode {
    let mut children = Vec::new();

    if depth < 3 {
        for i in 0 .. 3 {
            let child_name = format!("{}.{}", name, i);
            children.push(build_node(child_name, depth + 1));
        }
    }

    TreeNode {
        content: ButtonContent::Label(name),
        children,
    }
}
