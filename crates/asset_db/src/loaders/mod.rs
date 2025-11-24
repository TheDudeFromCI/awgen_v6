//! This module implements the asset loaders for Awgen asset databases.

use std::io::Write;

use bevy::prelude::*;
use bevy::tasks::Task;

mod image;
mod preview;

pub use image::*;
pub use preview::*;

/// An asset that is supported by the Awgen asset management system.
pub trait AwgenAsset: Asset + Sized {
    /// Returns the asset type name associated with this asset.
    ///
    /// This value will be the extension used within the Awgen asset database
    /// to identify assets of this type. This value should be unique per asset.
    fn type_name() -> &'static str;

    /// Converts this asset into a byte vector for storage in the Awgen asset
    /// database.
    fn save(&self) -> Result<Vec<u8>, AssetDataError>;

    /// Spawns a task that generates a preview image of this asset for asset
    /// thumbnails.
    ///
    /// A preview image should be a 128x128 RGBA image, with bilinear sampling.
    fn generate_preview(&self) -> Task<Result<ImagePreviewData, AssetDataError>>;
}

/// Error type for Awgen asset processing.
#[derive(Debug, thiserror::Error)]
#[error("Failed to process Awgen asset: {0}")]
pub struct AssetDataError(pub String);

impl From<std::io::Error> for AssetDataError {
    fn from(e: std::io::Error) -> Self {
        AssetDataError(format!("I/O error: {}", e))
    }
}

/// A simple in-memory writer for byte vectors.
#[derive(Debug, Default, Clone)]
pub struct ByteWriter {
    /// The byte data being written.
    pub data: Vec<u8>,
}

impl ByteWriter {
    /// Creates a new empty [`ByteWriter`].
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Writes a 32-bit little-endian integer to the byte stream.
    pub fn write_num(&mut self, value: i32) -> Result<(), AssetDataError> {
        self.write_all(&value.to_le_bytes())?;
        Ok(())
    }
}

impl Write for ByteWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
