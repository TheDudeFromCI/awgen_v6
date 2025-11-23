//! This crate provides utilities for managing and loading assets in the Awgen
//! game engine.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;

use bevy::asset::io::{AssetSource, AssetSourceId};
use bevy::prelude::*;

use crate::connection::AssetDatabase;
use crate::prelude::AssetDatabaseName;
use crate::source::{AwgenDbSource, AwgenDbWatcher};

pub mod connection;
pub mod loaders;
pub mod module;
pub mod param;
pub mod record;
pub mod source;

/// Prelude module for easy importing of commonly used items.
pub mod prelude {
    pub use super::connection::*;
    pub use super::loaders::*;
    pub use super::module::*;
    pub use super::param::*;
    pub use super::record::*;
    pub use super::{AwgenAssetPlugin, AwgenAssetPluginExt};
}

/// Bevy plugin for Awgen asset database support.
pub struct AwgenAssetPlugin;
impl Plugin for AwgenAssetPlugin {
    fn build(&self, app_: &mut App) {
        app_.register_asset_loader(loaders::AwgenImageAssetLoader);
    }
}

/// Extension trait for registering the Awgen asset database sources.
pub trait AwgenAssetPluginExt {
    /// Registers an Awgen asset database source with the given name and path.
    fn register_asset_db<N, P>(&mut self, path: P) -> &mut Self
    where
        N: AssetDatabaseName + Unpin + Send + Sync + 'static,
        P: Into<PathBuf>;
}

impl AwgenAssetPluginExt for App {
    fn register_asset_db<N, P>(&mut self, path: P) -> &mut Self
    where
        N: AssetDatabaseName + Unpin + Send + Sync + 'static,
        P: Into<PathBuf>,
    {
        let database = AssetDatabase::<N>::new(path).expect("Failed to connect to asset database");
        let reader = Box::new(AwgenDbSource {
            database: database.clone(),
        });
        let watcher = database.clone();

        self.insert_resource(database).register_asset_source(
            AssetSourceId::Name(N::database_name().into()),
            AssetSource::build()
                .with_reader(move || reader.clone())
                .with_watcher(move |sender| {
                    watcher.add_watcher(sender);
                    Some(Box::new(AwgenDbWatcher))
                }),
        )
    }
}
