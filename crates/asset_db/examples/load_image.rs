//! This example demonstrates how to load an image asset using the Awgen asset
//! management system.

use awgen_assets::prelude::*;
use bevy::asset::LoadState;
use bevy::prelude::*;

struct ExampleDatabase;
impl AssetDatabaseName for ExampleDatabase {
    fn database_name() -> &'static str {
        "example"
    }
}

/// A resource to hold the test image handle while it is loading.
#[derive(Resource)]
struct LoadingTestImage {
    /// The handle to the test image.
    handle: Handle<Image>,
}

/// A resource to hold an asset record ID for the saved image.
#[derive(Resource)]
struct AwgenImage {
    /// An asset record ID for the image.
    record: AssetRecordID,
}

fn main() {
    App::new()
        .register_asset_db::<ExampleDatabase, _>(":memory:")
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "wgpu=error,naga=warn,calloop=debug,polling=debug,cosmic_text=info"
                    .to_string(),
                ..default()
            }),
            AwgenAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                save_image.run_if(resource_exists::<LoadingTestImage>),
                show_image.run_if(resource_exists::<AwgenImage>),
            ),
        )
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(LoadingTestImage {
        handle: asset_server.load("test_image.png"),
    });
}

fn save_image(
    test_image: Res<LoadingTestImage>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut assets: AwgenAssets<ExampleDatabase>,
    mut commands: Commands,
) {
    match asset_server.get_load_state(&test_image.handle) {
        Some(LoadState::Loading) => return,
        Some(LoadState::Loaded) => {}
        _ => panic!("Failed to load test image"),
    }
    commands.remove_resource::<LoadingTestImage>();

    let image = images.get(&test_image.handle).unwrap();

    let module_id = assets.create_module("Example Module").unwrap();
    let asset_id = assets.create_asset("test_image", module_id, image).unwrap();

    commands.insert_resource(AwgenImage { record: asset_id });
}

fn show_image(
    awgen_image: Res<AwgenImage>,
    assets: AwgenAssets<ExampleDatabase>,
    mut commands: Commands,
) {
    commands.spawn((
        Node {
            margin: UiRect::AUTO,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                children![
                    Text::new("Loaded Image"),
                    (
                        Node {
                            width: Val::Px(256.0),
                            height: Val::Px(256.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                        BorderRadius::all(Val::Px(4.0)),
                        ImageNode {
                            image: assets.load_asset(awgen_image.record),
                            ..default()
                        }
                    )
                ],
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                children![
                    Text::new("Preview Image"),
                    (
                        Node {
                            width: Val::Px(256.0),
                            height: Val::Px(256.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                        BorderRadius::all(Val::Px(4.0)),
                        ImageNode {
                            image: assets.load_asset_preview(awgen_image.record),
                            ..default()
                        }
                    )
                ],
            )
        ],
    ));

    commands.remove_resource::<AwgenImage>();
}
