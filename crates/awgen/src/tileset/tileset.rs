//! This module implements the data structure for a tileset in Awgen.

use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageSampler};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use image::DynamicImage;

/// The magic number that identifies a valid Tileset file.
pub const MAGIC_NUMBER: &[u8; 13] = b"AWGEN TILESET";

/// The data structure representing a tileset in Awgen.
#[derive(Debug)]
pub struct Tileset {
    /// The binary pixel data of the tileset, including all tiles and mipmaps.
    binary: Vec<u8>,

    /// The size of each tile in pixels. All tiles in the tileset must be
    /// square and of the same size.
    size: u32,

    /// The number of tiles in the tileset.
    tile_count: u32,

    /// The number of mipmaps generated for each tile in the tileset.
    mipmaps: u32,
}

impl Tileset {
    /// Creates a new, empty [`Tileset`] instance.
    pub fn new() -> Self {
        Self {
            binary: Vec::new(),
            size: 0,
            tile_count: 0,
            mipmaps: 0,
        }
    }

    /// Creates a new [`Tileset`] from a binary representation.
    pub fn from_binary(binary: Vec<u8>) -> Result<Self, TilesetError> {
        let mut offset = 0;
        read_magic(&binary, &mut offset)?;

        let size = read_uint(&binary, &mut offset)?;
        let tile_count = read_uint(&binary, &mut offset)?;
        let mipmaps = mipmap_count(size);

        let mut tileset = Tileset {
            binary: Vec::new(),
            size,
            tile_count,
            mipmaps,
        };

        let expected_binary_len =
            tileset.expected_tile_bytes() * tile_count as usize + MAGIC_NUMBER.len() + 8;

        if binary.len() != expected_binary_len {
            return Err(TilesetError::InvalidFile(format!(
                "Invalid binary size: expected {} bytes, got {} bytes",
                expected_binary_len,
                binary.len(),
            )));
        }

        tileset.binary = binary[offset ..].to_vec();
        Ok(tileset)
    }

    /// Appends a [`TileImage`] to the tileset.
    ///
    /// The tile must be a square image, and its size must be a power of two,
    /// matching the tileset size.
    ///
    /// If the tileset is empty, the first tile will set the size of the
    /// tileset.
    pub fn append_tile(&mut self, tile: impl TileImage) -> Result<(), TilesetError> {
        let width = tile.width();
        let height = tile.height();

        if width != height {
            return Err(TilesetError::TileNotSquare(width, height));
        }

        if !is_power_of_two(width) {
            return Err(TilesetError::TileNotPowerOfTwo(width));
        }

        if self.size == 0 {
            self.size = width;
            self.mipmaps = mipmap_count(width);
        }

        if width != self.size {
            return Err(TilesetError::TileSizeMismatch(self.size, width));
        }

        let pixels = tile.binary();

        let expected_bytes = (width * height * 4) as usize;
        if pixels.len() != expected_bytes {
            return Err(TilesetError::CorruptedTileData(
                expected_bytes,
                pixels.len(),
            ));
        }

        self.generate_mipmaps(pixels);
        self.tile_count += 1;

        Ok(())
    }

    /// Generates mipmaps for the given image bytes and append them to the end
    /// of the byte vector.
    fn generate_mipmaps(&mut self, mut pixels: Vec<u8>) {
        self.binary.reserve(self.expected_tile_bytes());
        self.binary.extend_from_slice(&pixels);

        let mut size = self.size;
        for _ in 0 .. self.mipmaps {
            size /= 2;
            let mut new_pixels = Vec::new();

            for y in 0 .. size {
                for x in 0 .. size {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    let mut a = 0;

                    for j in 0 .. 2 {
                        for i in 0 .. 2 {
                            let index = ((y * 2 + j) * size * 2 + x * 2 + i) as usize * 4;
                            r += pixels[index] as u32;
                            g += pixels[index + 1] as u32;
                            b += pixels[index + 2] as u32;
                            a += pixels[index + 3] as u32;
                        }
                    }

                    r /= 4;
                    g /= 4;
                    b /= 4;
                    a /= 4;

                    new_pixels.push(r as u8);
                    new_pixels.push(g as u8);
                    new_pixels.push(b as u8);
                    new_pixels.push(a as u8);
                }
            }

            self.binary.extend_from_slice(&new_pixels);
            pixels = new_pixels;
        }
    }

    /// Calculates the expected byte size of a single tile, including all
    /// mipmaps.
    fn expected_tile_bytes(&self) -> usize {
        let mut bytes = 0;

        let mut s = self.size;
        for _ in 0 ..= self.mipmaps {
            bytes += s * s * 4;
            s /= 2;
        }

        bytes as usize
    }

    /// Converts this [`Tileset`] into a bevy [`Image`].
    pub fn into_image(mut self) -> Image {
        if self.tile_count == 0 {
            self.size = 4;
            self.mipmaps = 0;
            self.tile_count = 2;
            self.binary = vec![255; self.expected_tile_bytes() * 2];
        }

        let mut tileset = Image {
            data: Some(self.binary),
            ..default()
        };

        tileset.asset_usage = RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD;
        tileset.texture_descriptor.mip_level_count = self.mipmaps + 1;
        tileset.texture_descriptor.dimension = TextureDimension::D2;
        tileset.texture_descriptor.format = TextureFormat::Rgba8UnormSrgb;
        tileset.texture_descriptor.size = Extent3d {
            width: self.size,
            height: self.size,
            depth_or_array_layers: self.tile_count.max(2),
        };

        tileset.sampler = ImageSampler::nearest();
        let sampler_descriptor = tileset.sampler.get_or_init_descriptor();
        sampler_descriptor.address_mode_u = ImageAddressMode::Repeat;
        sampler_descriptor.address_mode_v = ImageAddressMode::Repeat;
        sampler_descriptor.lod_max_clamp = self.mipmaps as f32 + 1.0;

        tileset
    }

    /// Serializes this [`Tileset`] into a binary representation that can be
    /// saved to a file.
    pub fn as_binary(&self) -> Vec<u8> {
        let expected_binary_len =
            self.expected_tile_bytes() * self.tile_count as usize + MAGIC_NUMBER.len() + 8;

        let mut binary = Vec::with_capacity(expected_binary_len);
        binary.extend_from_slice(MAGIC_NUMBER);
        binary.extend_from_slice(self.size.to_le_bytes().as_ref());
        binary.extend_from_slice(self.tile_count.to_le_bytes().as_ref());
        binary.extend_from_slice(&self.binary);
        binary
    }
}

/// Errors that can be thrown while editing a tileset.
#[derive(Debug, thiserror::Error)]
pub enum TilesetError {
    /// An error that occurs when attempting to add a tile that is not square.
    #[error("The tile size, {0}x{1}, is not square")]
    TileNotSquare(u32, u32),

    /// An error that occurs when attempting to add a tile that is not a power
    /// of two.
    #[error("The tile size {0} is not a power of two")]
    TileNotPowerOfTwo(u32),

    /// An error that occurs when the size of the tile does not match the
    /// expected size.
    #[error("Tile size does not match the tileset. Expected {0}x{0}, got {1}x{1}")]
    TileSizeMismatch(u32, u32),

    /// The binary for the tile data does not match the expected size.
    #[error("The tile binary is an invalid size. Expected {0} bytes, got {1} bytes")]
    CorruptedTileData(usize, usize),

    /// The file is not a valid Tileset file.
    #[error("Invalid Tileset file: {0}")]
    InvalidFile(String),
}

/// A trait that defines an image binary that can be added to a tileset.
pub trait TileImage {
    /// The RGBA8 pixel data of the image.
    fn binary(&self) -> Vec<u8>;

    /// The width of the image in pixels.
    fn width(&self) -> u32;

    /// The height of the image in pixels.
    fn height(&self) -> u32;
}

impl TileImage for DynamicImage {
    fn binary(&self) -> Vec<u8> {
        self.to_rgba8().to_vec()
    }

    fn width(&self) -> u32 {
        self.width()
    }

    fn height(&self) -> u32 {
        self.height()
    }
}

/// Checks if the given number is a power of two.
pub fn is_power_of_two(n: u32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

/// Calculates the number of mipmaps for the given image size.
pub fn mipmap_count(size: u32) -> u32 {
    let mut count = 0;
    let mut s = size;
    while s > 4 {
        count += 1;
        s /= 2;
    }
    count
}

/// Read the magic number from the given byte slice at the given offset and
/// increments the offset by the length of the magic number.
fn read_magic(bytes: &[u8], offset: &mut usize) -> Result<(), TilesetError> {
    if bytes.len() < *offset + MAGIC_NUMBER.len() {
        return Err(TilesetError::InvalidFile("End of stream".into()));
    }

    if &bytes[*offset .. *offset + MAGIC_NUMBER.len()] != MAGIC_NUMBER {
        return Err(TilesetError::InvalidFile("Invalid magic number".into()));
    }

    *offset += MAGIC_NUMBER.len();
    Ok(())
}

/// Read a 32-bit unsigned integer from the given byte slice at the given offset
/// and increments the offset by 4.
fn read_uint(bytes: &[u8], offset: &mut usize) -> Result<u32, TilesetError> {
    if bytes.len() < *offset + 4 {
        return Err(TilesetError::InvalidFile("End of stream".into()));
    }

    let int = u32::from_le_bytes(bytes[*offset .. *offset + 4].try_into().unwrap());
    *offset += 4;
    Ok(int)
}
