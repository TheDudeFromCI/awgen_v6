//! This module implements the tileset builder functionality for Awgen.

use std::path::PathBuf;

use bevy::prelude::*;
use image::ImageReader;

use crate::tileset::asset_loader::MAGIC_NUMBER;

/// Creates a new tileset file from a list of provided tile image paths.
pub fn create_tileset(
    tile_paths: Vec<PathBuf>,
    output_path: PathBuf,
) -> Result<(), TilesetBuilderError> {
    let mut binary: Vec<u8> = vec![];
    let mut expected_size = 0;
    let tile_count = tile_paths.len();

    for tile in tile_paths {
        let img = ImageReader::open(&tile)?.decode()?;

        let width = img.width();
        let height = img.height();

        if width != height {
            return Err(TilesetBuilderError::TileNotSquare(
                tile.clone(),
                width,
                height,
            ));
        }

        if !is_power_of_two(width) {
            return Err(TilesetBuilderError::TileNotPowerOfTwo(tile.clone(), width));
        }

        if expected_size > 0 && width != expected_size {
            return Err(TilesetBuilderError::TileSizeMismatch(
                tile.clone(),
                expected_size,
                width,
            ));
        }

        expected_size = width;
        let mipmaps = mipmap_count(width);

        if binary.is_empty() {
            let expected_byte_size = expected_byte_size(width, mipmaps);
            binary.reserve(expected_byte_size * tile_count + MAGIC_NUMBER.len() + 8);
            binary.extend_from_slice(MAGIC_NUMBER);
            binary.extend_from_slice(expected_size.to_le_bytes().as_ref());
            binary.extend_from_slice((tile_count as u32).to_le_bytes().as_ref());
        }

        let pixels = img.to_rgba8().to_vec();
        binary.extend_from_slice(&pixels);

        generate_mipmaps(pixels, width, mipmaps, &mut binary);
    }

    std::fs::write(output_path, &binary)?;
    Ok(())
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

    /// An error that occurs when attempting to add a tile that is not square.
    #[error("The tile \"{0}\" is not square: {1}x{2}")]
    TileNotSquare(PathBuf, u32, u32),

    /// An error that occurs when attempting to add a tile that is not a power
    /// of two.
    #[error("The tile \"{0}\" size is not a power of two: {1}")]
    TileNotPowerOfTwo(PathBuf, u32),

    /// An error that occurs when the size of the tile does not match the
    /// expected size.
    #[error("Tile \"{0}\" size does not match the tileset: expected {1}x{1}, got {2}x{2}")]
    TileSizeMismatch(PathBuf, u32, u32),
}

/// Checks if the given number is a power of two.
fn is_power_of_two(n: u32) -> bool {
    n > 0 && (n & (n - 1)) == 0
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

/// Generates mipmaps for the given image bytes and append them to the end of
/// the byte vector.
fn generate_mipmaps(bytes: Vec<u8>, mut size: u32, mipmaps: u32, output: &mut Vec<u8>) {
    let mut sample = bytes;

    for _ in 0 .. mipmaps {
        size /= 2;
        let mut new_bytes = Vec::new();

        for y in 0 .. size {
            for x in 0 .. size {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                let mut a = 0;

                for j in 0 .. 2 {
                    for i in 0 .. 2 {
                        let index = ((y * 2 + j) * size * 2 + x * 2 + i) as usize * 4;
                        r += sample[index] as u32;
                        g += sample[index + 1] as u32;
                        b += sample[index + 2] as u32;
                        a += sample[index + 3] as u32;
                    }
                }

                r /= 4;
                g /= 4;
                b /= 4;
                a /= 4;

                new_bytes.push(r as u8);
                new_bytes.push(g as u8);
                new_bytes.push(b as u8);
                new_bytes.push(a as u8);
            }
        }

        output.extend_from_slice(&new_bytes);
        sample = new_bytes;
    }
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
