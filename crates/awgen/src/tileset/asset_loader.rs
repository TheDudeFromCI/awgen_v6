//! The asset loader for the Awgen tileset file format.

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;

use crate::tileset::tileset::{Tileset, TilesetError};

/// The asset loader for the Awgen tileset file format.
#[derive(Debug, Default)]
pub struct TilesetAssetLoader;
impl AssetLoader for TilesetAssetLoader {
    type Asset = Image;
    type Settings = ();
    type Error = TilesetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let tileset = Tileset::from_binary(bytes)?;
        Ok(tileset.into_image())
    }

    fn extensions(&self) -> &[&str] {
        &["tiles"]
    }
}

/// The error types for the Tileset asset loader.
#[derive(Debug, thiserror::Error)]
pub enum TilesetLoaderError {
    /// An IO error occurred.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// The file is not a valid Tileset file.
    #[error("{0}")]
    InvalidFile(#[from] TilesetError),
}
