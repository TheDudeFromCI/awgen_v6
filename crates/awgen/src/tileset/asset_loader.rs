//! The asset loader for the Awgen tileset file format.

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext, RenderAssetUsages};
use bevy::image::{ImageAddressMode, ImageSampler};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// The magic number that identifies a valid Tileset file.
pub const MAGIC_NUMBER: &[u8; 13] = b"AWGEN TILESET";

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

        let mut offset = 0;
        read_magic(&bytes, &mut offset)?;

        let size = read_uint(&bytes, &mut offset)?;
        let tile_count = read_uint(&bytes, &mut offset)?;
        let mipmaps = mipmap_count(size);
        let expected_size = expected_byte_size(size, mipmaps) * tile_count as usize;

        if bytes.len() != offset + expected_size {
            return Err(TilesetLoaderError::InvalidFile(
                "Corrupted pixel data".into(),
            ));
        }

        let mut tileset = Image {
            data: Some(bytes[offset ..].to_vec()),
            ..default()
        };

        tileset.asset_usage = RenderAssetUsages::RENDER_WORLD;
        tileset.texture_descriptor.mip_level_count = mipmaps + 1;
        tileset.texture_descriptor.dimension = TextureDimension::D2;
        tileset.texture_descriptor.format = TextureFormat::Rgba8UnormSrgb;
        tileset.texture_descriptor.size = Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: tile_count.max(2) as u32,
        };

        tileset.sampler = ImageSampler::nearest();
        let sampler_descriptor = tileset.sampler.get_or_init_descriptor();
        sampler_descriptor.address_mode_u = ImageAddressMode::Repeat;
        sampler_descriptor.address_mode_v = ImageAddressMode::Repeat;
        sampler_descriptor.lod_max_clamp = mipmaps as f32 + 1.0;

        Ok(tileset)
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
    #[error("Invalid Tileset file: {0}")]
    InvalidFile(String),
}

/// Read the magic number from the given byte slice at the given offset and
/// increments the offset by the length of the magic number.
fn read_magic(bytes: &[u8], offset: &mut usize) -> Result<(), TilesetLoaderError> {
    if bytes.len() < *offset + MAGIC_NUMBER.len() {
        return Err(TilesetLoaderError::InvalidFile("End of stream".into()));
    }

    if &bytes[*offset .. *offset + MAGIC_NUMBER.len()] != MAGIC_NUMBER {
        return Err(TilesetLoaderError::InvalidFile(
            "Invalid magic number".into(),
        ));
    }

    *offset += MAGIC_NUMBER.len();
    Ok(())
}

/// Read a 32-bit unsigned integer from the given byte slice at the given offset
/// and increments the offset by 4.
fn read_uint(bytes: &[u8], offset: &mut usize) -> Result<u32, TilesetLoaderError> {
    if bytes.len() < *offset + 4 {
        return Err(TilesetLoaderError::InvalidFile("End of stream".into()));
    }

    let int = u32::from_le_bytes(bytes[*offset .. *offset + 4].try_into().unwrap());
    *offset += 4;
    Ok(int)
}

/// Calculates the number of mipmaps for the given image size.
fn mipmap_count(size: u32) -> u32 {
    let mut count = 0;
    let mut s = size;
    while s > 4 {
        count += 1;
        s /= 2;
    }
    count
}

/// Calculates the expected byte size of the image data for the given size and
/// number of mipmaps, per layer.
fn expected_byte_size(size: u32, mipmaps: u32) -> usize {
    let mut bytes = 0;

    let mut s = size;
    for _ in 0 ..= mipmaps {
        bytes += s * s * 4;
        s /= 2;
    }

    bytes as usize
}
