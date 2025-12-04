//! The implementation for the Awgen AssetExplorer tool.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;

use awgen_asset_db::prelude::*;
use awgen_ui::FOLDER_ICON;
use awgen_ui::prelude::*;
use awgen_ui::themes::hearth_theme;
use awgen_ui::widgets::grid_preview::GridPreview;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use clap::{Parser, command};

/// The arguments for the command line interface.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The project folder.
    #[arg(long)]
    project: PathBuf,
}

/// The project asset database identifier.
pub struct ProjectDatabase;
impl AssetDatabaseName for ProjectDatabase {
    fn database_name() -> &'static str {
        "project"
    }
}

fn main() {
    let args = Args::parse();

    App::new()
        .register_asset_db::<ProjectDatabase, _>(args.project)
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,naga=warn,calloop=debug,polling=debug,cosmic_text=info"
                    .to_string(),
                ..default()
            }),
            AwgenAssetPlugin,
            AwgenUiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

/// Initializes the asset explorer ui.
fn setup(
    asset_server: Res<AssetServer>,
    // asset_db: AwgenAssets<ProjectDatabase>,
    mut commands: Commands,
) {
    // let assets = asset_db.list_assets().expect("Failed to list assets");
    let theme = hearth_theme(&asset_server);
    let mut folders = tree_builder();
    let previews = grid_preview_builder();

    set_icon_recursive(&mut folders, asset_server.load(FOLDER_ICON));

    commands.spawn(Camera2d);
    commands.spawn((
        ScreenAnchor::Fullscreen,
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(4.0),
            ..default()
        },
        theme.outer_window.clone(),
        children![
            (
                Node {
                    width: percent(20.0),
                    ..default()
                },
                TreeView::from_builder(theme.clone(), folders),
            ),
            (
                Node {
                    width: percent(80.0),
                    ..default()
                },
                GridPreview::with_cells(theme, previews)
            )
        ],
    ));
}

/// Builds a sample tree structure for the TreeView.
fn tree_builder() -> TreeNodeBuilder {
    TreeNodeBuilder {
        content: TreeNodeContent::from("root"),
        children: vec![
            TreeNodeBuilder {
                content: TreeNodeContent::from("child 1"),
                children: vec![TreeNodeBuilder {
                    content: TreeNodeContent::from("grandchild 1.1"),
                    children: vec![],
                }],
            },
            TreeNodeBuilder {
                content: TreeNodeContent::from("child 2"),
                children: vec![
                    TreeNodeBuilder {
                        content: TreeNodeContent::from("grandchild 2.1"),
                        children: vec![],
                    },
                    TreeNodeBuilder {
                        content: TreeNodeContent::from("grandchild 2.2"),
                        children: vec![],
                    },
                    TreeNodeBuilder {
                        content: TreeNodeContent::from("grandchild 2.3"),
                        children: vec![],
                    },
                ],
            },
            TreeNodeBuilder {
                content: TreeNodeContent::from("child 3"),
                children: vec![
                    TreeNodeBuilder {
                        content: TreeNodeContent::from("grandchild 3.1"),
                        children: vec![],
                    },
                    TreeNodeBuilder {
                        content: TreeNodeContent::from("grandchild 3.2"),
                        children: vec![
                            TreeNodeBuilder {
                                content: TreeNodeContent::from("great-grandchild 3.2.1"),
                                children: vec![],
                            },
                            TreeNodeBuilder {
                                content: TreeNodeContent::from("great-grandchild 3.2.2"),
                                children: vec![],
                            },
                        ],
                    },
                    TreeNodeBuilder {
                        content: TreeNodeContent::from("grandchild 3.3"),
                        children: vec![],
                    },
                ],
            },
        ],
    }
}

/// Recursively sets the icon for a tree node and its children.
fn set_icon_recursive(node: &mut TreeNodeBuilder, icon: Handle<Image>) {
    node.content.icon = Some(icon.clone());
    for child in &mut node.children {
        set_icon_recursive(child, icon.clone());
    }
}

/// Builds sample grid preview cells.
fn grid_preview_builder() -> Vec<GridNodeBuilder> {
    vec![
        GridNodeBuilder {
            icon: Handle::default(),
            label: "Asset".into(),
        };
        10
    ]
}
