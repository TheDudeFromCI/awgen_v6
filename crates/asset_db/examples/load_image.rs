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
        .add_plugins((DefaultPlugins, AwgenAssetPlugin))
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
    assets: AwgenAssets<ExampleDatabase>,
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
    commands.spawn(ImageNode {
        image: assets.load_asset(awgen_image.record),
        ..Default::default()
    });

    commands.remove_resource::<AwgenImage>();
}
