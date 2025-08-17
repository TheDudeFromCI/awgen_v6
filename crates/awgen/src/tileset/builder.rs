//! This module implements the tileset builder functionality for Awgen.

use std::path::PathBuf;

use bevy::prelude::*;
use image::ImageReader;

use crate::tileset::tileset::{Tileset, TilesetError};

/// Creates a new tileset file from a list of provided tile image paths.
///
/// If there is already a tileset at the given output path, it will be
/// overwritten.
pub fn create_tileset(
    tile_paths: Vec<PathBuf>,
    output_path: PathBuf,
) -> Result<Image, TilesetBuilderError> {
    let mut tileset = Tileset::new();

    for tile in tile_paths {
        let img = ImageReader::open(&tile)?.decode()?;
        tileset
            .append_tile(img)
            .map_err(|e| TilesetBuilderError::TileError(tile.clone(), e))?;
    }

    std::fs::write(output_path, tileset.as_binary())?;
    Ok(tileset.into_image())
}

/// Errors that can be thrown while creating a tileset.
#[derive(Debug, thiserror::Error)]
pub enum TilesetBuilderError {
    /// An error occurred while loading an image file.
    #[error("Failed to load image file: {0}")]
    Io(#[from] std::io::Error),

    /// An error occurred while parsing an image file.
    #[error("Failed to parse image file: {0}")]
    ParseError(#[from] image::ImageError),

    /// An error that occurs when adding a tile to a tileset.
    #[error("Failed to add tile: {0}")]
    TileError(PathBuf, TilesetError),
}
